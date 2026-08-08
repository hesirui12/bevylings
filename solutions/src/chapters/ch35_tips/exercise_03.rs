//! # 练习 35.03 —— 无 winit：不需要窗口的 App
//!
//! 出处：https://bevy.org/examples-webgpu/app/without-winit/
//!
//! ## 概念
//! 不是所有 Bevy 程序都需要窗口。官方示例在 `DefaultPlugins` 里
//! **禁用掉 `WinitPlugin`**（负责创建窗口和事件循环的那个插件）：
//! ```text
//! DefaultPlugins.build().disable::<WinitPlugin>()
//! ```
//! 这样 App 就没有窗口事件循环，主循环**跑一遍就自动退出**，适合做
//! 服务器逻辑、命令行工具、批量渲染。有窗口的 App 则会一直循环到用户关窗。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3503` 查看现象，改正后运行 `bevylings test 3503` 让测试通过。
//!
//! 小贴士：`frames_before_exit(false)` 表示"没有窗口时主循环跑几帧"，
//! 无窗口应该只跑 1 帧就退出。

use bevy::{prelude::*, winit::WinitPlugin};

/// 记录跑过的帧数。
#[derive(Resource, Default)]
struct FrameCounter(u32);

/// 无窗口的 App 没有事件循环，主循环"跑一遍就退出"；
/// 有窗口的 App 会一直循环。返回主循环会执行的帧数。
fn frames_before_exit(has_window: bool) -> u32 {
    if has_window {
        u32::MAX
    } else {
        1
    }
}

/// 每帧给帧数加 1（配合"跑一遍就退出"观察只执行了一次）。
fn count_frame(mut frames: ResMut<FrameCounter>) {
    frames.0 += 1;
}

pub fn run() {
    // 打印一下"退出策略"，演示无窗口模式只跑一遍。
    if frames_before_exit(false) == 1 {
        info!("headless mode: run the main schedule once and exit");
    }
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
        .init_resource::<FrameCounter>()
        .add_systems(Update, count_frame)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headless_app_quits_after_one_frame() {
        assert_eq!(frames_before_exit(false), 1, "无窗口时只跑一帧就退出");
        assert_eq!(frames_before_exit(true), u32::MAX, "有窗口时一直循环");
    }

    #[test]
    fn headless_runs_fewer_frames() {
        assert!(
            frames_before_exit(false) < frames_before_exit(true),
            "无窗口的帧数应该远小于有窗口的帧数"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings test 3503`，看"无窗口只跑一帧"的断言怎么失败。
// 2. 有窗口才无限循环（u32::MAX），无窗口只跑 1 帧。
// 3. 把 `frames_before_exit` 的两个分支换回来就过关了。
