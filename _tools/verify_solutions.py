# -*- coding: utf-8 -*-
"""批量验证参考答案：每个练习跑 cargo test -p solutions --features sol_XXXX。
要求：全部编译通过且测试全绿。
增量记录：_sources/sol_passes.txt / _sources/sol_fails.txt，支持 --resume 断点续跑。
用法: python _tools/verify_solutions.py [--resume]
"""
import io, json, os, subprocess, sys, time

ROOT = r"C:\Users\jcsyh\Documents\AI夜间巡航\bevylings"
manifest = json.load(io.open(os.path.join(ROOT, "exercises.manifest.json"), encoding="utf-8"))
ids = [ex[3:] for ch in manifest["chapters"] for ex in ch["exercises"]]
PASS_FILE = os.path.join(ROOT, "_sources", "sol_passes.txt")
FAIL_FILE = os.path.join(ROOT, "_sources", "sol_fails.txt")
os.makedirs(os.path.join(ROOT, "_sources"), exist_ok=True)

passed = set()
if "--resume" in sys.argv and os.path.exists(PASS_FILE):
    passed = set(io.open(PASS_FILE, encoding="utf-8").read().split())

failed = []
t0 = time.time()
for i, eid in enumerate(ids):
    if eid in passed:
        continue
    print(f"[{i+1}/{len(ids)}] {eid} ...", flush=True)
    try:
        r = subprocess.run(
            ["cargo", "test", "-p", "solutions", "--features", f"sol_{eid}"],
            cwd=ROOT, capture_output=True, timeout=900,
        )
        out = (r.stdout or b"").decode("utf-8", "replace") + (r.stderr or b"").decode("utf-8", "replace")
        ok = r.returncode == 0 and "test result: ok." in out
    except subprocess.TimeoutExpired:
        ok = False
        out = "TIMEOUT"
    if ok:
        passed.add(eid)
        with io.open(PASS_FILE, "a", encoding="utf-8") as f:
            f.write(eid + "\n")
        print(f"    PASS ({time.time()-t0:.0f}s)", flush=True)
    else:
        with io.open(FAIL_FILE, "a", encoding="utf-8") as f:
            f.write(f"{eid}\n{out[-2000:]}\n{'-'*40}\n")
        failed.append(eid)
        print(f"    FAIL ({time.time()-t0:.0f}s)", flush=True)

print("\n===== 汇总 =====")
print(f"通过: {len(passed)} / {len(ids)}")
if failed:
    print(f"失败: {len(failed)}")
    for eid in failed:
        print(f"### {eid}")
    sys.exit(1)
print("全部参考答案编译并测试通过 ✅")
