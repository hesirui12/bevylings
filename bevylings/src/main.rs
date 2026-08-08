//! bevylings 练习运行器（类似 rustlings 的 Bevy 版本）
//!
//! 用法：
//!   bevylings list                列出所有章节与练习
//!   bevylings next                显示下一个未完成的练习
//!   bevylings run <id>            编译并运行某个练习（错误版，看看 bug 的效果）
//!   bevylings test <id>           运行某个练习的单元测试（通过 = 完成）
//!   bevylings verify <id>         运行参考答案的测试，展示"正确行为"
//!   bevylings hint <id>           打印练习文件底部的提示
//!   bevylings reset               重置进度
//!   bevylings status              查看进度

use serde::Deserialize;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MANIFEST: &str = "exercises.manifest.json";
const STATE_DIR: &str = ".bevylings";
const STATE_FILE: &str = ".bevylings/state.json";

#[derive(Deserialize)]
struct Manifest {
    chapters: Vec<Chapter>,
}

#[derive(Deserialize)]
struct Chapter {
    num: usize,
    slug: String,
    title: String,
    count: usize,
    exercises: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct State {
    done: BTreeSet<String>,
}

fn find_root() -> PathBuf {
    if let Ok(r) = env::var("BEVYLINGS_ROOT") {
        return PathBuf::from(r);
    }
    let mut dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loop {
        if dir.join(MANIFEST).exists() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn load_manifest(root: &Path) -> Manifest {
    let raw = fs::read_to_string(root.join(MANIFEST)).expect("找不到 exercises.manifest.json");
    serde_json::from_str(&raw).expect("manifest 解析失败")
}

fn load_state(root: &Path) -> State {
    fs::read_to_string(root.join(STATE_FILE))
        .map(|s| serde_json::from_str(&s).unwrap_or_default())
        .unwrap_or_default()
}

fn save_state(root: &Path, state: &State) {
    let dir = root.join(STATE_DIR);
    fs::create_dir_all(&dir).ok();
    fs::write(
        dir.join("state.json"),
        serde_json::to_string_pretty(state).unwrap(),
    )
    .ok();
}

fn all_exercises<'m>(manifest: &'m Manifest) -> Vec<(&'m Chapter, &'m String)> {
    manifest
        .chapters
        .iter()
        .flat_map(|c| c.exercises.iter().map(move |e| (c, e)))
        .collect()
}

fn norm(id: &str) -> &str {
    id.strip_prefix("ex_").unwrap_or(id)
}

fn chapter_of<'m>(manifest: &'m Manifest, id: &str) -> Option<&'m Chapter> {
    let nid = norm(id);
    manifest
        .chapters
        .iter()
        .find(|c| c.exercises.iter().any(|e| norm(e) == nid))
}

fn exercise_path(root: &Path, chapter: &Chapter, _id: &str, n: usize, sol: bool) -> PathBuf {
    let crate_name = if sol { "solutions" } else { "exercises" };
    root.join(format!(
        "{crate_name}/src/chapters/{}/exercise_{:02}.rs",
        chapter.slug, n
    ))
}

fn num_of(chapter: &Chapter, id: &str) -> usize {
    let nid = norm(id);
    chapter.exercises.iter().position(|e| norm(e) == nid).unwrap() + 1
}

fn hint_of(root: &Path, chapter: &Chapter, id: &str, n: usize) -> String {
    let path = exercise_path(root, chapter, id, n, false);
    match fs::read_to_string(&path) {
        Ok(src) => {
            let mut hints: Vec<String> = Vec::new();
            let mut in_hints = false;
            for line in src.lines() {
                let t = line.trim_start();
                if t.starts_with("// 提示") {
                    in_hints = true;
                    hints.push(t.trim_start_matches("// ").trim().to_string());
                    continue;
                }
                if in_hints {
                    if t.starts_with("//") {
                        hints.push(t.trim_start_matches("//").trim().to_string());
                    } else if t.is_empty() {
                        continue;
                    } else {
                        break;
                    }
                }
            }
            if hints.is_empty() {
                "（本练习没有提供提示）".to_string()
            } else {
                hints.join("\n")
            }
        }
        Err(_) => format!("找不到练习文件 {}", path.display()),
    }
}

