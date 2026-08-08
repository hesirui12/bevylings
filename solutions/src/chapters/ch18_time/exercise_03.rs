//! # 练习 18.03 —— Stopwatch：秒表
//!
//! 出处：https://bevy.org/examples-webgpu/time/timers/
//!
//! ## 概念
//! `Stopwatch` 是秒表：只计"已经过去多久"，永远不"结束"。
//! 每帧 `stopwatch.tick(time.delta())` 累加时间，用 `elapsed_secs()` 读出秒数。
//! 它还能 `pause()` 暂停 / `unpause()` 继续 —— 暂停时 tick 不会累计时间。
//!
//! 本练习做一个秒表，每帧读出"已进行时间"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1803` 查看现象（目前编译会报错），
//! 改正后运行 `bevylings test 1803` 让测试通过。
//!
//! 小贴士：`elapsed_secs()` 返回 f32（秒数）；`elapsed()` 返回 Duration（时间段），不能当 f32 用。

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::Stopwatch;

/// 给秒表走时间，返回已过去的秒数。
fn seconds_elapsed(stopwatch: &mut Stopwatch, delta: Duration) -> f32 {
    stopwatch.tick(delta);
    stopwatch.elapsed_secs()
}

/// 秒表：挂在实体上的计时组件。
#[derive(Component)]
struct GameClock(Stopwatch);

/// 每帧推进秒表。
fn tick_clock(mut query: Query<&mut GameClock>, time: Res<Time>) {
    for mut clock in &mut query {
        let secs = seconds_elapsed(&mut clock.0, time.delta());
        println!("游戏已进行 {secs:.1} 秒");
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, tick_clock)
        .run();
}

/// 生成一个秒表实体。
fn setup(mut commands: Commands) {
    commands.spawn(GameClock(Stopwatch::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stopwatch_accumulates_time() {
        let mut stopwatch = Stopwatch::new();
        assert_eq!(
            seconds_elapsed(&mut stopwatch, Duration::from_secs_f32(1.5)),
            1.5
        );
        assert_eq!(
            seconds_elapsed(&mut stopwatch, Duration::from_secs_f32(0.5)),
            2.0
        );
    }

    #[test]
    fn paused_stopwatch_does_not_advance() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.pause();
        stopwatch.tick(Duration::from_secs_f32(1.5));
        assert!(stopwatch.is_paused());
        assert_eq!(stopwatch.elapsed_secs(), 0.0, "暂停时 tick 不累计时间");
    }
}

// 提示：
// 1. 先运行 `bevylings run 1803`，看编译错误里"expected `f32`, found `Duration`"。
// 2. `elapsed()` 给"时长对象"，`elapsed_secs()` 给"秒数（f32）"。
// 3. 换成 `elapsed_secs()` 再运行 `bevylings test 1803`。
