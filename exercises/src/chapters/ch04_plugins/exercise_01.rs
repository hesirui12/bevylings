//! # 练习 04.01 —— 自定义插件（Plugin trait）
//!
//! 出处：https://bevy.org/examples-webgpu/application/plugin/
//!
//! ## 概念
//! 插件是 Bevy 里组织代码的基本单位：任何"给 App 添加东西"的逻辑都可以
//! 打包成一个插件。实现插件只需三步：
//! 1. 定义一个 struct（可以带配置字段）；
//! 2. 为它实现 `Plugin` trait，在 `build` 里添加系统、资源等；
//! 3. 用 `add_plugins` 把它注册进 App。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0401` 观察现象，改正后运行 `bevylings test 0401` 让测试通过。
//!
//! 小贴士：`Startup` 只跑一次，`Update` 每帧都跑——注册到哪个调度，
//! 决定系统的运行时机。

// I AM NOT DONE

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

/// 统计问候系统运行了多少次。
#[derive(Resource, Default)]
struct Count(u32);

/// 每帧运行一次的问候系统。
fn say_hello(mut count: ResMut<Count>) {
    count.0 += 1;
    println!("hello!");
}

/// 我们的第一个自定义插件。
struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Count::default());
        // BUG: 系统被注册进了错误的调度：现在它只在启动时运行一次，
        // 但我们希望它每一帧都运行。
        app.add_systems(Startup, say_hello);
    }
}

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_plugins(HelloPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins(HelloPlugin);
        app
    }

    #[test]
    fn hello_runs_every_frame() {
        let mut app = build_app();
        app.update();
        app.update();
        app.update();
        let count = app.world().resource::<Count>();
        assert_eq!(count.0, 3, "Update 系统每帧都应该运行，3 帧后计数应为 3");
    }

    #[test]
    fn plugin_inserts_count_resource() {
        let mut app = build_app();
        app.update();
        assert!(
            app.world().get_resource::<Count>().is_some(),
            "插件应该在 build 时插入 Count 资源"
        );
    }
}

// 提示：
// 1. 运行测试，看看跑 3 帧后计数是多少。如果只有 1，说明系统只跑了一次。
// 2. "每帧都运行"应该注册到哪个调度？`Startup` 还是 `Update`？
// 3. 修改后运行 `bevylings test 0401`，两个测试都通过就过关了。
