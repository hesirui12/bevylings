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

## 硬性约束
- 只写 `exercises/src/chapters/<slug>/` 与 `solutions/src/chapters/<slug>/` 下的练习文件。**不要修改任何其他文件**（Cargo.toml、main.rs、mod.rs、manifest、规范都不要碰）。
- **绝对不要运行 cargo / rustc / 任何编译命令**（编译验证由主代理统一做，避免并发冲突）。
- 不引入任何 bevy 之外的 crate；不使用非默认 feature（默认已含 2d/3d/ui/audio/picking/scene/gltf/gizmos/state/asset/log）。
- 每章 bug 类型必须混合（编译错误型 + 逻辑错误型），且按规范第 4 节执行。
- 每章内部练习难度递进。
- 中文说明要耐心、面向"只学过 Rust、没接触过 Bevy 和游戏开发"的初学者。
- 交付前用规范第 8 节检查清单逐项自查。

完成后再把本任务文件里的检查清单打勾输出到最终回复里。


## 负责章节：06 ch06_breakout —— 打砖块小游戏（共 4 个练习，ID: ex_0601, ex_0602, ex_0603, ex_0604）

本章重点：经典 Breakout：挡板、球、碰撞、计分（拆成小块学习）

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/showcase/breakout.rs`

### 本章练习安排
  - exercise_01（ex_0601）：挡板组件与键盘左右移动：读键盘输入改挡板 x 坐标（showcase/breakout.rs 中 paddle 部分简化）
  - exercise_02（ex_0602）：球：速度向量与每帧移动（showcase/breakout.rs 中 ball 移动部分简化）
  - exercise_03（ex_0603）：球撞墙反弹：x/y 边界翻转速度（showcase/breakout.rs 碰撞部分简化）
  - exercise_04（ex_0604）：计分资源：命中砖块 +1，打印分数（showcase/breakout.rs 得分部分简化）


## 负责章节：13 ch13_games —— 小游戏合集（共 5 个练习，ID: ex_1301, ex_1302, ex_1303, ex_1304, ex_1305）

本章重点：综合小游戏：菜单、加载屏、弹跳球、吃蛋糕

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/showcase/contributors.rs`
  - `_sources/bevy-src/examples/showcase/game_menu.rs`
  - `_sources/bevy-src/examples/showcase/loading_screen.rs`
  - `_sources/bevy-src/examples/showcase/alien_cake_addict.rs`

### 本章练习安排
  - exercise_01（ex_1301）：弹跳球：contributors 的物理简化（showcase/contributors.rs 简化，去掉资产）
  - exercise_02（ex_1302）：菜单与状态：game_menu 的状态切换（showcase/game_menu.rs 简化成 ECS 状态）
  - exercise_03（ex_1303）：加载屏：loading_screen 的进度与状态（showcase/loading_screen.rs 简化）
  - exercise_04（ex_1304）：吃蛋糕：alien_cake_addict 的吃到加分逻辑（showcase/alien_cake_addict.rs 简化）
  - exercise_05（ex_1305）：胜利判定：breakout 的过关条件（showcase/breakout.rs 一部分简化）


## 交付检查清单（完成后逐项自查并在回复中报告）
- [ ] 2 章、9 个练习全部完成（每章数量正确）
- [ ] 每个练习都有 exercise 与 solution 两个文件，文件名正确（exercise_01.rs ...）
- [ ] 每个练习有出处 URL、中文讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）
- [ ] solution 是修好 bug 的版本，测试与 exercise 一致
- [ ] 无第三方 crate、无非默认 feature、测试不依赖外部文件
- [ ] 每章 bug 类型混合（编译错误 + 逻辑错误）
