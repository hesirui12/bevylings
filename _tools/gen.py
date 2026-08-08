# -*- coding: utf-8 -*-
"""根据 exercises.manifest.json 生成:
- exercises/Cargo.toml + solutions/Cargo.toml（feature 定义）
- exercises/src/main.rs + solutions/src/main.rs（分派）
- exercises/src/chapters/mod.rs + 每章 mod.rs（cfg 门控）以及 solutions 同构
"""
import json, os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MANIFEST = os.path.join(ROOT, "exercises.manifest.json")

def ex_name(ex):  # ex_0101 -> 0101
    return ex.split("_")[1]

def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(text)

def cargo_toml(pkg, feature_prefix, desc):
    lines = []
    lines.append("[package]")
    lines.append(f'name = "{pkg}"')
    lines.append('version = "0.1.0"')
    lines.append('edition = "2021"')
    lines.append(f'description = "{desc}"')
    lines.append("")
    lines.append("[dependencies]")
    lines.append('bevy = { version = "0.19", optional = true }')
    lines.append("")
    lines.append("[features]")
    lines.append("default = []")
    for ch in manifest["chapters"]:
        chf = f"ch_{ch['num']:02d}"
        lines.append(f'{chf} = []')
    for ch in manifest["chapters"]:
        chf = f"ch_{ch['num']:02d}"
        for ex in ch["exercises"]:
            fid = ex_name(ex)
            feat = f"{feature_prefix}{fid}"
            lines.append(f'{feat} = ["{chf}", "dep:bevy"]')
    lines.append("")
    return "\n".join(lines)

def main_rs(feature_prefix, pkg_name):
    lines = ["//! bevylings 练习入口（由 _tools/gen.py 自动生成，请勿手改）", "mod chapters;", "", "fn main() {"]
    gates = []
    for ch in manifest["chapters"]:
        slug = ch["slug"]
        for i, ex in enumerate(ch["exercises"]):
            fid = ex_name(ex)
            n = f"{i + 1:02d}"
            feat = f"{feature_prefix}{fid}"
            gates.append(f'    #[cfg(feature = "{feat}")]')
            gates.append(f"    chapters::{slug}::exercise_{n}::run();")
    lines.extend(gates)
    all_feats = ", ".join(f'feature = "{feature_prefix}{ex_name(ex)}"' for ch in manifest["chapters"] for ex in ch["exercises"])
    lines.append(f'    #[cfg(not(any({all_feats})))]')
    lines.append('    {')
    lines.append('        eprintln!("请使用 bevylings 运行器，或手动指定练习 feature，例如:");')
    lines.append(f'        eprintln!("  cargo run -p {pkg_name} --features {feature_prefix}0101");')
    lines.append('    }')
    lines.append("}")
    return "\n".join(lines)

def chapters_mod_rs():
    lines = ["//! bevylings 章节（由 _tools/gen.py 自动生成）"]
    for ch in manifest["chapters"]:
        lines.append(f'#[cfg(feature = "ch_{ch["num"]:02d}")]')
        lines.append(f"pub mod {ch['slug']};")
    return "\n".join(lines)

def chapter_mod_rs(ch):
    lines = [f"//! 第 {ch['num']} 章：{ch['title']}（由 _tools/gen.py 自动生成）"]
    for i, ex in enumerate(ch["exercises"]):
        fid = ex_name(ex)
        n = f"{i + 1:02d}"
        lines.append(f'#[cfg(feature = "ex_{fid}")]')
        lines.append(f"pub mod exercise_{n};")
    return "\n".join(lines)

def solution_chapter_mod_rs(ch):
    lines = [f"//! 第 {ch['num']} 章参考答案：{ch['title']}（由 _tools/gen.py 自动生成）"]
    for i, ex in enumerate(ch["exercises"]):
        fid = ex_name(ex)
        n = f"{i + 1:02d}"
        lines.append(f'#[cfg(feature = "sol_{fid}")]')
        lines.append(f"pub mod exercise_{n};")
    return "\n".join(lines)

if __name__ == "__main__":
    manifest = json.load(open(MANIFEST, encoding="utf-8"))

    write(os.path.join(ROOT, "exercises/Cargo.toml"),
          cargo_toml("exercises", "ex_", "bevylings 练习题库（故意改错的代码）"))
    write(os.path.join(ROOT, "solutions/Cargo.toml"),
          cargo_toml("solutions", "sol_", "bevylings 参考答案（正确代码）"))

    write(os.path.join(ROOT, "exercises/src/main.rs"), main_rs("ex_", "exercises"))
    write(os.path.join(ROOT, "solutions/src/main.rs"), main_rs("sol_", "solutions"))

    write(os.path.join(ROOT, "exercises/src/chapters/mod.rs"), chapters_mod_rs())
    write(os.path.join(ROOT, "solutions/src/chapters/mod.rs"), chapters_mod_rs().replace("pub mod", "pub mod"))

    for ch in manifest["chapters"]:
        slug = ch["slug"]
        write(os.path.join(ROOT, f"exercises/src/chapters/{slug}/mod.rs"), chapter_mod_rs(ch))
        write(os.path.join(ROOT, f"solutions/src/chapters/{slug}/mod.rs"), solution_chapter_mod_rs(ch))

    n = sum(len(c["exercises"]) for c in manifest["chapters"])
    print(f"generated scaffold for {len(manifest['chapters'])} chapters / {n} exercises")
