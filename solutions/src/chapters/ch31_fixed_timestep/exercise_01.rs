//! # 练习 31.01 —— FixedUpdate 基础：固定间隔运行
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/fixed_timestep/
//!
//! ## 概念
//! `Update` 里的系统每帧都跑，但每帧的耗时并不固定。
//! `FixedUpdate` 则**每隔固定时长运行一次**（比如每 0.5 秒一次），
//! 与帧率无关：这一帧流逝了 1 秒，它就"追赶"着连跑 2 次。
//! 官方示例用 `Time::<Fixed>::from_seconds(0.5)` 设置步长。
//!
//! 本练习用纯函数 `fixed_steps_needed` 模拟这个"追赶循环"：
//! 给定已流逝的时间和固定步长，算一算应该跑几次。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3101` 查看现象，改正后运行 `bevylings test 3101` 让测试通过。
//!
//! 小贴士：1.2 秒 ÷ 0.5 秒步长 = 2.4，只会跑满 2 次，第 3 次时间不够。

use bevy::prelude::*;

/// 固定步长：每 0.5 秒运行一次。
const FIXED_STEP_SECS: f64 = 0.5;

/// 模拟追赶循环：给定已流逝时间，计算 FixedUpdate 应该运行几次。
fn fixed_steps_needed(elapsed_secs: f64, fixed_secs: f64) -> u32 {
    (elapsed_secs / fixed_secs).floor() as u32
}

/// 每帧打印"按固定步长应该推进几次"。
fn frame_update(time: Res<Time>, fixed_time: Res<Time<Fixed>>) {
    let steps = fixed_steps_needed(time.elapsed_secs() as f64, fixed_time.timestep().as_secs_f64());
    info!(
        "已流逝 {} 秒，固定步长应推进 {} 次",
        time.elapsed_secs(),
        steps
    );
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_seconds(FIXED_STEP_SECS))
        .add_systems(Update, frame_update)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_step_does_not_run() {
        assert_eq!(fixed_steps_needed(0.4, 0.5), 0, "0.4 秒不够 0.5 秒，一次都不跑");
        assert_eq!(fixed_steps_needed(0.5, 0.5), 1, "正好一个步长");
    }

    #[test]
    fn full_second_runs_twice_at_half_second_steps() {
        assert_eq!(fixed_steps_needed(1.0, 0.5), 2);
        assert_eq!(fixed_steps_needed(1.2, 0.5), 2, "2.4 个步长只跑满 2 次");
    }

    #[test]
    fn smaller_steps_run_more_often() {
        let coarse = fixed_steps_needed(1.0, 0.5);
        let fine = fixed_steps_needed(1.0, 0.25);
        assert!(fine > coarse, "步长越小，同样时间跑的次数越多");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3101`，看第一个测试为什么失败。
// 2. "跑满几次"要用向下取整：1.2 / 0.5 = 2.4 -> 2 次。
// 3. 把取整方向改对后运行 `bevylings test 3101`，三个测试全绿就过关。
