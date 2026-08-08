//! # 练习 19.02 —— 旋转：Quat 与 rotate_y
//!
//! 出处：https://bevy.org/examples-webgpu/transforms/3d-rotation/
//!
//! ## 概念
//! 旋转用四元数 `Quat` 表示。平时不用亲手写四元数：
//! - `Quat::from_rotation_y(角度)` 构造"绕 Y 轴转多少度"的四元数；
//! - `Transform::rotate_y(角度)` 直接让 Transform 绕自身 Y 轴旋转；
//! - 角度单位是弧度，一圈 = `TAU`（≈ 6.283）。
//!
//! 所以"每秒转 speed 圈"，这一帧就转 `speed × TAU × 帧时长` 弧度。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1902` 查看现象，改正后运行 `bevylings test 1902` 让测试通过。
//!
//! 小贴士：`time.delta_secs()` 是**方法调用**，括号不能省。

// I AM NOT DONE

use bevy::prelude::*;

use std::f32::consts::TAU;

/// 每秒转几圈。
#[derive(Component)]
struct Rotatable {
    speed: f32,
}

/// 这一帧应该转的弧度数。
fn radians_this_frame(rotations_per_sec: f32, delta_secs: f32) -> f32 {
    rotations_per_sec * TAU * delta_secs
}

/// 每帧旋转所有带 Rotatable 的实体。
fn rotate_cube(time: Res<Time>, mut query: Query<(&mut Transform, &Rotatable)>) {
    for (mut transform, cube) in &mut query {
        // BUG: `delta_secs` 是方法，后面少了调用括号，
        // 把"函数"当"数字"传给 radians_this_frame，编译会报类型错误。
        transform.rotate_y(radians_this_frame(cube.speed, time.delta_secs));
    }
}

/// 生成一个立方体、相机和灯光。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(Vec3::ZERO),
        Rotatable { speed: 0.3 },
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_turn_per_second_is_a_right_angle() {
        let rad = radians_this_frame(0.25, 1.0);
        assert!(
            (rad - std::f32::consts::FRAC_PI_2).abs() < 1e-5,
            "0.25 圈 = 直角 = π/2 弧度"
        );
    }

    #[test]
    fn no_time_means_no_rotation() {
        assert_eq!(radians_this_frame(1.0, 0.0), 0.0);
        assert!(
            (radians_this_frame(1.0, 0.5) - std::f32::consts::PI).abs() < 1e-5,
            "每秒 1 圈转 0.5 秒 = 半圈 = π 弧度"
        );
    }
}

// 提示：
// 1. 编译错误会指出 `delta_secs` 的类型不对，先想它应该是什么类型。
// 2. 方法调用要写成 `xxx()`，别忘了括号。
// 3. 改完运行 `bevylings test 1902`，测试全绿就过关。
