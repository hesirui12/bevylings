//! # 练习 02.01 —— 无窗口运行的 App（Headless App）
//!
//! 出处：https://bevy.org/examples-webgpu/application/headless/
//!
//! ## 概念
//! 不是所有 Bevy 程序都需要窗口和渲染。Bevy 的 `ScheduleRunnerPlugin`
//! 可以在没有图形界面的情况下驱动游戏循环，非常适合做服务器逻辑、
//! 命令行工具和单元测试。
//! `run_once()` 表示"只跑一帧就退出"。
//!
//! 为了让"帧数"可被测试观察，我们用资源 `FrameCounter` 记录已经跑过的帧数，
//! 并用一个纯函数 `should_print` 决定"这一帧要不要打印计数"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0201` 观察现象，改正后运行 `bevylings test 0201` 让测试通过。
//!
//! 小贴士：`MinimalPlugins` 是"最小插件组"，只包含时间、任务池等基础功能。

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

/// 记录已经跑过的帧数（一个全局计数器）。
#[derive(Resource, Default)]
struct FrameCounter(u32);

/// 每跑 5 帧打印一次计数。
fn counter(mut counter: ResMut<FrameCounter>) {
    counter.0 += 1;
    if should_print(counter.0) {
        println!("frame {}", counter.0);
    }
}

/// 什么时候该打印？
fn should_print(count: u32) -> bool {
    count % 5 == 0
}

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .insert_resource(FrameCounter::default())
        .add_systems(Update, counter)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_every_five_frames() {
        assert!(should_print(5), "第 5 帧应该打印");
        assert!(should_print(10), "第 10 帧应该打印");
        assert!(!should_print(7), "第 7 帧不应该打印");
    }

    #[test]
    fn counter_increments_on_each_frame() {
        let mut app = App::new();
        app.insert_resource(FrameCounter::default());
        app.add_systems(Update, counter);
        app.update();
        app.update();
        app.update();
        let counter = app.world().resource::<FrameCounter>();
        assert_eq!(counter.0, 3, "跑了 3 帧，计数应该是 3");
    }
}

// 提示：
// 1. 先运行 `bevylings run 0201`，看看它打印了几次。
// 2. 想一想：5 和 60 之间是什么关系？"每 5 帧打印一次"应该写成什么？
// 3. 修改后运行 `bevylings test 0201`，测试全绿就算过关。
