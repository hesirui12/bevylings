# -*- coding: utf-8 -*-
"""从练习文件派生参考答案：
1. 删除 `// I AM NOT DONE`
2. 删除 BUG 注释行（// BUG: ...）
3. 应用修复映射（每章提供 BUG 行的 old -> new 替换）

用法: python _tools/gen_solution.py <chapter_slug> <exercise_num> <old_bug_line> <new_fixed_line>
      old/new 需用 \n 转义多行。
"""
import io, sys

def main():
    root = r"C:\Users\jcsyh\Documents\AI夜间巡航\bevylings"
    slug, num, old, new = sys.argv[1], sys.argv[2], sys.argv[3].replace(r"\n", "\n"), sys.argv[4].replace(r"\n", "\n")
    ex_path = f"{root}\\exercises\\src\\chapters\\{slug}\\exercise_{num}.rs"
    sol_path = f"{root}\\solutions\\src\\chapters\\{slug}\\exercise_{num}.rs"
    with io.open(ex_path, encoding="utf-8") as f:
        src = f.read()
    src = src.replace("// I AM NOT DONE\n\n", "")
    assert old in src, "BUG old text not found in exercise file"
    src = src.replace(old, new, 1)
    with io.open(sol_path, "w", encoding="utf-8", newline="\n") as f:
        f.write(src)
    print("solution written:", sol_path)

if __name__ == "__main__":
    main()
