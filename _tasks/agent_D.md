# bevylings 练习作者任务（subagent）

你是 bevylings（一个类似 rustlings 的 Bevy 0.19 练习工具）的练习作者。
项目根目录：`C:\Users\jcsyh\Documents\AI夜间巡航\bevylings`

## 第一步：读规范（必须完整读完）
`_specs/AUTHORING.md` —— 这是唯一的格式权威，含文件模板、放 bug 规则、Bevy 0.19 已验证 API 速查、测试模式、禁止事项、检查清单。

## 第二步：读风格参照（已完成的样例，必须模仿其风格）
- `exercises/src/chapters/ch02_apps/exercise_01.rs` 与其参考答案 `solutions/src/chapters/ch02_apps/exercise_01.rs`
- `exercises/src/chapters/ch03_ecs/exercise_01.rs` 与其参考答案 `solutions/src/chapters/ch03_ecs/exercise_01.rs`
（注意：练习文件入口函数是 `pub fn run()` 而不是 `fn main()`；模块文件名为 `exercise_01.rs`、`exercise_02.rs`……）

## 第三步：读你负责章节的官方源码（本地已克隆，是 https://bevy.org/examples-webgpu/ 与 https://bevy.org/learn/ 的全部代码）
下方"本章官方源码"列出的文件必须逐个阅读，练习代码必须基于它们改写（这是"使用全站代码"的要求）。

## 第四步：撰写练习
- 每个练习两个文件：`exercises/src/chapters/<slug>/exercise_NN.rs`（错误版）与 `solutions/src/chapters/<slug>/exercise_NN.rs`（正确版）。
- 每个练习的主题见"本章练习安排"。练习数量必须与安排一致。
- 参考答案 = 修好 bug 的版本：删掉 `// I AM NOT DONE`、删掉 BUG 注释行、应用修复；测试保持不变。


## ⏱️ 时间节奏（重要！务必遵守）
你有 90 分钟预算。请严格按此节奏推进，不要陷入无限调研：
1. 前 15 分钟：快速通读规范 _specs/AUTHORING.md + 2 个样例（ch02/01、ch03/01 的 exercise 与 solution）+ 本章官方源码（边读边记关键 API）。
2. 之后每 5~8 分钟写一个练习（exercise + solution 两个文件一起写，写完即过）。
3. 遇到不确定的 API：立即在对应官方源码里 grep 确认，不要上网查。
4. 最后 15 分钟：按规范第 8 节检查清单自查，补齐遗漏。
如果发现时间不够，优先保证"所有练习文件数量齐全 + 每章 bug 类型混合 + 测试存在"，再完善讲解文字。

## 硬性约束
- 只写 `exercises/src/chapters/<slug>/` 与 `solutions/src/chapters/<slug>/` 下的练习文件。**不要修改任何其他文件**（Cargo.toml、main.rs、mod.rs、manifest、规范都不要碰）。
- **绝对不要运行 cargo / rustc / 任何编译命令**（编译验证由主代理统一做，避免并发冲突）。
- 不引入任何 bevy 之外的 crate；不使用非默认 feature（默认已含 2d/3d/ui/audio/picking/scene/gltf/gizmos/state/asset/log）。
- 每章 bug 类型必须混合（编译错误型 + 逻辑错误型），且按规范第 4 节执行。
- 每章内部练习难度递进。
- 中文说明要耐心、面向"只学过 Rust、没接触过 Bevy 和游戏开发"的初学者。
- 交付前用规范第 8 节检查清单逐项自查。

完成后再把本任务文件里的检查清单打勾输出到最终回复里。


## 负责章节：07 ch07_2d_basics —— 2D 基础（共 7 个练习，ID: ex_0701, ex_0702, ex_0703, ex_0704, ex_0705, ex_0706, ex_0707）

本章重点：Sprite、2D 形状、Transform 移动/旋转/缩放、透明度、Mesh2d、Text2d

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/2d/sprite.rs`
  - `_sources/bevy-src/examples/2d/2d_shapes.rs`
  - `_sources/bevy-src/examples/2d/move_sprite.rs`
  - `_sources/bevy-src/examples/2d/rotation.rs`
  - `_sources/bevy-src/examples/2d/transparency_2d.rs`
  - `_sources/bevy-src/examples/2d/mesh2d.rs`
  - `_sources/bevy-src/examples/2d/sprite_flipping.rs`
  - `_sources/bevy-src/examples/2d/sprite_scale.rs`
  - `_sources/bevy-src/examples/2d/text2d.rs`

### 本章练习安排
  - exercise_01（ex_0701）：Sprite 生成：Sprite::from_color 与 from_image（2d/sprite.rs 简化，测试不加载图片）
  - exercise_02（ex_0702）：2D 形状：Mesh2d + Rectangle/Circle 与 MeshMaterial2d（2d/2d_shapes.rs 简化）
  - exercise_03（ex_0703）：Transform 移动：用 Time.delta_secs() 每帧移动（2d/move_sprite.rs 简化）
  - exercise_04（ex_0704）：旋转：Quat::from_rotation_z 与 transform.rotate_z（2d/rotation.rs 简化）
  - exercise_05（ex_0705）：缩放与翻转：transform.scale / Sprite flip（2d/sprite_scale.rs 与 sprite_flipping.rs 简化）
  - exercise_06（ex_0706）：透明度：Sprite alpha / 透明度排序（2d/transparency_2d.rs 简化）
  - exercise_07（ex_0707）：Text2d：Text2d::new 与 TextFont（2d/text2d.rs 简化）


## 负责章节：23 ch23_movement —— 移动（共 3 个练习，ID: ex_2301, ex_2302, ex_2303）

本章重点：平滑跟随、固定时间步物理、转向、屏幕坐标与世界坐标

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/movement/smooth_follow.rs`
  - `_sources/bevy-src/examples/movement/physics_in_fixed_timestep.rs`
  - `_sources/bevy-src/examples/2d/rotate_to_cursor.rs`
  - `_sources/bevy-src/examples/2d/2d_viewport_to_world.rs`

### 本章练习安排
  - exercise_01（ex_2301）：平滑跟随：smooth_follow 插值（movement/smooth_follow.rs 简化）
  - exercise_02（ex_2302）：固定时间步移动：FixedUpdate 下的移动（movement/physics_in_fixed_timestep.rs 简化）
  - exercise_03（ex_2303）：转向鼠标：rotate_to_cursor（2d/rotate_to_cursor.rs 简化）


## 交付检查清单（完成后逐项自查并在回复中报告）
- [ ] 2 章、10 个练习全部完成（每章数量正确）
- [ ] 每个练习都有 exercise 与 solution 两个文件，文件名正确（exercise_01.rs ...）
- [ ] 每个练习有出处 URL、中文讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）
- [ ] solution 是修好 bug 的版本，测试与 exercise 一致
- [ ] 无第三方 crate、无非默认 feature、测试不依赖外部文件
- [ ] 每章 bug 类型混合（编译错误 + 逻辑错误）
