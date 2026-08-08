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


## 负责章节：09 ch09_animation —— 动画（共 4 个练习，ID: ex_0901, ex_0902, ex_0903, ex_0904）

本章重点：AnimatedTransform、缓动、AnimationPlayer、动画事件

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/animation/animated_transform.rs`
  - `_sources/bevy-src/examples/animation/eased_motion.rs`
  - `_sources/bevy-src/examples/animation/easing_functions.rs`
  - `_sources/bevy-src/examples/animation/color_animation.rs`
  - `_sources/bevy-src/examples/animation/animation_events.rs`

### 本章练习安排
  - exercise_01（ex_0901）：缓动运动：eased_motion 的动画过程（animation/eased_motion.rs 简化）
  - exercise_02（ex_0902）：缓动函数：EasingFunction 系列（animation/easing_functions.rs 简化）
  - exercise_03（ex_0903）：动画组件：AnimatedTransform 添加与播放（animation/animated_transform.rs 简化）
  - exercise_04（ex_0904）：颜色动画：ColorAnimation 与色相（animation/color_animation.rs 简化）


## 负责章节：14 ch14_gizmos —— 调试绘制（共 4 个练习，ID: ex_1401, ex_1402, ex_1403, ex_1404）

本章重点：Gizmos：线/圆/矩形/轴、2D/3D、调试可视化

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/gizmos/2d_gizmos.rs`
  - `_sources/bevy-src/examples/gizmos/3d_gizmos.rs`
  - `_sources/bevy-src/examples/gizmos/axes.rs`
  - `_sources/bevy-src/examples/gizmos/transform_gizmo.rs`
  - `_sources/bevy-src/examples/gizmos/2d_text_gizmos.rs`

### 本章练习安排
  - exercise_01（ex_1401）：2D Gizmos：线、矩形、圆（gizmos/2d_gizmos.rs 简化）
  - exercise_02（ex_1402）：3D Gizmos：立方体、球、射线（gizmos/3d_gizmos.rs 简化）
  - exercise_03（ex_1403）：坐标轴：axes gizmos（gizmos/axes.rs 简化）
  - exercise_04（ex_1404）：变换 Gizmo：TransformGizmo 拖拽（gizmos/transform_gizmo.rs 简化）


## 负责章节：15 ch15_math —— 数学（共 4 个练习，ID: ex_1501, ex_1502, ex_1503, ex_1504）

本章重点：Vec2/Vec3 运算、样条、随机、边界盒、数学原语

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/math/cubic_splines.rs`
  - `_sources/bevy-src/examples/math/random_sampling.rs`
  - `_sources/bevy-src/examples/math/custom_primitives.rs`
  - `_sources/bevy-src/examples/math/bounding_2d.rs`
  - `_sources/bevy-src/examples/math/render_primitives.rs`

### 本章练习安排
  - exercise_01（ex_1501）：向量运算：Vec2/Vec3 加减、长度、归一化（基于 math/render_primitives.rs 或 bounding_2d.rs 简化）
  - exercise_02（ex_1502）：样条曲线：CubicSpline 与位置插值（math/cubic_splines.rs 简化）
  - exercise_03（ex_1503）：自定义数学原语：implement Bounded2d（math/custom_primitives.rs 简化）
  - exercise_04（ex_1504）：边界与相交：Aabb / bounding_2d 判定（math/bounding_2d.rs 简化）


## 交付检查清单（完成后逐项自查并在回复中报告）
- [ ] 3 章、12 个练习全部完成（每章数量正确）
- [ ] 每个练习都有 exercise 与 solution 两个文件，文件名正确（exercise_01.rs ...）
- [ ] 每个练习有出处 URL、中文讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）
- [ ] solution 是修好 bug 的版本，测试与 exercise 一致
- [ ] 无第三方 crate、无非默认 feature、测试不依赖外部文件
- [ ] 每章 bug 类型混合（编译错误 + 逻辑错误）
