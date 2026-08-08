//! # 练习 01.02 —— 添加系统与 Startup（Startup Schedule）
//!
//! 出处：https://bevy.org/examples-webgpu/application/empty_defaults/
//!
//! ## 概念
//! 系统（System）就是普通的 Rust 函数，Bevy 会在固定的时机自动调用它们。
//! - 注册进 `Startup` 调度的系统：程序启动时只运行一次。
//! - 注册进 `Update` 调度的系统：每一帧都会运行。
//! 用 `add_systems` 把系统注册进某个调度，一个调度里可以放很多系统。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0102` 观察现象，改正后运行 `bevylings test 0102` 让测试通过。
//!
//! 小贴士：`DefaultPlugins` 会打开一个窗口。窗口出现，说明程序真的在运行了。

use bevy::prelude::*;

/// 记录每个调度各跑了多少次，方便测试观察。
#[derive(Resource, Default)]
struct RunCount {
    startup: u32,
    update: u32,
}

/// Startup 系统：只在启动时运行一次。
fn startup_system(mut count: ResMut<RunCount>) {
    count.startup += 1;
    println!("startup system ran");
}

/// Update 系统：每一帧都运行。
fn update_system(mut count: ResMut<RunCount>) {
    count.update += 1;
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RunCount::default())
        .add_systems(Startup, startup_system)
        .add_systems(Update, update_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(RunCount::default());
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, update_system);
        app
    }

    #[test]
    fn startup_runs_exactly_once() {
        let mut app = build_app();
        app.update();
        app.update();
        app.update();
        let count = app.world().resource::<RunCount>();
        assert_eq!(count.startup, 1, "Startup 系统只应该在启动时运行一次");
    }

    #[test]
    fn update_runs_every_frame() {
        let mut app = build_app();
        app.update();
        app.update();
        app.update();
        let count = app.world().resource::<RunCount>();
        assert_eq!(count.update, 3, "Update 系统每一帧都应该运行");
    }
}

// 提示：
// 1. 先运行 `bevylings run 0102` 确认现在编译不过。
// 2. 报错说的是"no method named ..."。Bevy 注册系统的方法是复数形式，
//    和它相比，现在这行少了什么？
// 3. 修改后运行 `bevylings test 0102`，两个测试都通过就过关了。
