//! # 练习 01.05 —— 无窗口运行：MinimalPlugins 与 run_once
//!
//! 出处：https://bevy.org/examples-webgpu/application/headless/
//!
//! ## 概念
//! 不是所有 Bevy 程序都需要窗口和渲染。`MinimalPlugins` 是一组最精简的插件，
//! 只提供任务池、时间、帧计数和调度循环，非常适合服务器、命令行工具。
//! 其中 `ScheduleRunnerPlugin` 负责驱动游戏循环：默认无限循环，
//! 用 `run_once()` 可以改成"只跑一帧就退出"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0105` 观察现象，改正后运行 `bevylings test 0105` 让测试通过。
//!
//! 小贴士：系统名是一个"函数"，注册给 add_systems 时不要在后面加括号调用它。

// I AM NOT DONE

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

/// 记录已经跑过的帧数。
#[derive(Resource, Default)]
struct FrameCounter(u32);

/// 每帧把帧数加 1。
fn count_frame(mut frames: ResMut<FrameCounter>) {
    frames.0 += 1;
}

/// 每跑一帧打印一次问候语。
fn hello_world_system() {
    println!("hello world");
}

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .insert_resource(FrameCounter::default())
        // BUG: 这一行把系统函数"调用"了（多了括号），
        // 传进去的就不再是函数本身，编译器会报错。
        .add_systems(Update, (hello_world_system(), count_frame))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_increment_on_each_update() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(FrameCounter::default());
        app.add_systems(Update, count_frame);
        app.update();
        app.update();
        let frames = app.world().resource::<FrameCounter>();
        assert_eq!(frames.0, 2, "跑了 2 帧，计数应该是 2");
    }

    #[test]
    fn run_once_plugin_runs_single_frame() {
        let plugin = ScheduleRunnerPlugin::run_once();
        assert!(
            matches!(plugin.run_mode, bevy::app::RunMode::Once),
            "run_once() 应该把运行模式设置为 Once（只跑一帧）"
        );
    }
}

// 提示：
// 1. 报错大概长这样：expected function, found `()`。
//    这说明 add_systems 收到的是一个"函数调用的结果"，而不是"函数本身"。
// 2. 去掉 hello_world_system 后面的括号，让它保持"函数"的身份。
// 3. 修改后运行 `bevylings test 0105`，两个测试都通过就过关了。