/// 记录当前 target/debug/deps 下已有的可执行文件（用于对比出“本次检验新增”的产物）
fn snapshot_deps_exes(root: &Path) -> BTreeSet<PathBuf> {
    let dir = root.join("target/debug/deps");
    let mut before = BTreeSet::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x == "exe") {
                before.insert(p);
            }
        }
    }
    before
}

/// 删除本次 cargo 调用新增的可执行文件（每个 Bevy debug 二进制 40~60MB）。
/// 保留 rlib/rmeta 编译缓存，下次做题仍是增量编译，速度不受影响。
fn cleanup_new_exes(root: &Path, before: &BTreeSet<PathBuf>) {
    let dir = root.join("target/debug/deps");
    let mut removed = 0usize;
    let mut freed = 0u64;
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x == "exe") && !before.contains(&p) {
                if let Ok(md) = fs::metadata(&p) {
                    freed += md.len();
                }
                if fs::remove_file(&p).is_ok() {
                    removed += 1;
                }
            }
        }
    }
    if removed > 0 {
        println!(
            "\n🧹 已清理本次检验新增的 {removed} 个可执行文件（约 {:.1} MB），编译缓存已保留。",
            freed as f64 / 1024.0 / 1024.0
        );
    }
}

fn run_cargo(root: &Path, pkg: &str, features: &str, mode: &str) -> bool {
    let mut cmd = Command::new("cargo");
    cmd.arg(mode)
        .arg("-p")
        .arg(pkg)
        .arg("--features")
        .arg(features);
    cmd.current_dir(root);
    let mut child = cmd.spawn().expect("无法启动 cargo");
    let status = child.wait().expect("cargo 运行失败");
    status.success()
}

fn run_cargo_test_capture(root: &Path, pkg: &str, features: &str) -> (bool, String) {
    let output = Command::new("cargo")
        .args(["test", "-p", pkg, "--features", features])
        .current_dir(root)
        .output()
        .expect("无法启动 cargo");
    let out = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    let ok = output.status.success() && out.contains("test result: ok.");
    (ok, out)
}

fn cmd_list(root: &Path) {
    let manifest = load_manifest(root);
    let state = load_state(root);
    let mut lines = Vec::new();
    lines.push(format!(
        "bevylings 练习进度：{} / {} 完成\n",
        state.done.len(),
        all_exercises(&manifest).len()
    ));
    for ch in &manifest.chapters {
        let done = ch
            .exercises
            .iter()
            .filter(|e| state.done.contains(norm(e)))
            .count();
        lines.push(format!(
            "[{:02}] {} （{} / {} 完成）",
            ch.num, ch.title, done, ch.count
        ));
        for (i, ex) in ch.exercises.iter().enumerate() {
            let mark = if state.done.contains(norm(ex)) { "✔" } else { " " };
            lines.push(format!(
                "    {} {}  {}.{}  {}",
                mark,
                ex,
                ch.num,
                i + 1,
                title_of(root, ch, ex, i + 1)
            ));
        }
    }
    println!("{}", lines.join("\n"));
}

fn title_of(root: &Path, ch: &Chapter, id: &str, n: usize) -> String {
    let path = exercise_path(root, ch, id, n, false);
    if let Ok(src) = fs::read_to_string(&path) {
        for line in src.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("//! # 练习 ") {
                // 形如 "02.01 —— 标题" 或 "1.1 标题"
                let rest = rest.trim();
                let title = match rest.find("——") {
                    Some(i) => rest[i + "——".len()..].trim(),
                    None => rest
                        .split_once(' ')
                        .map(|(_, t2)| t2.trim())
                        .unwrap_or(rest),
                };
                return title.to_string();
            }
        }
    }
    String::new()
}

fn cmd_status(root: &Path) {
    let manifest = load_manifest(root);
    let state = load_state(root);
    let total = all_exercises(&manifest).len();
    println!("已完成 {}/{}", state.done.len(), total);
    let all = all_exercises(&manifest);
    let next = all.iter().find(|(_, e)| !state.done.contains(norm(e))).map(|(c, e)| (c, e));
    match next {
        Some((ch, id)) => {
            let n = num_of(ch, id);
            println!("下一个：{} ({})", id, title_of(root, ch, id, n));
        }
        None => println!("全部完成！🎉"),
    }
}

