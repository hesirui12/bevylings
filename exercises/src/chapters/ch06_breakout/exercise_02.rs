//! # 练习 06.02 —— 球：速度向量与每帧移动
//!
//! 出处：https://bevy.org/examples/games/breakout/
//!
//! ## 概念
//! 球有一份**速度**（velocity）：一个二维向量，表示"每秒往右/往上各移动多少像素"。
//! 想让球动起来，只需要每帧执行：`新位置 = 旧位置 + 速度 × 帧时间`。
//! 向量和标量相乘（`Vec2 * f32`）会分别作用到两个分量上，
//! 所以 x 和 y 方向可以一次算完。这正是官方示例 `apply_velocity` 系统做的事。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0602` 观察现象，改正后运行 `bevylings test 0602` 让测试通过。
//!
//! 小贴士：速度是"每秒"的位移，计算单帧移动时必须乘以这一帧的时长 `delta_secs`。

// I AM NOT DONE

use bevy::prelude::*;

/// 球的速度大小：每秒 400 像素。
const BALL_SPEED: f32 = 400.0;
/// 球的初始方向（先向左下飞）。
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

/// 球：标记组件。
#[derive(Component)]
struct Ball;

/// 速度：每秒在 x、y 方向各移动多少像素。
#[derive(Component)]
struct Velocity(Vec2);

/// 计算球移动后的新位置：旧位置 + 速度 × 帧时间。
fn move_ball(position: Vec2, velocity: Vec2, delta_secs: f32) -> Vec2 {
    // BUG: 漏乘了帧时间 `delta_secs`。
    // 速度表示"每秒"的位移，不乘时间，就会把一秒的位移当成一帧的位移。
    position + velocity
}

/// 每帧把速度应用到球的位置上。
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        let next = move_ball(transform.translation.truncate(), velocity.0, time.delta_secs());
        transform.translation.x = next.x;
        transform.translation.y = next.y;
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, apply_velocity)
        .run();
}

/// 生成相机和球。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
        Transform::from_xyz(0.0, -50.0, 1.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ball_moves_by_velocity_times_time() {
        let pos = move_ball(Vec2::ZERO, Vec2::new(200.0, -200.0), 0.5);
        assert_eq!(pos, Vec2::new(100.0, -100.0));
    }

    #[test]
    fn zero_delta_means_no_movement() {
        let pos = Vec2::new(3.0, 4.0);
        assert_eq!(move_ball(pos, Vec2::new(400.0, 100.0), 0.0), pos);
    }

    #[test]
    fn vertical_velocity_moves_only_y() {
        let pos = move_ball(Vec2::new(10.0, 10.0), Vec2::new(0.0, -100.0), 0.1);
        assert_eq!(pos, Vec2::new(10.0, 0.0));
    }
}

// 提示：
// 1. 先运行 `bevylings run 0602`，观察球是不是快得离谱。
// 2. 速度是"每秒"的位移：`速度 × delta_secs` 才是这一帧的位移。
// 3. 注意 `Vec2` 与 `f32` 相乘是逐分量相乘，不需要分开写 x、y。
