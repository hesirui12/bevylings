# -*- coding: utf-8 -*-
"""生成 13 个内容生成 subagent 的任务文件（_tasks/agent_*.md）。"""
import json, io, os

ROOT = r"C:\Users\jcsyh\Documents\AI夜间巡航\bevylings"
manifest = json.load(io.open(os.path.join(ROOT, "exercises.manifest.json"), encoding="utf-8"))
chapters = {c["slug"]: c for c in manifest["chapters"]}

PREAMBLE = """# bevylings 练习作者任务（subagent）

你是 bevylings（一个类似 rustlings 的 Bevy 0.19 练习工具）的练习作者。
项目根目录：`C:\\Users\\jcsyh\\Documents\\AI夜间巡航\\bevylings`

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
"""

TOPICS = {
    "ch01_hello_engine": [
        "最小 App：App::new().run()，什么都不做的程序（learn getting-started 代码）",
        "添加系统与 Startup：App::new().add_systems(Startup, ...)（getting-started + app/empty_defaults.rs）",
        "多个 Update 系统与顺序：hello_world + 计数器（ecs/startup_system.rs 与 app/empty_defaults.rs 简化）",
        "Local 系统本地状态：打印运行次数（app/logs.rs 中 counter 模式 / app/headless.rs）",
        "无窗口运行：MinimalPlugins + ScheduleRunnerPlugin::run_once（app/headless.rs 简化）",
        "AppExit 退出循环：运行 N 帧后发送 AppExit 事件结束程序（app/return_after_run.rs 简化）",
    ],
    "ch02_apps": [
        "手动驱动循环：app.update() 逐帧手动运行（app/custom_loop.rs 简化，去掉渲染相关）",
        "日志级别与过滤：LogPlugin level/filter，info!/warn!/error!（app/logs.rs 简化）",
        "自定义插件组：定义 PluginGroup 组合系统（app/plugin_group.rs 简化）",
        "默认插件配置：DefaultPlugins.set(WindowPlugin/LogPlugin ...)（app/settings.rs 简化）",
        "线程池资源：TaskPool 资源读取线程数（app/thread_pool_resources.rs 简化）",
        "无渲染保留逻辑：headless 计算任务（app/no_renderer.rs 简化）",
    ],
    "ch03_ecs": [
        "Bundle 组件组合：把多个组件打包成一个 Bundle 再生成实体（ecs/ecs_guide.rs 简化）",
        "Commands 生成与删除实体：spawn/despawn（ecs/delayed_commands.rs 简化）",
        "Query 多种访问：&T 与 &mut T 的冲突与拆分（ecs/ecs_guide.rs 或 nondeterministic_system_order.rs 简化）",
        "Query::single / get：精确取一个或按实体取（ecs/ecs_guide.rs 简化）",
        "Query 过滤器 With/Without/Or（ecs/ecs_guide.rs 或 callbacks.rs 简化）",
        "实体关系基础：Entity 编号使用（ecs/ecs_guide.rs 简化：把实体 id 存进组件）",
        "组合迭代：iter_combinations 简化版（两两配对计算距离）（ecs/iter_combinations.rs 简化，不用 rand）",
    ],
    "ch04_plugins": [
        "自定义插件：struct 实现 Plugin trait，build 里加系统（app/plugin.rs 简化）",
        "插件添加资源与消息：build 里 insert_resource + add_systems（app/plugin.rs 简化扩展）",
        "自定义插件组 PluginGroup（app/plugin_group.rs 简化）",
        "插件配置字段与 DefaultPlugins.set 修改默认插件（app/settings.rs 简化）",
    ],
    "ch05_resources": [
        "insert_resource 与 Res 读取（learn resources 页代码简化：分数资源）",
        "ResMut 修改共享数据：每帧加分（learn resources 页代码简化）",
        "init_resource + Option<Res>：资源不存在时安全读取（learn resources + 官方代码简化）",
        "remove_resource 资源生命周期（learn resources 页 + ecs 简化写法）",
    ],
    "ch06_breakout": [
        "挡板组件与键盘左右移动：读键盘输入改挡板 x 坐标（showcase/breakout.rs 中 paddle 部分简化）",
        "球：速度向量与每帧移动（showcase/breakout.rs 中 ball 移动部分简化）",
        "球撞墙反弹：x/y 边界翻转速度（showcase/breakout.rs 碰撞部分简化）",
        "计分资源：命中砖块 +1，打印分数（showcase/breakout.rs 得分部分简化）",
    ],
    "ch07_2d_basics": [
        "Sprite 生成：Sprite::from_color 与 from_image（2d/sprite.rs 简化，测试不加载图片）",
        "2D 形状：Mesh2d + Rectangle/Circle 与 MeshMaterial2d（2d/2d_shapes.rs 简化）",
        "Transform 移动：用 Time.delta_secs() 每帧移动（2d/move_sprite.rs 简化）",
        "旋转：Quat::from_rotation_z 与 transform.rotate_z（2d/rotation.rs 简化）",
        "缩放与翻转：transform.scale / Sprite flip（2d/sprite_scale.rs 与 sprite_flipping.rs 简化）",
        "透明度：Sprite alpha / 透明度排序（2d/transparency_2d.rs 简化）",
        "Text2d：Text2d::new 与 TextFont（2d/text2d.rs 简化）",
    ],
    "ch08_3d_basics": [
        "3D 场景：Camera3d + 立方体 + 灯光（3d/3d_scene.rs 与 3d/3d_shapes.rs 简化成命令式写法）",
        "多形状与颜色：3d_shapes 的球、圆柱、环面（3d/3d_shapes.rs 简化）",
        "灯光：PointLight / DirectionalLight / GlobalAmbientLight 区别（3d/lighting.rs 简化）",
        "父子与移动：父实体移动带动子实体（3d/parenting.rs 简化）",
        "透明度：3D 透明材质（3d/transparency_3d.rs 简化）",
        "正交相机：OrthographicProjection（3d/orthographic.rs 简化）",
        "顶点颜色网格：Mesh 顶点颜色（3d/vertex_colors.rs 简化）",
    ],
    "ch09_animation": [
        "缓动运动：eased_motion 的动画过程（animation/eased_motion.rs 简化）",
        "缓动函数：EasingFunction 系列（animation/easing_functions.rs 简化）",
        "动画组件：AnimatedTransform 添加与播放（animation/animated_transform.rs 简化）",
        "颜色动画：ColorAnimation 与色相（animation/color_animation.rs 简化）",
    ],
    "ch10_audio": [
        "播放音频：AudioPlayer + asset_server.load（audio/audio.rs 简化）",
        "音量与循环：PlaybackSettings 与音量控制（audio/audio_control.rs 简化）",
        "音效播放：PlaySoundEffect（audio/play_sound_effect.rs 简化）",
        "空间音频：AudioBundle 空间位置（audio/spatial_audio_3d.rs 简化）",
    ],
    "ch11_camera": [
        "2D 相机跟随：键盘控制相机移动（camera/2d_top_down_camera.rs 简化）",
        "相机环绕：围绕目标旋转（camera/camera_orbit.rs 简化）",
        "投影缩放：Projection::Orthographic 缩放（camera/projection_zoom.rs 简化）",
        "平滑跟随：插值跟随目标（camera/free_camera_controller.rs 或 movement/smooth_follow.rs 简化）",
    ],
    "ch12_diagnostics": [
        "内置诊断：FrameTimeDiagnosticsPlugin + LogDiagnosticsPlugin（diagnostics/log_diagnostics.rs 简化）",
        "自定义诊断：Diagnostic 注册与更新（diagnostics/custom_diagnostic.rs 简化）",
        "日志级别：LogPlugin level 设置（app/logs.rs 简化）",
    ],
    "ch13_games": [
        "弹跳球：contributors 的物理简化（showcase/contributors.rs 简化，去掉资产）",
        "菜单与状态：game_menu 的状态切换（showcase/game_menu.rs 简化成 ECS 状态）",
        "加载屏：loading_screen 的进度与状态（showcase/loading_screen.rs 简化）",
        "吃蛋糕：alien_cake_addict 的吃到加分逻辑（showcase/alien_cake_addict.rs 简化）",
        "胜利判定：breakout 的过关条件（showcase/breakout.rs 一部分简化）",
    ],
    "ch14_gizmos": [
        "2D Gizmos：线、矩形、圆（gizmos/2d_gizmos.rs 简化）",
        "3D Gizmos：立方体、球、射线（gizmos/3d_gizmos.rs 简化）",
        "坐标轴：axes gizmos（gizmos/axes.rs 简化）",
        "变换 Gizmo：TransformGizmo 拖拽（gizmos/transform_gizmo.rs 简化）",
    ],
    "ch15_math": [
        "向量运算：Vec2/Vec3 加减、长度、归一化（基于 math/render_primitives.rs 或 bounding_2d.rs 简化）",
        "样条曲线：CubicSpline 与位置插值（math/cubic_splines.rs 简化）",
        "自定义数学原语：implement Bounded2d（math/custom_primitives.rs 简化）",
        "边界与相交：Aabb / bounding_2d 判定（math/bounding_2d.rs 简化）",
    ],
    "ch16_scene": [
        "BSN 场景基础：bsn_list! 组成简单场景（scene/bsn.rs 简化）",
        "BSN 交互：场景内 button + on 观察者（scene/bsn.rs UI 部分简化）",
        "加载场景：asset_server.load + SceneRoot 生成（scene 相关官方代码 / 3d_scene 简化）",
        "世界序列化：serialize/deserialize 世界（scene/world_serialization.rs 简化）",
    ],
    "ch17_state": [
        "States 基础：init_state + OnEnter/OnExit（state/states.rs 简化）",
        "状态切换：NextState 触发转移（state/states.rs 简化）",
        "计算状态：ComputedStates（state/computed_states.rs 简化）",
        "子状态：SubStates（state/sub_states.rs 简化）",
    ],
    "ch18_time": [
        "Time 与 delta：每帧按时间移动（time/time.rs 或 2d/move_sprite.rs 简化）",
        "Timer 组件：倒计时与重复（time/timers.rs 简化）",
        "Stopwatch 秒表（time/timers.rs 中 stopwatch 部分简化）",
        "虚拟时间：Virtual 时间缩放（time/virtual_time.rs 简化）",
    ],
    "ch19_transform": [
        "Transform 与平移：from_xyz / translation 修改（transforms/transform.rs 与 translation.rs 简化）",
        "旋转：Quat 与 rotate（transforms/3d_rotation.rs 简化）",
        "缩放：scale 与局部缩放（transforms/scale.rs 简化）",
        "朝向：LookAt / align 对齐（transforms/align.rs 简化）",
    ],
    "ch20_input": [
        "键盘：ButtonInput<KeyCode> just_pressed/pressed（input/keyboard_input.rs 简化）",
        "鼠标：ButtonInput<MouseButton> 与 MouseMotion（input/mouse_input.rs 简化）",
        "触摸：Touches 与 TouchInput（input/touch_input.rs 简化）",
        "手柄：Gamepad 按钮与摇杆（input/gamepad_input.rs 简化）",
        "修饰键：KeyModifiers 组合键（input/keyboard_modifiers.rs 简化）",
        "键盘事件：KeyboardInput 消息读取（input/keyboard_input_events.rs 简化）",
    ],
    "ch21_ui": [
        "文本 UI：Text::new + TextFont + Node（ui/text/text.rs 简化，默认字体）",
        "按钮：Button + Interaction 状态切换（ui/widgets/button.rs 简化，去掉 InputFocus）",
        "Flex 布局：FlexDirection / JustifyContent / AlignItems（ui/layout/flex_layout.rs 简化）",
        "锚点定位：PositionType::Absolute + 边距（ui/layout/anchor_layout.rs 简化）",
        "边框与背景：BorderColor / BackgroundColor（ui/styling/borders.rs 简化）",
        "大小约束：Size / MinSize / MaxSize（ui/layout/size_constraints.rs 简化）",
        "计数器组件：feathers_counter 的点击计数（ui/widgets/feathers_counter.rs 简化）",
    ],
    "ch22_window": [
        "窗口设置：标题/分辨率/大小可调（window/window_settings.rs 简化）",
        "窗口调整：运行时改分辨率（window/window_resizing.rs 简化）",
        "多窗口：第二个窗口（window/multiple_windows.rs 简化）",
        "清屏颜色：ClearColor 资源（window/clear_color.rs 简化）",
    ],
    "ch23_movement": [
        "平滑跟随：smooth_follow 插值（movement/smooth_follow.rs 简化）",
        "固定时间步移动：FixedUpdate 下的移动（movement/physics_in_fixed_timestep.rs 简化）",
        "转向鼠标：rotate_to_cursor（2d/rotate_to_cursor.rs 简化）",
    ],
    "ch24_async_tasks": [
        "异步计算：AsyncComputeTaskPool spawn + Task 轮询（async_tasks/async_compute.rs 简化）",
        "异步通道：async_channel 传递结果（async_tasks/async_channel_pattern.rs 简化）",
        "外部线程：external_source 线程传数据（async_tasks/external_source_external_thread.rs 简化）",
    ],
    "ch25_shaders": [
        "自定义材质：ShaderMaterial + AsBindGroup，WGSL 用 const 字符串内联（shader/shader_material.rs 简化，不 include_str 外部文件）",
        "动画着色器：shader_defs / 时间 uniform（shader/animate_shader.rs 简化内联）",
        "2D 着色器材质：shader_material_2d（shader/shader_material_2d.rs 简化内联）",
    ],
    "ch26_gltf": [
        "加载 glTF：asset_server.load 模型（gltf/load_gltf.rs 简化）",
        "场景生成：SceneRoot 组件（gltf/load_gltf.rs 简化）",
        "更新场景：遍历并修改（gltf/update_gltf_scene.rs 简化）",
        "查询图元：遍历网格材质（gltf/query_gltf_primitives.rs 简化）",
    ],
    "ch27_picking": [
        "基础拾取：Pointer<Over> 悬停变色（picking/simple_picking.rs 简化）",
        "网格拾取：mesh_picking 点击高亮（picking/mesh_picking.rs 简化）",
        "Sprite 拾取：sprite_picking（picking/sprite_picking.rs 简化）",
        "拖拽拾取：dragdrop（picking/dragdrop_picking.rs 简化）",
    ],
    "ch28_assets": [
        "资产加载：AssetServer.load + Assets 集合（asset/asset_loading.rs 简化）",
        "热重载：file_watcher（asset/hot_asset_reloading.rs 简化，注意 file_watcher 非默认 feature——若编译不过请改写为普通 load + AssetEvent）",
        "嵌入资产：embedded_asset 宏（asset/embedded_asset.rs 简化）",
        "生成资产：generated_assets（asset/generated_assets.rs 简化）",
        "资产设置：asset_settings（asset/asset_settings.rs 简化）",
    ],
    "ch29_events": [
        "自定义消息：#[derive(Message)] + MessageWriter（ecs/message.rs 简化）",
        "读取消息：MessageReader 与系统顺序（ecs/message.rs 简化）",
        "观察者：add_observer + On<MyEvent> + commands.trigger（ecs/observers.rs 简化）",
        "组件生命周期：On<Add<T>> / On<Remove<T>>（ecs/component_hooks.rs 简化）",
    ],
    "ch30_run_conditions": [
        "run_if 基础：condition 为真才运行（ecs/run_conditions.rs 简化）",
        "条件组合：and/or/not 与资源条件（ecs/run_conditions.rs 简化）",
        "系统管道：pipe 串联（ecs/system_piping.rs 简化）",
    ],
    "ch31_fixed_timestep": [
        "FixedUpdate 基础：固定间隔运行（ecs/fixed_timestep.rs 简化）",
        "Time<Fixed>：读取固定步长时间（ecs/fixed_timestep.rs 简化）",
        "固定步长物理：physics_in_fixed_timestep（movement/physics_in_fixed_timestep.rs 简化）",
    ],
    "ch32_change_detection": [
        "Changed<T>：组件变化才处理（ecs/change_detection.rs 简化）",
        "Added<T> 与 RemovedComponents：组件生命周期事件（ecs/removal_detection.rs 简化）",
        "EntityDisabled：禁用实体（ecs/entity_disabling.rs 简化）",
    ],
    "ch33_hierarchy": [
        "Parent/Children：生成层级并查询（ecs/hierarchy.rs 简化）",
        "父子变换：add_child 后子随父动（3d/parenting.rs 简化）",
        "关系组件：ChildOf / 关系查询（ecs/relationships.rs 简化）",
    ],
    "ch34_ecs_advanced": [
        "一次性系统：OneShotSystem 手动触发（ecs/one_shot_systems.rs 简化）",
        "泛型系统：GenericSystem（ecs/generic_system.rs 简化）",
        "并行查询：多 Query 同时迭代（ecs/parallel_query.rs 简化）",
        "自定义调度：Schedules / ScheduleLabel（ecs/custom_schedule.rs 简化）",
    ],
    "ch35_tips": [
        "冷却计时：cooldown 模式（usage/cooldown.rs 简化）",
        "日志分层：log_layers 输出（app/log_layers.rs 简化）",
        "无 winit：不依赖窗口的 App（app/without_winit.rs 简化）",
    ],
}

