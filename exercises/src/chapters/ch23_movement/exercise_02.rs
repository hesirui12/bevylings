//! # 练习 23.02 —— 固定时间步移动（FixedUpdate）
//!
//! 出处：https://bevy.org/examples-webgpu/movement/physics_in_fixed_timestep/
//!
//! ## 概念
//! 游戏每帧的耗时是不固定的（有的帧 16 毫秒，有的 5 毫秒）。
//! 如果每帧都移动固定距离，速度就会随帧率忽快忽慢。更稳的做法是把
//! **物理推进**放到 `FixedUpdate` 调度里：它每隔固定时长（比如 1/60 秒）
//! 运行一次，与帧率无关。系统里 `Res<Time<Fixed>>` 提供固定步长，
//! `advance_position` 用公式 `新位置 = 当前位置 + 速度 × 步长` 推进。
//! 官方示例还会把"物理位置"和"渲染位置"分开存，这里我们简化成一份。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2302` 观察现象，改正后运行 `bevylings test 2302` 让测试通过。
//!
//! 小贴士：`Time::<Fixed>::from_hz(60.0)` 表示"每秒固定跑 60 次物理推进"。

// I AM NOT DONE

use bevy::prelude::*;

/// 玩家在"物理世界"里的位置（不随帧率变化）。
#[derive(Component, Default)]
struct PhysicalTranslation(Vec3);

/// 玩家的速度：每秒移动多少个单位。
#[derive(Component, Default)]
struct Velocity(Vec3);

/// 按固定步长推进位置：新位置 = 当前位置 + 速度 × 步长。
fn advance_position(current: Vec3, velocity: Vec3, dt: f32) -> Vec3 {
    current + velocity * dt
}

/// 在 `FixedUpdate` 里推进物理位置。
fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    query: Query<(&PhysicalTranslation, &Velocity)>,
) {
    for (physical, velocity) in &query {
        // BUG: 查询参数写成了只读的 `&`，但这一行要修改 `physical` 的值。
        // 想一想：要修改组件，查询里的引用应该怎么写？
        physical.0 = advance_position(physical.0, velocity.0, fixed_time.delta_secs());
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(Update, sync_transform)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 一个以每秒 120 单位向右移动的方块
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        PhysicalTranslation(Vec3::new(0.0, 0.0, 0.0)),
        Velocity(Vec3::new(120.0, 0.0, 0.0)),
    ));
}

/// 把物理位置同步到渲染用的 Transform。
fn sync_transform(mut query: Query<(&mut Transform, &PhysicalTranslation)>) {
    for (mut transform, physical) in &mut query {
        transform.translation = physical.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_by_velocity_times_step() {
        let p = advance_position(Vec3::ZERO, Vec3::new(120.0, 0.0, 0.0), 0.1);
        assert!((p - Vec3::new(12.0, 0.0, 0.0)).length() < 1e-5);
    }

    #[test]
    fn zero_step_means_no_movement() {
        let p = advance_position(Vec3::new(3.0, 4.0, 0.0), Vec3::new(100.0, 0.0, 0.0), 0.0);
        assert!((p - Vec3::new(3.0, 4.0, 0.0)).length() < 1e-5);
    }

    #[test]
    fn faster_velocity_moves_further() {
        let slow = advance_position(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), 1.0);
        let fast = advance_position(Vec3::ZERO, Vec3::new(2.0, 0.0, 0.0), 1.0);
        assert!(fast.x > slow.x);
    }
}

// 提示：
// 1. 先运行 `bevylings run 2302`，看看编译器对 `physical.0 = ...` 报什么错。
// 2. 想一想：只读借用 `&` 能不能修改值？查询参数要怎么写才允许修改？
// 3. 改好后运行 `bevylings test 2302`，三个测试全绿就过关了。
