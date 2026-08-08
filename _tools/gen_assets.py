# -*- coding: utf-8 -*-
"""为练习中引用的资产路径生成占位资源（assets/ 工作区根目录）。

扫描 exercises/src/chapters 里 asset_server.load("...") 引用的路径，
按扩展名生成最小合法占位文件：
  .ogg/.wav/.mp3 -> ffmpeg 正弦波音
  .png           -> 纯色 PNG（纯 python 生成）
  .glb           -> 最小合法 glTF 二进制（空场景）
  .gltf          -> 最小 JSON glTF
  .wgsl          -> 最小 WGSL（顶点+片元透传）
  .txt/.json/.ron -> 最小文本内容
  .ttf           -> 跳过（Bevy 内置默认字体兜底）
"""
import io, json, os, re, struct, subprocess, sys, zlib

ROOT = r"C:\Users\jcsyh\Documents\AI夜间巡航\bevylings"
SRC = os.path.join(ROOT, "exercises", "src", "chapters")
OUT = os.path.join(ROOT, "assets")

def find_asset_paths():
    paths = set()
    pat = re.compile(r'(?:load|load_async|load_untyped)\s*\(\s*"([^"]+)"')
    for dirpath, _, files in os.walk(SRC):
        for f in files:
            if f.endswith(".rs"):
                try:
                    src = io.open(os.path.join(dirpath, f), encoding="utf-8").read()
                except Exception:
                    continue
                for line_no, line in enumerate(src.splitlines()):
                    stripped = line.strip()
                    if stripped.startswith("//"):
                        continue
                    for m in pat.finditer(line):
                        p = m.group(1)
                        if not p.startswith("http"):
                            paths.add(p)
    return sorted(paths)

def solid_png(path, rgb=(120, 144, 255)):
    w = h = 4
    def chunk(tag, data):
        c = struct.pack(">I", len(data)) + tag + data
        return c + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)
    ihdr = struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0)
    raw = b"".join(b"\x00" + bytes(rgb) * w for _ in range(h))
    idat = zlib.compress(raw)
    with io.open(path, "wb") as f:
        f.write(b"\x89PNG\r\n\x1a\n")
        f.write(chunk(b"IHDR", ihdr))
        f.write(chunk(b"IDAT", idat))
        f.write(chunk(b"IEND", b""))

def solid_glb(path):
    gltf = {"asset": {"version": "2.0"}, "scenes": [{"name": "scene", "nodes": []}], "scene": 0, "nodes": []}
    j = json.dumps(gltf).encode("utf-8")
    j = j + b" " * ((4 - len(j) % 4) % 4)
    bin_data = b""
    header = struct.pack("<III", 0x46546C67, 12 + 8 + len(j) + 8 + len(bin_data), 2)
    chunks = struct.pack("<I", len(j)) + b"JSON" + j
    chunks += struct.pack("<I", len(bin_data)) + b"BIN\x00" + bin_data
    with io.open(path, "wb") as f:
        f.write(header + chunks)

def ffmpeg_tone(path, seconds=1.0, freq=440.0):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    ext = os.path.splitext(path)[1][1:]
    args = ["ffmpeg", "-y", "-f", "lavfi", "-i",
            f"sine=frequency={freq}:duration={seconds}", "-ac", "2", path]
    r = subprocess.run(args, capture_output=True)
    return r.returncode == 0 and os.path.exists(path)

MIN_WGSL = """// 最小占位着色器（由 bevylings 生成）
@vertex
fn vs_main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 1.0);
}
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.5, 0.56, 1.0, 1.0);
}
"""

def main():
    paths = find_asset_paths()
    print(f"发现 {len(paths)} 个资产引用")
    missing, made, skipped = [], [], []
    for p in paths:
        dst = os.path.join(OUT, p.replace("/", os.sep))
        if os.path.exists(dst):
            skipped.append(p)
            continue
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        ext = os.path.splitext(p)[1].lower()
        if ext in (".ogg", ".wav", ".mp3"):
            if ffmpeg_tone(dst):
                made.append(p)
            else:
                missing.append(p + " (ffmpeg 失败)")
        elif ext == ".png":
            solid_png(dst)
            made.append(p)
        elif ext == ".glb":
            solid_glb(dst)
            made.append(p)
        elif ext == ".gltf":
            io.open(dst, "w", encoding="utf-8").write(
                json.dumps({"asset": {"version": "2.0"}, "scenes": [{"nodes": []}], "scene": 0}))
            made.append(p)
        elif ext == ".wgsl":
            io.open(dst, "w", encoding="utf-8", newline="\n").write(MIN_WGSL)
            made.append(p)
        elif ext == ".ttf":
            skipped.append(p + " (ttf 跳过)")
        else:
            io.open(dst, "w", encoding="utf-8", newline="\n").write("placeholder\n")
            made.append(p)
    print("已生成:", len(made))
    for m in made:
        print("  +", m)
    if skipped:
        print("已存在/跳过:")
        for s in skipped:
            print("  ~", s)
    if missing:
        print("失败:")
        for m in missing:
            print("  !", m)
        sys.exit(1)

if __name__ == "__main__":
    main()
