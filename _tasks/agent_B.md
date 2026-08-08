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


## 负责章节：03 ch03_ecs —— ECS 实体组件系统（共 7 个练习，ID: ex_0301, ex_0302, ex_0303, ex_0304, ex_0305, ex_0306, ex_0307）

本章重点：Entity、Component、Bundle、System、Query、Commands、Startup/Update 调度

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/ecs/startup_system.rs`
  - `_sources/bevy-src/examples/ecs/ecs_guide.rs`
  - `_sources/bevy-src/examples/ecs/hierarchy.rs`
  - `_sources/learn/md/getting-started_ecs.md`

### 本章练习安排
  - exercise_01（ex_0301）：Bundle 组件组合：把多个组件打包成一个 Bundle 再生成实体（ecs/ecs_guide.rs 简化）
  - exercise_02（ex_0302）：Commands 生成与删除实体：spawn/despawn（ecs/delayed_commands.rs 简化）
  - exercise_03（ex_0303）：Query 多种访问：&T 与 &mut T 的冲突与拆分（ecs/ecs_guide.rs 或 nondeterministic_system_order.rs 简化）
  - exercise_04（ex_0304）：Query::single / get：精确取一个或按实体取（ecs/ecs_guide.rs 简化）
  - exercise_05（ex_0305）：Query 过滤器 With/Without/Or（ecs/ecs_guide.rs 或 callbacks.rs 简化）
  - exercise_06（ex_0306）：实体关系基础：Entity 编号使用（ecs/ecs_guide.rs 简化：把实体 id 存进组件）
  - exercise_07（ex_0307）：组合迭代：iter_combinations 简化版（两两配对计算距离）（ecs/iter_combinations.rs 简化，不用 rand）


## 负责章节：05 ch05_resources —— 资源（共 4 个练习，ID: ex_0501, ex_0502, ex_0503, ex_0504）

本章重点：Res/ResMut、init_resource/insert_resource/remove_resource、共享可变状态

### 本章官方源码（必须阅读）
  - `_sources/learn/md/getting-started_resources.md`
  - `_sources/bevy-src/examples/app/thread_pool_resources.rs`
  - `_sources/bevy-src/examples/ecs/immutable_components.rs`

### 本章练习安排
  - exercise_01（ex_0501）：insert_resource 与 Res 读取（learn resources 页代码简化：分数资源）
  - exercise_02（ex_0502）：ResMut 修改共享数据：每帧加分（learn resources 页代码简化）
  - exercise_03（ex_0503）：init_resource + Option<Res>：资源不存在时安全读取（learn resources + 官方代码简化）
  - exercise_04（ex_0504）：remove_resource 资源生命周期（learn resources 页 + ecs 简化写法）


## 负责章节：18 ch18_time —— 时间与计时器（共 4 个练习，ID: ex_1801, ex_1802, ex_1803, ex_1804）

本章重点：Time、delta_secs、Timer、Stopwatch、虚拟时间、固定时间步

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/time/time.rs`
  - `_sources/bevy-src/examples/time/timers.rs`
  - `_sources/bevy-src/examples/time/virtual_time.rs`
  - `_sources/bevy-src/examples/ecs/fixed_timestep.rs`

### 本章练习安排
  - exercise_01（ex_1801）：Time 与 delta：每帧按时间移动（time/time.rs 或 2d/move_sprite.rs 简化）
  - exercise_02（ex_1802）：Timer 组件：倒计时与重复（time/timers.rs 简化）
  - exercise_03（ex_1803）：Stopwatch 秒表（time/timers.rs 中 stopwatch 部分简化）
  - exercise_04（ex_1804）：虚拟时间：Virtual 时间缩放（time/virtual_time.rs 简化）


> ⚠️ 注意：ch03 的 exercise_01 已由主代理完成（样例）。请只写 ch03 的 exercise_02 到 exercise_07（共 6 个），本章实际工作量 = 6(ch03) + 4(ch05) + 4(ch18) = 14 个练习。

## 交付检查清单（完成后逐项自查并在回复中报告）
- [ ] 3 章、15 个练习全部完成（每章数量正确）
- [ ] 每个练习都有 exercise 与 solution 两个文件，文件名正确（exercise_01.rs ...）
- [ ] 每个练习有出处 URL、中文讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）
- [ ] solution 是修好 bug 的版本，测试与 exercise 一致
- [ ] 无第三方 crate、无非默认 feature、测试不依赖外部文件
- [ ] 每章 bug 类型混合（编译错误 + 逻辑错误）
