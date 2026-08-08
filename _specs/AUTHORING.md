# bevylings 练习作者规范（AUTHORING.md）

> 本文件是 subagent 编写练习的唯一权威规范。动手前**必须完整读完**本文件，
> 并阅读本任务指定的**所有**官方源码（`_sources/bevy-src/examples/...`）与学习指南页面。

## 1. 项目背景

bevylings 是一个类似 rustlings 的 Bevy 练习工具：
- 学生是**只学过 Rust、从未接触过 Bevy 和游戏开发**的小白。
- 每个练习是官方 Bevy 示例/官方指南代码的**最小化精简版**，代码里**故意改错一处**（`// BUG:` 标注）。
- 学生任务：找到并修复这个 bug，让代码编译通过、测试通过。
- 每个练习 = 一个 `.rs` 模块文件（buggy 版） + 一个对应的参考答案文件（fixed 版）。
- Bevy 版本 **0.19.0**，只用默认 feature（2d/3d/ui/audio 全套默认已开，包含 picking、scene、gltf、gizmos、state）。

## 2. 你要写的文件

每章有两个目录：`exercises/src/chapters/<slug>/`（错误代码）和 `solutions/src/chapters/<slug>/`（正确代码）。
为本章每个练习各写两个文件：

| 练习 | 错误版 | 正确版 |
|---|---|---|
| 第 n 个 | `exercises/src/chapters/<slug>/exercise_<NN>.rs` | `solutions/src/chapters/<slug>/exercise_<NN>.rs` |

- `<NN>` = 两位序号（01, 02, ...）。
- **`mod.rs`、`Cargo.toml`、`main.rs` 等脚手架文件不要动**（由 `_tools/gen.py` 生成）。
- 文件名、模块名必须与上面一致（脚手架已按 manifest 生成，`mod.rs` 里已有 `#[cfg(feature = "ex_XXXX")] pub mod exercise_NN;`，你的文件名必须匹配）。

## 3. 练习文件模板（严格遵守）

```rust
//! # 练习 <章节>.<序号> —— <中文标题>
//!
//! 出处：https://bevy.org/examples-webgpu/<分类>/<示例名>/ （或 learn 指南 URL）
//!
//! ## 概念
//! （2-5 句面向 Rust 小白的解释，讲清楚本练习涉及的 Bevy 概念。
//!  不要用行话，可以类比普通 Rust 程序）
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run <编号>` 查看现象，改正后运行 `bevylings test <编号>` 让测试通过。
//!
//! 小贴士：<1 条实用提示，帮助新手理解上下文>

// I AM NOT DONE

use bevy::prelude::*;

// ...（核心代码，从官方源码精简而来；唯一的一处错误用 `// BUG: ...` 标出）...

#[cfg(test)]
mod tests {
    use super::*;

    // ...至少 2 个测试，buggy 代码上失败/编译不过，fixed 代码上全部通过...

}

// 提示：
// 1. ...
// 2. ...
```

要求：
- 整个文件（含注释）**≤ 160 行**，代码部分（不含注释）尽量 10~60 行。
- 顶部的 `// I AM NOT DONE` 必须保留（rustlings 惯例，表示尚未完成）。
- 有且只有**一处**故意错误，用 `// BUG:` 注释解释"这行本来的意图"（**不要直接说出正确答案**，而是说明它做错了什么）。
- 提示写在文件底部 `// 提示：` 后，2-3 条，从"哪里入手"到"具体 API"渐进。
- 测试写 `#[cfg(test)] mod tests`，用 `cargo test -p exercises --features ex_XXXX` 运行。

## 4. 放 bug 的规则（最重要）

- **每章内部，bug 类型尽量错开**：Rust 编译错误（类型、借用、方法名、参数错位、移动语义）与逻辑错误（比较反向、加减号、字段用错、资源没更新、查询过滤错）交替出现。
- bug 必须"小而教学"：改一个符号、一个参数、一个方法名、一个过滤条件、一个字段。**不要**删掉整个系统或改大段逻辑。
- bug 的难度循序渐进：每章前 1-2 个偏简单（一眼能看出），后面的稍难。
- bug 尽量让"编译失败"与"逻辑错误"在整章中混合。
- 对编译错误型 bug：保证修好后能编译、测试全过。
- 对逻辑错误型 bug：保证 buggy 版能编译但至少 1 个测试失败（或者运行输出明显错误），fixed 版测试全过。
- **禁止**在测试里依赖随机数、全局时间、外部文件；测试必须确定性、可重复。

## 5. Bevy 0.19 已验证 API 速查（必须用这些写法）

