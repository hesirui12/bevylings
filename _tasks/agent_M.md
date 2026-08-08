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


## 负责章节：30 ch30_run_conditions —— 运行条件（共 3 个练习，ID: ex_3001, ex_3002, ex_3003）

本章重点：run_if、condition 组合、系统管道、状态作用域

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/ecs/run_conditions.rs`
  - `_sources/bevy-src/examples/ecs/system_piping.rs`
  - `_sources/bevy-src/examples/ecs/state_scoped.rs`

### 本章练习安排
  - exercise_01（ex_3001）：run_if 基础：condition 为真才运行（ecs/run_conditions.rs 简化）
  - exercise_02（ex_3002）：条件组合：and/or/not 与资源条件（ecs/run_conditions.rs 简化）
  - exercise_03（ex_3003）：系统管道：pipe 串联（ecs/system_piping.rs 简化）


## 负责章节：33 ch33_hierarchy —— 层级与父子关系（共 3 个练习，ID: ex_3301, ex_3302, ex_3303）

本章重点：Parent/Children、add_child、全局变换、关系组件

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/ecs/hierarchy.rs`
  - `_sources/bevy-src/examples/ecs/relationships.rs`
  - `_sources/bevy-src/examples/3d/parenting.rs`

### 本章练习安排
  - exercise_01（ex_3301）：Parent/Children：生成层级并查询（ecs/hierarchy.rs 简化）
  - exercise_02（ex_3302）：父子变换：add_child 后子随父动（3d/parenting.rs 简化）
  - exercise_03（ex_3303）：关系组件：ChildOf / 关系查询（ecs/relationships.rs 简化）


## 负责章节：34 ch34_ecs_advanced —— 高级 ECS（共 4 个练习，ID: ex_3401, ex_3402, ex_3403, ex_3404）

本章重点：OneShotSystem、泛型系统、并行查询、自定义调度、组合迭代

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/ecs/one_shot_systems.rs`
  - `_sources/bevy-src/examples/ecs/generic_system.rs`
  - `_sources/bevy-src/examples/ecs/parallel_query.rs`
  - `_sources/bevy-src/examples/ecs/custom_schedule.rs`
  - `_sources/bevy-src/examples/ecs/iter_combinations.rs`
  - `_sources/bevy-src/examples/ecs/delayed_commands.rs`

### 本章练习安排
  - exercise_01（ex_3401）：一次性系统：OneShotSystem 手动触发（ecs/one_shot_systems.rs 简化）
  - exercise_02（ex_3402）：泛型系统：GenericSystem（ecs/generic_system.rs 简化）
  - exercise_03（ex_3403）：并行查询：多 Query 同时迭代（ecs/parallel_query.rs 简化）
  - exercise_04（ex_3404）：自定义调度：Schedules / ScheduleLabel（ecs/custom_schedule.rs 简化）


## 负责章节：35 ch35_tips —— 实用技巧（共 3 个练习，ID: ex_3501, ex_3502, ex_3503）

本章重点：冷却计时、日志分层、无 winit 运行、任务队列

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/usage/cooldown.rs`
  - `_sources/bevy-src/examples/app/log_layers.rs`
  - `_sources/bevy-src/examples/app/without_winit.rs`

### 本章练习安排
  - exercise_01（ex_3501）：冷却计时：cooldown 模式（usage/cooldown.rs 简化）
  - exercise_02（ex_3502）：日志分层：log_layers 输出（app/log_layers.rs 简化）
  - exercise_03（ex_3503）：无 winit：不依赖窗口的 App（app/without_winit.rs 简化）


## 交付检查清单（完成后逐项自查并在回复中报告）
- [ ] 4 章、13 个练习全部完成（每章数量正确）
- [ ] 每个练习都有 exercise 与 solution 两个文件，文件名正确（exercise_01.rs ...）
- [ ] 每个练习有出处 URL、中文讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）
- [ ] solution 是修好 bug 的版本，测试与 exercise 一致
- [ ] 无第三方 crate、无非默认 feature、测试不依赖外部文件
- [ ] 每章 bug 类型混合（编译错误 + 逻辑错误）
