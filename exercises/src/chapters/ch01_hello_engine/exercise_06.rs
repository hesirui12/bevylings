//! # 练习 01.06 —— 用 AppExit 让程序退出
//!
//! 出处：https://bevy.org/examples-webgpu/application/return_after_run/
//!
//! ## 概念
//! 游戏循环默认会一直转下去，但我们可以向 Bevy 发送一条"退出消息"
//! （`AppExit`）来结束循环：发送后 `run()` 会返回，程序正常结束。
//! 在 Bevy 0.19 中，事件（Event）改名为消息（Message）：
//! 用 `MessageWriter<T>` 写入，用 `.write(...)` 发送。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0106` 观察现象，改正后运行 `bevylings test 0106` 让测试通过。
//!
//! 小贴士：`AppExit::Success` 表示"正常退出"。

// I AM NOT DONE

use bevy::prelude::*;

/// 最多运行几帧？
const MAX_FRAMES: u32 = 5;

/// 记录已经跑过的帧数。
#[derive(Resource, Default)]
struct Frames(u32);

/// 判断"跑满 MAX_FRAMES 帧"后是否该退出。
fn should_exit_after(frames: u32) -> bool {
    // BUG: 比较方向写反了：现在返回的是"还没跑满"的结果，
    // 而调用方需要的是"已经跑满 MAX_FRAMES 帧"的结果。
    frames < MAX_FRAMES
}

/// 每帧计数；一旦满足条件就发送 AppExit，结束游戏循环。
fn check_exit(mut frames: ResMut<Frames>, mut exit: MessageWriter<AppExit>) {
    frames.0 += 1;
    if should_exit_after(frames.0) {
        exit.write(AppExit::Success);
    }
}

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(Frames::default())
        .add_systems(Update, check_exit)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exits_only_after_max_frames() {
        assert!(!should_exit_after(1), "第 1 帧不应该退出");
        assert!(!should_exit_after(4), "还没跑满 5 帧，不应该退出");
        assert!(should_exit_after(5), "跑满 5 帧，应该退出");
        assert!(should_exit_after(8), "超过 5 帧，也应该退出");
    }

    #[test]
    fn app_sends_exit_when_frames_are_up() {
        let mut app = App::new();
        app.insert_resource(Frames::default());
        app.add_systems(Update, check_exit);
        for _ in 0..4 {
            app.update();
            assert!(app.should_exit().is_none(), "前 4 帧不应该退出");
        }
        app.update();
        assert_eq!(
            app.should_exit(),
            Some(AppExit::Success),
            "第 5 帧应该发送正常退出消息"
        );
    }
}

// 提示：
// 1. "跑满 5 帧就退出"用 `frames < MAX_FRAMES` 来表达对吗？
//    试一试用它判断第 1 帧，会发现结果正好相反。
// 2. 想一想 `>=` 和 `<=`，哪个才是"已经跑满"。
// 3. 修改后运行 `bevylings test 0106`，两个测试都通过就过关了。