通用：
- 最小程序：`App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup).add_systems(Update, sys).run();`
- 组件：`#[derive(Component)] struct Pos(f32);`；Bundle：`#[derive(Bundle)]`；资源：`#[derive(Resource)]`。
- 生成实体：`commands.spawn((CompA, CompB, Transform::from_xyz(...)))`；`commands.spawn_empty()`。
- 查询：`fn sys(query: Query<&Pos>, mut q2: Query<&mut Pos>)`，迭代 `for p in &query` / `&mut q2`。
- `Query::single` / `query.single_mut()`（0.19 中 single_mut 存在）；`query.get(entity)` 返回 Result。
- `Commands`：`commands.entity(e).insert(...)` / `.despawn()`。
- 相机：`commands.spawn(Camera2d);`（unit struct）、`commands.spawn(Camera3d::default());`
- 光源：`PointLight { color, intensity, shadows_enabled, ..default() }`、`DirectionalLight`、`AmbientLight`（0.19 用 `GlobalAmbientLight`）。
- 网格/材质：`Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(w,h))))`、`MeshMaterial2d(materials.add(Color::from(COLOR)))`；3D 用 `Mesh3d` / `MeshMaterial3d<StandardMaterial>`；`meshes.add(Cuboid::new(1.,1.,1.))`；`Transform::from_xyz`。
- Sprite：`Sprite::from_image(asset_server.load("..."))`、`Sprite::from_color(Color::srgb(r,g,b), Vec2::new(w,h))`。
- 时间：`time.delta_secs()`、`Timer::from_seconds(1.0, TimerMode::Repeating)`、`timer.tick(time.delta())`、`just_finished()`、`finished()`、`TimerMode::Once`。
- 输入：`Res<ButtonInput<KeyCode>>`、`keyboard.just_pressed(KeyCode::KeyW)` / `pressed` / `just_released`；鼠标 `Res<ButtonInput<MouseButton>>`、`MouseButton::Left`；`Res<MouseMotion>`（AccumulatedMouseMotion?）以鼠标源文件为准；触摸 `TouchInput`/`Touches`。
- 状态：`#[derive(States, Debug, Clone, PartialEq, Eq, Hash)] enum AppState { Menu, InGame }`；`app.init_state::<AppState>()`；系统参数 `ResMut<NextState<AppState>>` + `next_state.set(AppState::InGame)`；调度 `OnEnter(AppState::Menu)` / `OnExit(...)` / `OnTransition { from, to }`。
- 事件/消息（0.19 改名！）：`#[derive(Message)] struct MyMsg;`、`MessageWriter<MyMsg>` 的 `.write(MyMsg)`、`MessageReader<MyMsg>` 的 `.read()`；旧名 `EventWriter` 已删除。
- 观察者：`.add_observer(fn_system)`；系统里参数 `On<MyEvent>`、`Trigger<'_, MyEvent>`；`commands.trigger(MyEvent { .. })`；组件生命周期 `On<Add<Comp>>`、`On<Remove<Comp>>`。
- 运行条件：`.run_if(condition)`；`bevy::ecs::schedule::common_conditions::*`（如 `resource_exists`、`resource_changed`）；闭包 `|res: Res<T>| res.0 > 0`。
- 固定时间步：`app.add_systems(FixedUpdate, sys)`；`Time<Fixed>`、`time.delta_secs()`。
- 层级：`commands.entity(parent).add_child(child)`；`Parent`、`Children`、`Query<&Parent>` / `&Children`；`child_of(parent)`。
- Gizmos：系统参数 `mut gizmos: Gizmos`；`gizmos.line_2d(a, b, color)`、`gizmos.rect_2d(Isometry2d::IDENTITY, size, color)`、`gizmos.circle_2d(...)`、`gizmos.ray_3d(...)`。
- UI：`Node { width: px(100), height: px(50), ..default() }`（0.19 用 `px()` 函数！）、`Text::new("...")`、`TextFont { font_size: FontSize::Px(16.0), ..default() }`、`TextColor(color)`、`Button`、`Interaction`（`Interaction::Pressed/Hovered/None`）、`BackgroundColor(color.into())`、`FlexDirection::Row/Column`、`JustifyContent::Center`、`AlignItems::Center`、`Val::Px(5.)`（若示例用 Val）。**以官方源文件为准**。
- 窗口：`WindowPlugin { primary_window: Some(Window { title, resolution, resizable, ..default() }), ..default() }`；`Res<Window>`；`app.insert_resource(ClearColor(Color::srgb(...)))`；`add_plugins(DefaultPlugins.set(WindowPlugin { ... }))`。
- 无窗口/无渲染：`MinimalPlugins`（含 TaskPool + Time + FrameCount + ScheduleRunner）；`ScheduleRunnerPlugin::run_once()`；`DefaultPlugins.set(WindowPlugin { primary_window: None, ..default() })`。
- 日志：`info!` / `warn!` / `error!`（从 `bevy::prelude::*` 导入）；`LogPlugin { level: Level::WARN, filter: "wgpu=error,bevy_render=info".to_string(), ..default() }`。
- 场景/BSN（0.19 全新语法！）：场景用 `fn scene() -> impl SceneList { bsn_list![...] }` 定义，注册 `.add_systems(Startup, scene.spawn())`；`bsn! { ... }` 定义单个 Scene；`on(|e: On<Pointer<Press>>| ...)` 绑定事件。加载外部模型（glTF）用 `commands.spawn(WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/x.gltf"))))`。**旧版 `SceneRoot` 已不存在，不要用**。
- 异步：`AsyncComputeTaskPool::get()`、`task_pool.spawn(async move { ... })`、`Task<T>` 组件、`bevy::tasks::futures::check_ready`、`Task::is_finished()`。
- 音频：`AudioPlayer::new(asset_server.load("sounds/xxx.ogg"))`；`AudioBundle`；`PlaybackSettings::LOOP`；`Res<Assets<AudioSource>>` 事件 `AudioPlaybackEvents` 以官方源为准。
- 拾取：`.observe(|click: On<Pointer<Click>>| ...)`、`On<Pointer<Over>>`、`On<Pointer<Out>>`、`On<Pointer<Drag>>`、`On<Pointer<Press>>`；`PointerInteraction`、`PointerLocation` 以官方源为准。
- 变更检测：`Query<&T, Changed<T>>`、`Added<T>`；`RemovedComponents<T>` 系统参数；`EntityDisabled`。
- 诊断：`app.add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))`；自定义 `Diagnostic` + `DiagnosticsStore`。
- 数学：`Vec2::new`、`Vec3::new`、`Vec2::splat`、`.length()`、`.normalize()`、`.dot()`、`Vec2::X/Y/ZERO`、`Quat::from_rotation_z(angle)`、`Quat::from_rotation_x`、`Transform::from_rotation`、`Isometry2d`。

