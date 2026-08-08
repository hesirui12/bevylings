//! # 练习 18.02 —— Timer：倒计时与重复
//!
//! 出处：https://bevy.org/examples-webgpu/time/timers/
//!
//! ## 概念
//! `Timer` 是一个倒计时器：每帧用 `timer.tick(time.delta())` 让它走时间。
//! `just_finished()` 问"这一帧刚刚到点了吗"（只有到点的那一帧是 true）。
//! `TimerMode::Once` 只响一次；`TimerMode::Repeating` 到点后自动归零重新计时。
//!
//! 本练习做一个"闹钟"：每 2 秒响一次铃。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1802` 查看现象，改正后运行 `bevylings test 1802` 让测试通过。
//!
//! 小贴士：闹钟要响很多次，必须是 `Repeating`（循环）模式。

// I AM NOT DONE

use std::time::Duration;

use bevy::prelude::*;

/// 创建一个 2 秒响一次的闹钟计时器。
fn make_alarm() -> Timer {
    // BUG: 闹钟应该每隔 2 秒响一次（循环计时），
    // 这里却设成了 TimerMode::Once —— 只响一次就不再循环了。
    Timer::from_seconds(2.0, TimerMode::Once)
}

/// 给计时器走时间；刚走完一圈（just_finished）返回 true。
fn ring(timer: &mut Timer, delta: Duration) -> bool {
    timer.tick(delta).just_finished()
}

/// 闹钟：一个挂在实体上的计时器组件。
#[derive(Component)]
struct Alarm(Timer);

/// 每帧给闹钟计时，到点就响。
fn tick_alarms(mut query: Query<&mut Alarm>, time: Res<Time>) {
    for mut alarm in &mut query {
        if ring(&mut alarm.0, time.delta()) {
            println!("BOOM！闹钟响了！");
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, tick_alarms)
        .run();
}

/// 生成一个闹钟实体。
fn setup(mut commands: Commands) {
    commands.spawn(Alarm(make_alarm()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeating_alarm_rings_again() {
        let mut timer = make_alarm();
        assert!(ring(&mut timer, Duration::from_secs(2)), "第一次到点应该响");
        assert!(
            ring(&mut timer, Duration::from_secs(2)),
            "重复模式第二次到点还要响"
        );
    }

    #[test]
    fn alarm_rings_only_at_the_moment() {
        let mut timer = make_alarm();
        assert!(!ring(&mut timer, Duration::from_secs(1)), "还没到点不响");
        assert!(ring(&mut timer, Duration::from_secs(1)), "满 2 秒的这一刻响");
        assert!(
            !ring(&mut timer, Duration::from_secs(0)),
            "到点后的下一帧不再响"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 1802`，看看闹钟响了几次就停了。
// 2. 对比 `TimerMode::Once`（响一次）和 `TimerMode::Repeating`（循环响）。
// 3. 改成 `Repeating` 再运行 `bevylings test 1802`。
