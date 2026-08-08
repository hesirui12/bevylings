//! # 练习 31.03 —— 固定步长物理与插值渲染
//!
//! 出处：https://bevy.org/examples-webgpu/movement/physics_in_fixed_timestep/
//!
//! ## 概念
//! 物理模拟放在 `FixedUpdate` 里推进，步长恒定，物理才稳定。
//! 但画面每帧渲染，可能这一帧物理没跑、下一帧跑了两次，
//! 位置就会"一顿一顿"。官方示例的解决办法是**插值**：
//! 把上一次物理位置（previous）和当前物理位置（current）按
//! `alpha = fixed_time.overstep_fraction()` 线性混合，
//! 渲染用的 Transform 就平滑地夹在两者之间。
//!
//! 本练习精简为两件事：
//! - `advance_position`：固定步长推进物理位置；
//! - `interpolate_position`：按 alpha 插值出渲染位置。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3103` 查看现象，改正后运行 `bevylings test 3103` 让测试通过。
//!
//! 小贴士：`alpha = 0` 应该取"上一次位置"，`alpha = 1` 应该取"当前位置"。

use bevy::prelude::*;

/// 物理位置：固定步长推进。
#[derive(Component, Default)]
struct PhysicalTranslation(Vec3);

/// 上一次物理位置：插值用的起点。
#[derive(Component, Default)]
struct PreviousPhysicalTranslation(Vec3);

/// 速度：每秒移动多少单位。
#[derive(Component, Default)]
struct Velocity(Vec3);

/// 推进物理位置：新位置 = 当前位置 + 速度 × 步长。
fn advance_position(pos: Vec3, velocity: Vec3, dt: f32) -> Vec3 {
    pos + velocity * dt
}

/// 插值：alpha = 0 时返回 previous，alpha = 1 时返回 current。
fn interpolate_position(previous: Vec3, current: Vec3, alpha: f32) -> Vec3 {
    previous.lerp(current, alpha)
}

/// 在固定步长里推进物理位置，并记下上一步的位置。
fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut PhysicalTranslation, &mut PreviousPhysicalTranslation, &Velocity)>,
) {
    for (mut physical, mut previous, velocity) in &mut query {
        previous.0 = physical.0;
        physical.0 = advance_position(physical.0, velocity.0, fixed_time.delta_secs());
    }
}

/// 每帧按 alpha 插值出渲染位置。
fn interpolate_rendered(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &PhysicalTranslation, &PreviousPhysicalTranslation)>,
) {
    for (mut transform, physical, previous) in &mut query {
        transform.translation =
            interpolate_position(previous.0, physical.0, fixed_time.overstep_fraction());
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(Update, interpolate_rendered)
        .run();
}

/// 生成相机和一个向右移动的小球（速度 2 单位/秒）。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        PhysicalTranslation(Vec3::ZERO),
        PreviousPhysicalTranslation(Vec3::ZERO),
        Velocity(Vec3::new(2.0, 0.0, 0.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_color(Color::srgb(0.3, 0.6, 1.0), Vec2::new(40.0, 40.0)),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physics_advances_by_velocity_times_step() {
        let pos = advance_position(Vec3::ZERO, Vec3::new(2.0, 0.0, 0.0), 0.5);
        assert!((pos - Vec3::new(1.0, 0.0, 0.0)).length() < 1e-5);
    }

    #[test]
    fn interpolation_alpha_zero_is_previous() {
        let got = interpolate_position(Vec3::new(1.0, 0.0, 0.0), Vec3::new(3.0, 0.0, 0.0), 0.0);
        assert!(
            (got - Vec3::new(1.0, 0.0, 0.0)).length() < 1e-5,
            "alpha=0 应该取上一次位置，实际 {got:?}"
        );
    }

    #[test]
    fn interpolation_alpha_one_is_current() {
        let got = interpolate_position(Vec3::new(1.0, 0.0, 0.0), Vec3::new(3.0, 0.0, 0.0), 1.0);
        assert!(
            (got - Vec3::new(3.0, 0.0, 0.0)).length() < 1e-5,
            "alpha=1 应该取当前位置，实际 {got:?}"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings test 3103`，插值测试在 alpha=0 时失败。
// 2. `lerp(a, b, t)` 的结果是 a 与 b 之间"离 a 有 t 比例"的点。
// 3. 我们要 alpha=0 落在 previous 上，所以 previous 应该是第一个参数。
