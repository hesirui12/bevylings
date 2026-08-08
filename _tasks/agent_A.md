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


## 负责章节：01 ch01_hello_engine —— 你好，Bevy！（共 6 个练习，ID: ex_0101, ex_0102, ex_0103, ex_0104, ex_0105, ex_0106）

本章重点：创建 App、添加 DefaultPlugins、run()、println 日志

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/app/empty.rs`
  - `_sources/bevy-src/examples/app/empty_defaults.rs`
  - `_sources/learn/md/getting-started.md`

### 本章练习安排
  - exercise_01（ex_0101）：最小 App：App::new().run()，什么都不做的程序（learn getting-started 代码）
  - exercise_02（ex_0102）：添加系统与 Startup：App::new().add_systems(Startup, ...)（getting-started + app/empty_defaults.rs）
  - exercise_03（ex_0103）：多个 Update 系统与顺序：hello_world + 计数器（ecs/startup_system.rs 与 app/empty_defaults.rs 简化）
  - exercise_04（ex_0104）：Local 系统本地状态：打印运行次数（app/logs.rs 中 counter 模式 / app/headless.rs）
  - exercise_05（ex_0105）：无窗口运行：MinimalPlugins + ScheduleRunnerPlugin::run_once（app/headless.rs 简化）
  - exercise_06（ex_0106）：AppExit 退出循环：运行 N 帧后发送 AppExit 事件结束程序（app/return_after_run.rs 简化）


## 负责章节：02 ch02_apps —— App 与应用（共 6 个练习，ID: ex_0201, ex_0202, ex_0203, ex_0204, ex_0205, ex_0206）

本章重点：无窗口运行、ScheduleRunnerPlugin、日志级别、AppExit、手动 update 循环、线程池

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/app/headless.rs`
  - `_sources/bevy-src/examples/app/no_renderer.rs`
  - `_sources/bevy-src/examples/app/logs.rs`
  - `_sources/bevy-src/examples/app/settings.rs`
  - `_sources/bevy-src/examples/app/return_after_run.rs`
  - `_sources/bevy-src/examples/app/custom_loop.rs`
  - `_sources/bevy-src/examples/app/thread_pool_resources.rs`

### 本章练习安排
  - exercise_01（ex_0201）：手动驱动循环：app.update() 逐帧手动运行（app/custom_loop.rs 简化，去掉渲染相关）
  - exercise_02（ex_0202）：日志级别与过滤：LogPlugin level/filter，info!/warn!/error!（app/logs.rs 简化）
  - exercise_03（ex_0203）：自定义插件组：定义 PluginGroup 组合系统（app/plugin_group.rs 简化）
  - exercise_04（ex_0204）：默认插件配置：DefaultPlugins.set(WindowPlugin/LogPlugin ...)（app/settings.rs 简化）
  - exercise_05（ex_0205）：线程池资源：TaskPool 资源读取线程数（app/thread_pool_resources.rs 简化）
  - exercise_06（ex_0206）：无渲染保留逻辑：headless 计算任务（app/no_renderer.rs 简化）


## 负责章节：04 ch04_plugins —— 插件（共 4 个练习，ID: ex_0401, ex_0402, ex_0403, ex_0404）

本章重点：Plugin trait、插件组、插件配置、插件与系统/资源

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/app/plugin.rs`
  - `_sources/bevy-src/examples/app/plugin_group.rs`
  - `_sources/learn/md/getting-started_plugins.md`

### 本章练习安排
  - exercise_01（ex_0401）：自定义插件：struct 实现 Plugin trait，build 里加系统（app/plugin.rs 简化）
  - exercise_02（ex_0402）：插件添加资源与消息：build 里 insert_resource + add_systems（app/plugin.rs 简化扩展）
  - exercise_03（ex_0403）：自定义插件组 PluginGroup（app/plugin_group.rs 简化）
  - exercise_04（ex_0404）：插件配置字段与 DefaultPlugins.set 修改默认插件（app/settings.rs 简化）


> ⚠️ 注意：ch02 的 exercise_01 已由主代理完成（样例）。请只写 ch02 的 exercise_02 到 exercise_06（共 5 个），本章实际工作量 = 6(ch01) + 5(ch02) + 4(ch04) = 15 个练习。

## 交付检查清单（完成后逐项自查并在回复中报告）
- [ ] 3 章、16 个练习全部完成（每章数量正确）
- [ ] 每个练习都有 exercise 与 solution 两个文件，文件名正确（exercise_01.rs ...）
- [ ] 每个练习有出处 URL、中文讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）
- [ ] solution 是修好 bug 的版本，测试与 exercise 一致
- [ ] 无第三方 crate、无非默认 feature、测试不依赖外部文件
- [ ] 每章 bug 类型混合（编译错误 + 逻辑错误）
