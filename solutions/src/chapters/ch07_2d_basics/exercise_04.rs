//! # 练习 07.04 —— 旋转精灵（Quat 与 rotate_z）
//!
//! 出处：https://bevy.org/examples-webgpu/2d/rotation/
//!
//! ## 概念
//! 2D 游戏里旋转其实是绕着 **Z 轴**（垂直于屏幕、朝向你）转。
//! Bevy 用**四元数（Quat）**表示旋转：`Quat::from_rotation_z(弧度)` 生成
//! "绕 Z 轴转多少"的旋转，把它赋给 `Transform.rotation` 即可。
//! 想让物体持续旋转，就每帧调用 `transform.rotate_z(角速度 × 这一帧秒数)`。
//! 注意角度单位是**弧度**：一圈 = 2π ≈ 6.28 弧度。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0704` 观察现象，改正后运行 `bevylings test 0704` 让测试通过。
//!
//! 小贴士：`f32::to_radians(度数)` 可以把度数换算成弧度，
//! 官方示例里 `f32::to_radians(360.0)` 表示"每秒转一整圈"。

use bevy::prelude::*;

/// 会自己旋转的物体。
#[derive(Component)]
struct Spin {
    /// 每秒旋转的弧度数
    radians_per_second: f32,
}

/// 这一步应该转多少弧度：角速度 × 时间。
fn angle_step(radians_per_second: f32, delta_secs: f32) -> f32 {
    radians_per_second * delta_secs
}

fn spin(mut query: Query<(&Spin, &mut Transform)>, time: Res<Time>) {
    for (spin, mut transform) in &mut query {
        let angle = angle_step(spin.radians_per_second, time.delta_secs());
        transform.rotate_z(angle);
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, spin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 一个每秒转半圈（π 弧度）的方块
    commands.spawn((
        Sprite::from_color(Color::srgb(0.4, 0.7, 1.0), Vec2::new(80.0, 80.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Spin {
            radians_per_second: std::f32::consts::PI,
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_grows_with_time() {
        assert_eq!(angle_step(90.0, 0.5), 45.0);
    }

    #[test]
    fn zero_time_means_no_rotation() {
        assert_eq!(angle_step(90.0, 0.0), 0.0);
    }

    #[test]
    fn rotation_z_turns_x_into_y() {
        // 绕 Z 轴转 90 度：朝右的向量 (1,0,0) 会变成朝上的 (0,1,0)
        let rotated = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2) * Vec3::X;
        assert!((rotated - Vec3::Y).length() < 1e-5);
    }
}

// 提示：
// 1. 先运行 `bevylings run 0704`，看看编译器报"找不到方法"时的建议。
// 2. 官方示例里用的方法叫 `rotate_z(弧度)`，只绕 Z 轴转。
// 3. 改好后运行 `bevylings test 0704`，三个测试全绿就过关了。
