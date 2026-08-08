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


## 负责章节：08 ch08_3d_basics —— 3D 基础（共 7 个练习，ID: ex_0801, ex_0802, ex_0803, ex_0804, ex_0805, ex_0806, ex_0807）

本章重点：Mesh3d、Camera3d、灯光、材质、父子、透明度、正交相机

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/3d/3d_shapes.rs`
  - `_sources/bevy-src/examples/3d/3d_scene.rs`
  - `_sources/bevy-src/examples/3d/lighting.rs`
  - `_sources/bevy-src/examples/3d/parenting.rs`
  - `_sources/bevy-src/examples/3d/transparency_3d.rs`
  - `_sources/bevy-src/examples/3d/orthographic.rs`
  - `_sources/bevy-src/examples/3d/vertex_colors.rs`

### 本章练习安排
  - exercise_01（ex_0801）：3D 场景：Camera3d + 立方体 + 灯光（3d/3d_scene.rs 与 3d/3d_shapes.rs 简化成命令式写法）
  - exercise_02（ex_0802）：多形状与颜色：3d_shapes 的球、圆柱、环面（3d/3d_shapes.rs 简化）
  - exercise_03（ex_0803）：灯光：PointLight / DirectionalLight / GlobalAmbientLight 区别（3d/lighting.rs 简化）
  - exercise_04（ex_0804）：父子与移动：父实体移动带动子实体（3d/parenting.rs 简化）
  - exercise_05（ex_0805）：透明度：3D 透明材质（3d/transparency_3d.rs 简化）
  - exercise_06（ex_0806）：正交相机：OrthographicProjection（3d/orthographic.rs 简化）
  - exercise_07（ex_0807）：顶点颜色网格：Mesh 顶点颜色（3d/vertex_colors.rs 简化）


## 负责章节：19 ch19_transform —— 变换（共 4 个练习，ID: ex_1901, ex_1902, ex_1903, ex_1904）

本章重点：Transform/GlobalTransform、translate/rotate/scale、LookAt、align

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/transforms/transform.rs`
  - `_sources/bevy-src/examples/transforms/translation.rs`
  - `_sources/bevy-src/examples/transforms/rotation.rs`
  - `_sources/bevy-src/examples/transforms/scale.rs`
  - `_sources/bevy-src/examples/transforms/align.rs`

### 本章练习安排
  - exercise_01（ex_1901）：Transform 与平移：from_xyz / translation 修改（transforms/transform.rs 与 translation.rs 简化）
  - exercise_02（ex_1902）：旋转：Quat 与 rotate（transforms/3d_rotation.rs 简化）
  - exercise_03（ex_1903）：缩放：scale 与局部缩放（transforms/scale.rs 简化）
  - exercise_04（ex_1904）：朝向：LookAt / align 对齐（transforms/align.rs 简化）


## 交付检查清单（完成后逐项自查并在回复中报告）
- [ ] 2 章、11 个练习全部完成（每章数量正确）
- [ ] 每个练习都有 exercise 与 solution 两个文件，文件名正确（exercise_01.rs ...）
- [ ] 每个练习有出处 URL、中文讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）
- [ ] solution 是修好 bug 的版本，测试与 exercise 一致
- [ ] 无第三方 crate、无非默认 feature、测试不依赖外部文件
- [ ] 每章 bug 类型混合（编译错误 + 逻辑错误）