**通用原则**：凡是拿不准的 API，去读你任务清单里对应的官方源文件，照抄它的写法（这是"全站代码"的一部分）。不要自创写法。

## 6. 测试模式

- 测试**不得**使用 `DefaultPlugins`（不能开窗口、不能加载外部文件）。用：
  ```rust
  let mut app = App::new();
  app.add_systems(Startup, setup);          // 需要时
  app.add_systems(Update, my_system);
  app.update();                              // 手动跑 1 帧（第一次会跑 Startup）
  app.update();                              // 多跑几帧按需
  let mut q = app.world_mut().query::<&Pos>();
  let pos = q.single(app.world());
  assert_eq!(pos.0, 42.0);
  ```
- 纯函数型练习：直接测函数返回值，不需要 App。
- 需要 Time 的测试：`app.add_plugins(MinimalPlugins)`（提供 Time）后再 `app.update()`。
- 断言用 `assert_eq!`/`assert!`，**至少 2 个测试或 2 组断言**。
- buggy 版上：要么编译不过，要么至少一个断言失败。fixed 版上：全过。
- 测试里不要 `println!` 噪音之外的东西；不要用 `panic!` 当断言。

## 7. 禁止事项（违反 = 重写）

- ❌ 不引入任何 bevy 之外的第三方 crate（官方示例里的 chacha20/rand 等一律删掉或换成内置写法）。
- ❌ 不使用非默认 feature（不要碰 `bevy::dev_tools`、`ci_testing`、`bevy_inspect` 等）。
- ❌ 不在测试里 `asset_server.load(...)` 真实文件或读取外部路径（资产型练习的"加载"只出现在 main/run 的可视化路径，测试只验证逻辑）。
- ❌ 不运行 `cargo`（编译验证由主代理统一做，避免 cargo 锁冲突）。
- ❌ 不修改脚手架文件（Cargo.toml、main.rs、mod.rs、chapters/mod.rs）。
- ❌ 不删除或改掉 `// I AM NOT DONE`；不加多个 `// BUG:`（只允许一处）。
- ❌ 文件不能超过 160 行；代码要精简到只保留本练习要讲的概念。
- ❌ 不要照抄整个大示例（如整个 breakout 400 行）——必须**裁剪**成 10~60 行核心代码。
- ❌ 不要用英文/机翻式中文写说明——面向中国 Rust 初学者，中文要自然、耐心、循序渐进。
- ❌ 不要在头注释里写"正确答案"；提示可以，但不许直接给出修复后的整行代码。
- ❌ 练习题的目标概念必须与本章 focus 一致；出处 URL 必须真实（对应官方示例名）。

## 8. 完成检查清单（交付前逐项自查）

1. 本章每个练习都有 `exercises/.../exercise_NN.rs` 与 `solutions/.../exercise_NN.rs` 两个文件，数量与任务要求一致。
2. 每个练习文件：有出处 URL、中文概念讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）。
3. 参考答案文件 = 修好 bug 的版本，无 `// BUG:`、无 `// I AM NOT DONE`，测试与练习版一致。
4. 代码风格与官方源文件一致（API 照抄官方），不使用任何未在"第 5 节"或官方源中出现的写法。
5. 没有第三方 crate、没有非默认 feature、没有外部文件依赖（测试路径）。
6. 每章内部 bug 类型有变化（编译错误 + 逻辑错误混合）。
7. 阅读 2-3 个其他章节已有练习作为风格参照（若存在）。
