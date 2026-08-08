//! # 练习 18.01 —— Time 与 delta：按时间移动
//!
//! 出处：https://bevy.org/examples-webgpu/time/time/
//!
//! ## 概念
//! 游戏是"一帧一帧"跑的，每帧之间经过的时间叫 **delta**（`time.delta_secs()`）。
//! 移动要写"速度 × 时间"：`新位置 = 旧位置 + 速度 × delta`。
//! 乘上 delta 后，不管电脑快慢，每秒移动的距离都一样 —— 这就是"帧率无关"。
//!
//! 本练习让一个方块每帧按 SPEED 像素/秒向右移动。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1801` 查看现象，改正后运行 `bevylings test 1801` 让测试通过。
//!
//! 小贴士：移动量 = 速度 × 时间。忘乘时间的话，游戏快慢会随帧率变化。

use bevy::prelude::*;

/// 移动速度（像素/秒）。
const SPEED: f32 = 100.0;

/// 移动标记。
#[derive(Component)]
struct Mover;

/// 计算新位置：旧位置 + 速度 × 经过时间。
fn next_x(current: f32, speed: f32, delta_secs: f32) -> f32 {
    current + speed * delta_secs
}

/// 每帧用 delta 更新方块位置。
fn move_sprite(mut query: Query<&mut Transform, With<Mover>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x = next_x(transform.translation.x, SPEED, time.delta_secs());
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_sprite)
        .run();
}

/// 生成相机和方块。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mover,
        Sprite::from_color(Color::srgb(0.3, 0.7, 0.9), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_by_speed_times_delta() {
        assert_eq!(next_x(10.0, 5.0, 2.0), 20.0, "10 + 5×2 = 20");
        assert_eq!(next_x(0.0, 100.0, 0.5), 50.0, "0 + 100×0.5 = 50");
    }

    #[test]
    fn does_not_move_when_delta_zero() {
        assert_eq!(next_x(10.0, 5.0, 0.0), 10.0, "时间没走，位置不该变");
    }
}

// 提示：
// 1. 先运行 `bevylings run 1801`，观察方块是不是"飞"得太快。
// 2. `next_x` 应该体现"速度 × 时间"，delta_secs 就是那个"时间"。
// 3. 补上乘法再运行 `bevylings test 1801`。
