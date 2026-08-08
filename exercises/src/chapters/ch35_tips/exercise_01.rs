//! # 练习 35.01 —— 冷却计时：技能用完要等一等
//!
//! 出处：https://bevy.org/examples-webgpu/usage/cooldown/
//!
//! ## 概念
//! 很多游戏动作（吃食物、放技能）不能连续触发，中间要等一段**冷却时间**。
//! Bevy 的 `Timer` 专门干这个：`Timer::from_seconds(秒数, TimerMode::Once)`
//! 创建一次性计时器；每帧用 `timer.tick(time.delta())` 推进；
//! `timer.finished()` 判断是否走完了；走完后再用 `timer.reset()` 重新开始。
//!
//! 官方示例把它做成了 UI 上的按钮，本练习只保留核心逻辑：
//! 一个 `can_use` 函数判断"冷却结束了没有"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3501` 查看现象，改正后运行 `bevylings test 3501` 让测试通过。
//!
//! 小贴士：`finished` 是**方法**，调用时要带括号 `finished()`；不带括号会被当成字段。

// I AM NOT DONE

use std::time::Duration;

use bevy::prelude::*;

/// 冷却计时器（挂在使用冷却的实体上）。
#[derive(Component)]
struct Cooldown(Timer);

/// 记录"吃过东西"的事件，方便测试观察。
#[derive(Resource, Default)]
struct EatLog(Vec<String>);

/// 冷却结束了吗？结束后才能再次使用。
fn can_use(timer: &Timer) -> bool {
    // BUG: `finished` 是方法不是字段，直接写 `timer.finished` 会被编译器当成
    // "取值"，报错 attempted to take value of method。要加上括号调用它。
    timer.finished
}

/// 每帧推进所有冷却计时器。
fn tick_cooldowns(time: Res<Time>, mut query: Query<&mut Cooldown>) {
    for mut cooldown in &mut query {
        cooldown.0.tick(time.delta());
    }
}

/// 冷却好的实体可以再吃一次（吃完重新计时）。
fn try_eat(mut query: Query<&mut Cooldown>, mut log: ResMut<EatLog>) {
    for mut cooldown in &mut query {
        if can_use(&cooldown.0) {
            cooldown.0.reset();
            log.0.push("ate!".to_string());
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins)
        .init_resource::<EatLog>()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Cooldown(Timer::from_seconds(1.0, TimerMode::Once)));
        })
        .add_systems(Update, (tick_cooldowns, try_eat))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooldown_blocks_until_finished() {
        let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
        assert!(!can_use(&timer), "刚开始冷却，还没结束");
        timer.tick(Duration::from_secs(1));
        assert!(can_use(&timer), "1 秒后冷却结束，可以再用了");
    }

    #[test]
    fn reset_restarts_cooldown() {
        let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
        timer.tick(Duration::from_secs(3));
        assert!(can_use(&timer), "2 秒冷却早就走完了");
        timer.reset();
        assert!(!can_use(&timer), "重置后又要重新等 2 秒");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3501`，看编译报错的内容。
// 2. 方法是"函数"，调用要写括号：`timer.finished()`。
// 3. 改完后两个测试都绿就过关。
