//! # 练习 31.02 —— Time<Fixed>：读取固定步长时间
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/fixed_timestep/
//!
//! ## 概念
//! 在 `FixedUpdate` 里，普通的 `Res<Time>` 拿到的就是固定步长时间，
//! 它的 `delta_secs()` 恒等于固定步长（比如 0.5 秒）。
//! 但有些信息只有 `Time<Fixed>` 这个类型才提供：
//! - `overstep()`：为了凑满步长，当前"积压"了多少时间；
//! - `overstep_fraction()`：积压时间占步长的比例（0~1），
//!   做插值渲染时经常用到。
//!
//! 本练习的 `fixed_update` 想打印积压比例，但系统参数写错了类型。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3102` 查看现象，改正后运行 `bevylings test 3102` 让测试通过。
//!
//! 小贴士：`accumulate_overstep` 是 Bevy 专门留给测试用的方法，
//! 可以手动往积压时间里加值。

// I AM NOT DONE

use bevy::prelude::*;

/// 每跑一次固定步长，打印步长和积压比例。
fn fixed_update(time: Res<Time>) {
    // BUG: 普通的 Time 没有 overstep 系列方法，
    // 想读积压时间必须显式声明 Res<Time<Fixed>>。
    let fraction = time.overstep_fraction();
    info!("fixed timestep: {}", time.delta_secs());
    info!("overstep fraction: {fraction}");
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_systems(FixedUpdate, fixed_update)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn overstep_fraction_is_overstep_over_step() {
        let mut fixed_time = Time::<Fixed>::from_seconds(0.5);
        fixed_time.accumulate_overstep(Duration::from_millis(250));
        assert!(
            (fixed_time.overstep_fraction() - 0.5).abs() < 1e-6,
            "积压 0.25 秒占步长 0.5 秒的一半"
        );
    }

    #[test]
    fn no_overstep_means_zero_fraction() {
        let fixed_time = Time::<Fixed>::from_seconds(0.5);
        assert_eq!(fixed_time.overstep_fraction(), 0.0);
    }

    #[test]
    fn overstep_reports_accrued_time() {
        let mut fixed_time = Time::<Fixed>::from_seconds(0.5);
        fixed_time.accumulate_overstep(Duration::from_millis(250));
        assert_eq!(fixed_time.overstep(), Duration::from_millis(250));
    }
}

// 提示：
// 1. 先运行 `bevylings run 3102`，编译器会说 Time 没有 overstep_fraction 方法。
// 2. 想一想：overstep 是"固定时间"特有的信息，系统参数要用 `Res<Time<Fixed>>`。
// 3. 改好后运行 `bevylings test 3102`，三个测试全绿就过关。
