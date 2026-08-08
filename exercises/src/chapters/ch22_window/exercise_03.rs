//! # 练习 22.03 —— 多窗口：第二个窗口
//!
//! 出处：https://bevy.org/examples-webgpu/window/multiple-windows/
//!
//! ## 概念
//! Bevy 默认只创建一个"主窗口"。想要第二个窗口，直接像生成普通实体一样
//! `commands.spawn(Window { ... })` 就行——`Window` 本身就是一个组件。
//!
//! 光有窗口还不够，还得告诉某个相机"你负责渲染到这个窗口"，
//! 这是通过相机上的 `RenderTarget` 组件指定的：
//! `RenderTarget::Window(WindowRef::Entity(窗口实体))`。
//! 也就是说：先 spawn 窗口拿到它的实体 id，再把它"指"给相机。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2203` 查看现象（目前编译会报错），
//! 改正后运行 `bevylings test 2203` 让测试通过。
//!
//! 小贴士：`WindowRef` 是一个枚举：`Primary` 指主窗口，`Entity(e)` 指某个具体窗口实体。

// I AM NOT DONE

use bevy::{
    camera::RenderTarget,
    prelude::*,
    window::WindowRef,
};

/// 记住第二个窗口的实体 id，方便测试检查。
#[derive(Resource)]
struct SecondWindow(Entity);

/// 启动时：生成主窗口、第二个窗口，以及分别对着它们的两个相机。
fn setup_scene(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 生成第二个窗口，并拿到它的实体 id
    let second_window = commands
        .spawn(Window {
            title: "Second window".to_owned(),
            ..default()
        })
        .id();

    // 第二个相机的渲染目标是"第二个窗口"
    commands.spawn((
        Camera3d::default(),
        // BUG: WindowRef::Entity 的参数类型是 Entity，这里却把窗口实体
        // 直接塞给了 WindowRef，类型对不上，编译失败。
        // 要用 WindowRef::Entity(窗口实体) 包一层。
        RenderTarget::Window(second_window),
    ));

    commands.insert_resource(SecondWindow(second_window));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.add_systems(Startup, setup_scene);
        app
    }

    #[test]
    fn second_window_is_spawned() {
        let mut app = build_app();
        app.update();
        let mut windows = app.world_mut().query::<&Window>();
        let titles: Vec<String> = windows.iter(app.world()).map(|w| w.title.clone()).collect();
        assert!(
            titles.iter().any(|t| t == "Second window"),
            "应该有第二个窗口，实际: {titles:?}"
        );
    }

    #[test]
    fn second_camera_renders_to_second_window() {
        let mut app = build_app();
        app.update();
        let second = app.world().resource::<SecondWindow>().0;
        let mut targets = app.world_mut().query::<&RenderTarget>();
        let hit = targets.iter(app.world()).any(|rt| {
            matches!(rt, RenderTarget::Window(WindowRef::Entity(e)) if *e == second)
        });
        assert!(hit, "应该有相机的渲染目标指向第二个窗口");
    }
}

// 提示：
// 1. 先看 `// BUG:` 那一行：`RenderTarget::Window` 的参数类型是什么？
// 2. `WindowRef::Entity(e)` 把实体 id 包成"某个窗口"的引用。
// 3. 修改后运行 `bevylings test 2203`，两个测试都通过就过关了。
