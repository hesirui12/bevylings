//! # 练习 09.01 —— 缓动运动（EasingCurve）
//!
//! 出处：https://bevy.org/examples-webgpu/animation/eased-motion/
//!
//! ## 概念
//! 一段动画本质上是"值随时间变化"。最简单的是匀速变化，但真实运动
//! 很少匀速：物体启动要加速、停下要减速，这种"变速的节奏"由
//! **缓动函数（easing）** 控制。
//!
//! Bevy 的 `EasingCurve::new(起点, 终点, 缓动函数)` 会生成一条曲线：
//! 用 `sample(t)` 就能拿到进度 t（0 到 1）时的位置。
//! `EaseFunction::CubicInOut` 让运动"先慢、中间快、再慢"，
//! 是演示缓动的经典选择。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0901` 观察现象，改正后运行 `bevylings test 0901` 让测试通过。
//!
//! 小贴士：`EasingCurve::new` 的第一个参数是起点、第二个是终点，
//! 注意别写反。

use bevy::prelude::*;

/// 把进度 t（0 到 1）映射成方块位置：
/// 从 (0,0,0) 出发，用 CubicInOut 缓动滑到 (10,0,0)。
fn eased_position(t: f32) -> Vec3 {
    let curve = EasingCurve::new(
        Vec3::ZERO,
        Vec3::new(10.0, 0.0, 0.0),
        EaseFunction::CubicInOut,
    );
    curve.sample(t).expect("t 必须在 0..=1 之间")
}

/// 演示用的方块。
#[derive(Component)]
struct Mover;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((Mover, Transform::from_xyz(-6.0, 2.0, 0.0)));
}

fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<Mover>>) {
    // 进度 t 在 0..1 之间来回循环
    let t = time.elapsed_secs().rem_euclid(2.0) / 2.0;
    for mut transform in &mut query {
        transform.translation = eased_position(t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_origin() {
        assert_eq!(eased_position(0.0), Vec3::ZERO, "t=0 应该在起点 (0,0,0)");
    }

    #[test]
    fn ends_at_target() {
        assert_eq!(
            eased_position(1.0),
            Vec3::new(10.0, 0.0, 0.0),
            "t=1 应该在终点 (10,0,0)"
        );
    }

    #[test]
    fn midpoint_is_between() {
        let mid = eased_position(0.5);
        assert!(
            mid.x > 0.0 && mid.x < 10.0,
            "t=0.5 的位置应在起点终点之间，实际 {mid:?}"
        );
    }
}

// 提示：
// 1. 先看 `eased_position` 里 EasingCurve::new 的两个端点，思考"从 A 到 B"谁在前。
// 2. 曲线在 t=0 时给出起点、t=1 时给出终点，测试正是检查这两个端点。
// 3. 修改后运行 `bevylings test 0901`，测试全绿就算过关。