AGENTS = [
    ("A", ["ch01_hello_engine", "ch02_apps", "ch04_plugins"]),
    ("B", ["ch03_ecs", "ch05_resources", "ch18_time"]),
    ("C", ["ch06_breakout", "ch13_games"]),
    ("D", ["ch07_2d_basics", "ch23_movement"]),
    ("E", ["ch08_3d_basics", "ch19_transform"]),
    ("F", ["ch09_animation", "ch14_gizmos", "ch15_math"]),
    ("G", ["ch10_audio", "ch11_camera", "ch12_diagnostics"]),
    ("H", ["ch16_scene", "ch17_state", "ch31_fixed_timestep"]),
    ("I", ["ch20_input", "ch22_window"]),
    ("J", ["ch21_ui", "ch29_events"]),
    ("K", ["ch24_async_tasks", "ch25_shaders", "ch26_gltf"]),
    ("L", ["ch27_picking", "ch28_assets", "ch32_change_detection"]),
    ("M", ["ch30_run_conditions", "ch33_hierarchy", "ch34_ecs_advanced", "ch35_tips"]),
]

os.makedirs(os.path.join(ROOT, "_tasks"), exist_ok=True)
for letter, slugs in AGENTS:
    total = 0
    parts = [PREAMBLE]
    for slug in slugs:
        ch = chapters[slug]
        topics = TOPICS[slug]
        assert len(topics) == ch["count"], (slug, len(topics), ch["count"])
        total += ch["count"]
        srcs = "\n".join(f"  - `{s}`" for s in ch["sources"])
        ex_topics = "\n".join(
            f"  - exercise_{i+1:02d}（{eid}）：{t}" for i, (eid, t) in enumerate(zip(ch["exercises"], topics))
        )
        parts.append(f"""
## 负责章节：{ch['num']:02d} {ch['slug']} —— {ch['title']}（共 {ch['count']} 个练习，ID: {', '.join(ch['exercises'])}）

本章重点：{ch['focus']}

### 本章官方源码（必须阅读）
{srcs}

### 本章练习安排
{ex_topics}
""")
    parts.append(f"""
## 交付检查清单（完成后逐项自查并在回复中报告）
- [ ] {len(slugs)} 章、{total} 个练习全部完成（每章数量正确）
- [ ] 每个练习都有 exercise 与 solution 两个文件，文件名正确（exercise_01.rs ...）
- [ ] 每个练习有出处 URL、中文讲解、任务说明、`// I AM NOT DONE`、恰一处 `// BUG:`、底部提示、`#[cfg(test)] mod tests`（≥2 断言）
- [ ] solution 是修好 bug 的版本，测试与 exercise 一致
- [ ] 无第三方 crate、无非默认 feature、测试不依赖外部文件
- [ ] 每章 bug 类型混合（编译错误 + 逻辑错误）
""")
    path = os.path.join(ROOT, f"_tasks/agent_{letter}.md")
    io.open(path, "w", encoding="utf-8", newline="\n").write("\n".join(parts))
    print(f"agent_{letter}.md: {total} exercises")
