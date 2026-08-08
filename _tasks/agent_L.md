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


## 负责章节：27 ch27_picking —— 拾取点击（共 4 个练习，ID: ex_2701, ex_2702, ex_2703, ex_2704）

本章重点：Pointer<Over>/<Click>、拾取事件、拖拽、调试显示

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/picking/simple_picking.rs`
  - `_sources/bevy-src/examples/picking/mesh_picking.rs`
  - `_sources/bevy-src/examples/picking/sprite_picking.rs`
  - `_sources/bevy-src/examples/picking/debug_picking.rs`
  - `_sources/bevy-src/examples/picking/dragdrop_picking.rs`

### 本章练习安排
  - exercise_01（ex_2701）：基础拾取：Pointer<Over> 悬停变色（picking/simple_picking.rs 简化）
  - exercise_02（ex_2702）：网格拾取：mesh_picking 点击高亮（picking/mesh_picking.rs 简化）
  - exercise_03（ex_2703）：Sprite 拾取：sprite_picking（picking/sprite_picking.rs 简化）
  - exercise_04（ex_2704）：拖拽拾取：dragdrop（picking/dragdrop_picking.rs 简化）


## 负责章节：28 ch28_assets —— 资产管理（共 5 个练习，ID: ex_2801, ex_2802, ex_2803, ex_2804, ex_2805）

本章重点：AssetServer、load 回调、热重载、嵌入资产、生成资产

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/asset/asset_loading.rs`
  - `_sources/bevy-src/examples/asset/hot_asset_reloading.rs`
  - `_sources/bevy-src/examples/asset/embedded_asset.rs`
  - `_sources/bevy-src/examples/asset/generated_assets.rs`
  - `_sources/bevy-src/examples/asset/asset_settings.rs`
  - `_sources/bevy-src/examples/asset/alter_sprite.rs`

### 本章练习安排
  - exercise_01（ex_2801）：资产加载：AssetServer.load + Assets 集合（asset/asset_loading.rs 简化）
  - exercise_02（ex_2802）：热重载：file_watcher（asset/hot_asset_reloading.rs 简化，注意 file_watcher 非默认 feature——若编译不过请改写为普通 load + AssetEvent）
  - exercise_03（ex_2803）：嵌入资产：embedded_asset 宏（asset/embedded_asset.rs 简化）
  - exercise_04（ex_2804）：生成资产：generated_assets（asset/generated_assets.rs 简化）
  - exercise_05（ex_2805）：资产设置：asset_settings（asset/asset_settings.rs 简化）


## 负责章节：32 ch32_change_detection —— 变更检测（共 3 个练习，ID: ex_3201, ex_3202, ex_3203）

本章重点：Added/Changed、RemovedComponents、EntityDisabled

### 本章官方源码（必须阅读）
  - `_sources/bevy-src/examples/ecs/change_detection.rs`
  - `_sources/bevy-src/examples/ecs/removal_detection.rs`
  - `_sources/bevy-src/examples/ecs/entity_disabling.rs`

### 本章练习安排
  - exercise_01（ex_3201）：Changed<T>：组件变化才处理（ecs/change_detection.rs 简化）
  - exercise_02（ex_3202）：Added<T> 与 RemovedComponents：组件生命周期事件（ecs/removal_detection.rs 简化）
  - exercise_03（ex_3203）：EntityDisabled：禁用实体（ecs/entity_disabling.rs 简化）


## 交付检查清单（完成后逐项自查并在回复中报告）
- [ ] 3 章、12 个练习全部完成（每章数量正确）
- [ ] 每个练习都有 exercise 与 solution 两个文件，文件名正确（exercise_01.rs ...）
- [ ] 每个练习有出处 URL、中文讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）
- [ ] solution 是修好 bug 的版本，测试与 exercise 一致
- [ ] 无第三方 crate、无非默认 feature、测试不依赖外部文件
- [ ] 每章 bug 类型混合（编译错误 + 逻辑错误）