fn cmd_next(root: &Path) {
    let manifest = load_manifest(root);
    let state = load_state(root);
    let all = all_exercises(&manifest);
    let next = all.iter().find(|(_, e)| !state.done.contains(norm(e))).map(|(c, e)| (c, e));
    match next {
        Some((ch, id)) => {
            let n = num_of(ch, id);
            println!("{} ({})", id, title_of(root, ch, id, n));
            println!("  运行: bevylings run {}", id);
            println!("  测试: bevylings test {}", id);
            println!();
            let src = fs::read_to_string(exercise_path(root, ch, id, n, false)).unwrap_or_default();
            println!("{}", src);
        }
        None => println!("全部完成！🎉 运行 bevylings list 查看总览。"),
    }
}

fn cmd_run(root: &Path, id: &str) {
    let manifest = load_manifest(root);
    let Some(_ch) = chapter_of(&manifest, id) else {
        eprintln!("未知练习：{id}。用 bevylings list 查看所有练习。");
        return;
    };
    let before = snapshot_deps_exes(root);
    run_cargo(root, "exercises", &format!("ex_{}", norm(id)), "run");
    cleanup_new_exes(root, &before);
}

fn cmd_test(root: &Path, id: &str) {
    let manifest = load_manifest(root);
    let Some(_ch) = chapter_of(&manifest, id) else {
        eprintln!("未知练习：{id}。用 bevylings list 查看所有练习。");
        return;
    };
    let before = snapshot_deps_exes(root);
    let (ok, out) = run_cargo_test_capture(root, "exercises", &format!("ex_{}", norm(id)));
    print!("{out}");
    cleanup_new_exes(root, &before);
    let mut state = load_state(root);
    if ok {
        state.done.insert(norm(id).to_string());
        save_state(root, &state);
        println!("\n✅ {id} 测试通过，已标记完成！");
    } else {
        state.done.remove(norm(id));
        save_state(root, &state);
        println!("\n❌ {id} 尚未通过（测试失败或编译错误）。继续加油！");
    }
}

fn cmd_verify(root: &Path, id: &str) {
    let manifest = load_manifest(root);
    let Some(_ch) = chapter_of(&manifest, id) else {
        eprintln!("未知练习：{id}。用 bevylings list 查看所有练习。");
        return;
    };
    println!("运行参考答案（正确版本）的测试，展示期望行为...\n");
    let before = snapshot_deps_exes(root);
    run_cargo(root, "solutions", &format!("sol_{}", norm(id)), "test");
    cleanup_new_exes(root, &before);
}

fn cmd_hint(root: &Path, id: &str) {
    let manifest = load_manifest(root);
    let Some(ch) = chapter_of(&manifest, id) else {
        eprintln!("未知练习：{id}。用 bevylings list 查看所有练习。");
        return;
    };
    let n = num_of(ch, id);
    println!("{id} 提示：\n");
    println!("{}", hint_of(root, ch, id, n));
}

fn cmd_reset(root: &Path) {
    save_state(root, &State::default());
    println!("进度已重置。");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = find_root();
    if args.len() < 2 {
        cmd_list(&root);
        return;
    }
    match args[1].as_str() {
        "list" => cmd_list(&root),
        "status" => cmd_status(&root),
        "next" => cmd_next(&root),
        "run" => {
            if args.len() > 2 {
                cmd_run(&root, &args[2]);
            } else {
                eprintln!("用法: bevylings run <练习ID>");
            }
        }
        "test" => {
            if args.len() > 2 {
                cmd_test(&root, &args[2]);
            } else {
                eprintln!("用法: bevylings test <练习ID>");
            }
        }
        "verify" => {
            if args.len() > 2 {
                cmd_verify(&root, &args[2]);
            } else {
                eprintln!("用法: bevylings verify <练习ID>");
            }
        }
        "hint" => {
            if args.len() > 2 {
                cmd_hint(&root, &args[2]);
            } else {
                eprintln!("用法: bevylings hint <练习ID>");
            }
        }
        "reset" => cmd_reset(&root),
        other => {
            eprintln!("未知命令：{other}\n支持: list / status / next / run / test / verify / hint / reset");
        }
    }
}
