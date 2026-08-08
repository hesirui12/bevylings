# -*- coding: utf-8 -*-
"""bevylings 内容审计脚本：
1. 检查每个练习是否有 exercise + solution 两个文件
2. 检查 exercise 文件格式（I AM NOT DONE / BUG / 测试 / 提示 / 出处）
3. 检查 solution 无 BUG / I AM NOT DONE 标记
4. 汇总统计
用法: python _tools/audit_content.py
"""
import io, json, os, re, sys

ROOT = r"C:\Users\jcsyh\Documents\AI夜间巡航\bevylings"
manifest = json.load(io.open(os.path.join(ROOT, "exercises.manifest.json"), encoding="utf-8"))

problems = []
counts = {"exercises": 0, "solutions": 0, "with_tests": 0, "with_hints": 0, "with_source": 0}

for ch in manifest["chapters"]:
    slug = ch["slug"]
    for i, ex in enumerate(ch["exercises"]):
        n = f"{i + 1:02d}"
        ex_path = os.path.join(ROOT, f"exercises/src/chapters/{slug}/exercise_{n}.rs")
        sol_path = os.path.join(ROOT, f"solutions/src/chapters/{slug}/exercise_{n}.rs")
        eid = ex[3:]
        if not os.path.exists(ex_path):
            problems.append(f"[{eid}] 缺少练习文件 {ex_path}")
            continue
        if not os.path.exists(sol_path):
            problems.append(f"[{eid}] 缺少答案文件 {sol_path}")
            continue
        counts["exercises"] += 1
        counts["solutions"] += 1
        src = io.open(ex_path, encoding="utf-8").read()
        sol = io.open(sol_path, encoding="utf-8").read()
        if "// I AM NOT DONE" not in src:
            problems.append(f"[{eid}] 练习缺少 // I AM NOT DONE")
        bug_lines = re.findall(r"^\s*// BUG:", src, re.M)
        if not bug_lines:
            problems.append(f"[{eid}] 练习缺少 // BUG: 标记")
        if len(bug_lines) > 1:
            problems.append(f"[{eid}] 练习有多个 // BUG: 标记（{len(bug_lines)} 处）")
        if "#[cfg(test)]" not in src:
            problems.append(f"[{eid}] 练习缺少 #[cfg(test)] 测试")
        else:
            counts["with_tests"] += 1
        if "// 提示" in src:
            counts["with_hints"] += 1
        if "出处：" in src:
            counts["with_source"] += 1
        if re.findall(r"^\s*// BUG:", sol, re.M):
            problems.append(f"[{eid}] 答案文件残留 // BUG:")
        if "// I AM NOT DONE" in sol:
            problems.append(f"[{eid}] 答案文件残留 // I AM NOT DONE")
        # run() 入口
        if "pub fn run()" not in src:
            problems.append(f"[{eid}] 练习缺少 pub fn run() 入口")
        if "pub fn run()" not in sol:
            problems.append(f"[{eid}] 答案缺少 pub fn run() 入口")
        # 禁止第三方 crate 特征：外部 use
        bad = re.findall(r"^\s*use (?!bevy|super|std|core|crate)[a-zA-Z0-9_:]+", src, re.M)
        if bad:
            problems.append(f"[{eid}] 可能引用了非 bevy crate: {bad}")
        # 行数
        if len(src.splitlines()) > 165:
            problems.append(f"[{eid}] 练习文件过长（{len(src.splitlines())} 行）")

print(f"练习文件: {counts['exercises']} / 151")
print(f"答案文件: {counts['solutions']} / 151")
print(f"含测试: {counts['with_tests']}  含提示: {counts['with_hints']}  含出处: {counts['with_source']}")
print(f"问题数: {len(problems)}")
for p in problems[:80]:
    print("  !", p)
sys.exit(1 if problems else 0)
