# -*- coding: utf-8 -*-
"""分类验证每个练习（错误版）：
- 编译失败   -> 编译错误型 bug（符合预期，前提是答案版能编译 —— 由 verify_solutions 保证）
- 编译成功   -> 逻辑错误型 bug，必须存在失败的测试（否则 bug 未被测试捕获 = 问题）
用法: python _tools/verify_exercises.py
"""
import io, json, os, subprocess, sys, time

ROOT = r"C:\Users\jcsyh\Documents\AI夜间巡航\bevylings"
manifest = json.load(io.open(os.path.join(ROOT, "exercises.manifest.json"), encoding="utf-8"))
ids = [ex[3:] for ch in manifest["chapters"] for ex in ch["exercises"]]

compile_err, logic_err, problems = [], [], []
t0 = time.time()
for i, eid in enumerate(ids):
    print(f"[{i+1}/{len(ids)}] {eid} ...", flush=True)
    try:
        r = subprocess.run(
            ["cargo", "check", "-p", "exercises", "--features", f"ex_{eid}"],
            cwd=ROOT, capture_output=True, text=True, timeout=600,
        )
        compiles = r.returncode == 0
    except subprocess.TimeoutExpired:
        problems.append((eid, "check TIMEOUT"))
        continue
    if not compiles:
        compile_err.append(eid)
        print(f"    编译错误型 ({time.time()-t0:.0f}s)")
        continue
    try:
        t = subprocess.run(
            ["cargo", "test", "-p", "exercises", "--features", f"ex_{eid}"],
            cwd=ROOT, capture_output=True, timeout=600,
        )
        out = (t.stdout or b"").decode("utf-8", "replace") + (t.stderr or b"").decode("utf-8", "replace")
        has_fail = "test result: FAILED" in out or t.returncode != 0
    except subprocess.TimeoutExpired:
        problems.append((eid, "test TIMEOUT"))
        continue
    if has_fail:
        logic_err.append(eid)
        print(f"    逻辑错误型（测试失败 ✅）({time.time()-t0:.0f}s)")
    else:
        problems.append((eid, "编译通过但所有测试都过了 —— bug 未被测试捕获！"))

print("\n===== 汇总 =====")
print(f"编译错误型: {len(compile_err)}")
print(f"逻辑错误型: {len(logic_err)}")
print(f"问题: {len(problems)}")
for eid, why in problems:
    print(f"  ! {eid}: {why}")
if problems:
    sys.exit(1)
print("所有练习都符合预期（编译错误型或逻辑错误型且被测试捕获）✅")
