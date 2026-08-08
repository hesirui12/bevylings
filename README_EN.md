<p align="center">
  <a href="README.md"><img src="https://img.shields.io/badge/%E4%B8%AD%E6%96%87-Chinese-gray?style=for-the-badge" alt="中文"></a>
  <a href="README_EN.md"><img src="https://img.shields.io/badge/English-English-blue?style=for-the-badge" alt="English"></a>
</p>

# bevylings 🦀🎮

**A rustlings-style exercise tool built from the official Bevy examples and learning guides.**

Audience: beginners who know basic Rust but have never touched Bevy or game development.

> Every exercise is adapted from the [bevy.org/examples-webgpu](https://bevy.org/examples-webgpu/) (275 official examples)
> and [bevy.org/learn](https://bevy.org/learn/) (official quick-start guides), with **exactly one deliberate bug** injected.
> Your job is to find and fix it. A total of **151 exercises across 35 chapters**, from "Hello, Bevy!" to custom shaders.
>
> **Verified**: all 151 reference solutions compile and pass (343 unit tests, all green);
> every exercise's bug has been validated — 65 compile-error bugs, 86 logic bugs (caught by tests).

![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Bevy](https://img.shields.io/badge/Bevy-0.19-9cf.svg)
![Rust](https://img.shields.io/badge/Rust-Edition%202021-orange.svg)
![Exercises](https://img.shields.io/badge/exercises-151-green.svg)

## Why bevylings?

- **rustlings-style workflow**: `run` → observe the bug → fix the code → `test` passes = done, progress tracked automatically
- **Progressive difficulty**: 35 chapters from a minimal App to shaders, glTF, and advanced ECS — broad coverage of Bevy's mainstream APIs
- **Real code**: all adapted from official Bevy examples and guides — no toy snippets
- **Headless tests**: every exercise test runs headless — no window, no external files, instant feedback
- **Inline guidance**: each exercise file has concept explanations, task instructions, and a `// BUG:` marker

## Quick Start

```bash
# 1. Install Rust (https://rustup.rs)
# 2. Clone this repo
git clone https://github.com/hesirui12/bevylings.git && cd bevylings

# 3. Build the CLI (first build compiles Bevy, takes a few minutes)
cargo build -p bevylings

# 4. List all exercises
./target/debug/bevylings list
```

**Windows users**: you can also double-click `bevylings.bat` (it builds the CLI automatically on first run),
then use `bevylings list`. Or add the directory containing `bevylings.bat` to your PATH to use `bevylings` globally.

## Usage

| Command | Description |
|---|---|
| `bevylings list` | List all chapters and exercises with completion status |
| `bevylings next` | Show the next unfinished exercise (with code) |
| `bevylings run <id>` | Compile and run the exercise (buggy version) — see the bug in action |
| `bevylings test <id>` | Run the exercise's unit tests — **all green = done** |
| `bevylings verify <id>` | Run the reference solution's tests — see the expected behavior |
| `bevylings hint <id>` | Print the hint at the bottom of the exercise file |
| `bevylings status` | View overall progress |
| `bevylings reset` | Reset progress |

> `<id>` can be just the numeric part of the exercise ID, e.g. `0301`, or the full form `ex_0301`.

## Exercise Flow (rustlings convention)

1. `bevylings next` finds the next exercise; open its exercise file.
2. The top of the file has concept explanations and task instructions; the code has one bug marked with `// BUG:`.
3. `bevylings run 0301` to observe the symptom (compile error / runtime misbehavior).
4. Edit `exercises/src/chapters/ch03_ecs/exercise_01.rs`.
5. `bevylings test 0301` — automatically marked complete once tests pass ✅.

```bash
bevylings run 0301     # compile error or wrong behavior — find the BUG
# edit exercises/src/chapters/ch03_ecs/exercise_01.rs
bevylings test 0301    # ✅ passes, marked complete
bevylings next         # on to the next one!
```

Stuck? `bevylings verify 0301` shows what the correct behavior looks like, and `bevylings hint 0301` gives a nudge.
If you're truly stuck, just compare with the answer in `solutions/src/chapters/ch03_ecs/exercise_01.rs`.

## Directory Structure

```
bevylings/
├── bevylings/          # CLI runner (list/run/test/verify/hint/next/status)
├── exercises/          # Exercise bank (deliberately buggy code, Bevy 0.19)
│   └── src/chapters/   # 35 chapters, 3–7 exercises each
├── solutions/          # Reference solutions (correct code)
├── _sources/           # Official source snapshots (bevyengine/bevy) + learn pages
├── _specs/             # Authoring guide for exercise writers (AUTHORING.md)
├── _tools/             # Content generation & verification scripts (gen.py / verify_*.py, etc.)
├── _tasks/             # Task lists
├── exercises.manifest.json  # Chapter/exercise manifest
└── bevylings.bat       # Windows launcher
```

## Chapter Overview (35 chapters / 151 exercises)

| Chapter | Topics |
|---|---|
| 01 Hello, Bevy! | App, plugins, systems, logging, exit |
| 02 App & Applications | headless apps, log levels, plugin groups, thread pool |
| 03 ECS — Entities, Components, Systems | Entity, Component, Query, Bundle, Commands |
| 04 Plugins | Plugin trait, plugin groups, configuration |
| 05 Resources | Res/ResMut, init/insert/remove |
| 06 Breakout Mini-Game | paddle, ball, collision, scoring |
| 07 2D Basics | sprites, shapes, movement, rotation, scaling |
| 08 3D Basics | camera, cube, lights, parent/child, transparency |
| 09 Animation | easing, AnimatedTransform, color animation |
| 10 Audio | AudioPlayer, volume, sound effects, spatial audio |
| 11 Cameras | follow, orbit, zoom, controllers |
| 12 Diagnostics & Logging | FPS, custom diagnostics, log levels |
| 13 Mini-Game Collection | bouncing ball, menu, loading screen, cake eating |
| 14 Debug Drawing | Gizmos: lines/circles/rects/axes |
| 15 Math | vectors, splines, bounds, primitives |
| 16 Scenes | BSN, SceneRoot, world serialization |
| 17 State Management | States, OnEnter/OnExit, sub-states |
| 18 Time & Timers | Time, Timer, Stopwatch, virtual time |
| 19 Transform | Transform, rotation, scaling, facing |
| 20 Input | keyboard, mouse, touch, gamepad |
| 21 UI (User Interface) | text, buttons, layout, borders |
| 22 Windows | title, resolution, multiple windows, clear color |
| 23 Movement | smooth follow, fixed timestep, steering |
| 24 Async Tasks | AsyncComputeTaskPool, channels |
| 25 Shader Intro | custom materials, WGSL, 2D shaders |
| 26 glTF Models | loading, SceneRoot, traversal, materials |
| 27 Picking | hover, click, drag |
| 28 Assets | AssetServer, hot reload, embedded assets |
| 29 Events & Observers | Message, MessageReader, Observer |
| 30 Run Conditions | run_if, condition composition, system piping |
| 31 Fixed Timestep | FixedUpdate, Time\<Fixed\> |
| 32 Change Detection | Changed/Added, RemovedComponents |
| 33 Hierarchy & Parent/Child | Parent/Children, add_child |
| 34 Advanced ECS | one-shot systems, generic systems, parallel queries |
| 35 Tips & Tricks | cooldowns, log layers, no-winit apps |

## Technical Notes

- Engine version: **Bevy 0.19** (default features, no extra configuration needed)
- Each exercise is a feature-gated module: `cargo test -p exercises --features ex_0301`
- All exercise tests run headless — no window, no external file dependencies
- The learning path mirrors the official example categories (2D/3D/UI/audio/camera...) and the quick-start guide (ECS/plugins/resources...)

## Disk Space Management

- `run / test / verify` automatically **delete the executable produced by the current check** (each Bevy debug binary is 40–60 MB),
  while keeping the rlib/rmeta build cache — the next check stays incremental, with almost no speed impact.
- To fully purge build artifacts and reclaim space: `cargo clean` (the next check then rebuilds Bevy from scratch, a few minutes).

## FAQ

**Q: The first check takes forever to compile?**
A: Bevy's dependency tree is large; a cold build takes a few minutes — that's normal. Subsequent incremental builds are fast.

**Q: `bevylings run` opened a game window?**
A: Some exercises (e.g. mini-games) do open a window. On headless setups, use `bevylings test` instead.

**Q: Where is my progress stored?**
A: `.bevylings/state.json` (gitignored, so it won't pollute the repo). `bevylings reset` clears it.

**Q: Why do build artifacts keep growing?**
A: See "Disk Space Management" above — each check auto-cleans the executable it just produced.

**Q: Can I add my own exercises?**
A: See the authoring spec in `_specs/AUTHORING.md`, generate with `_tools/gen_*.py`, and validate with `_tools/verify_*.py`.

## Contributing

Contributions welcome — bug fixes for exercises, new chapters, translation improvements, or tooling:

1. Fork this repo and create a new branch
2. When editing exercises, make sure: the buggy version is caught by tests, and the reference solution passes all tests
3. Run `_tools/verify_exercises.py` and `_tools/verify_solutions.py` to validate
4. Open a PR describing your changes and validation results

## License

MIT License — see [LICENSE](LICENSE).

- All exercise code adapted from the Bevy repository (MIT): <https://github.com/bevyengine/bevy/tree/latest/examples>
- Documentation and tutorials from <https://bevy.org/learn/>
