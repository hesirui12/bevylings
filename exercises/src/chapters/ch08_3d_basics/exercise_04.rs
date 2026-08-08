//! # 练习 08.04 —— 父子关系：让子物体跟着父物体动
//!
//! 出处：https://bevy.org/examples-webgpu/3d/parenting/
//!
//! ## 概念
//! Bevy 里实体可以"认爹"：`children![(子实体的组件...)]` 写在父实体的生成元组里，
//! 就把子实体挂到父实体下面。子实体的 Transform 是**相对父实体**的，
//! 所以父实体一移动/旋转，子实体会跟着一起动（像火车头和车厢）。
//!
//! 本练习里"父"是一个会绕 X 轴旋转的立方体，"子"是挂在它上面的小立方体。
//! 旋转角度按"速度 × 帧时长"匀速增长。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0804` 查看现象，改正后运行 `bevylings test 0804` 让测试通过。
//!
//! 小贴士：旋转的角度单位是弧度（radian），一圈 = 2π ≈ 6.28。

// I AM NOT DONE

use bevy::prelude::*;

/// 标记"需要每帧旋转"的父立方体。
#[derive(Component)]
struct Rotator;

/// 每秒转多少弧度。
const ROTATION_SPEED: f32 = 3.0;

/// 这一帧应该转的角度：速度 × 帧时长。
fn rotation_step(speed: f32, delta: f32) -> f32 {
    // BUG: 角度应该随帧时长**成比例**地增长（乘法），
    // 这里写成了加法，每帧都加上"速度 + 帧时长"，角度会飞快失控。
    speed + delta
}

/// 每帧旋转所有带 Rotator 的实体（它们的子实体会自动跟着转）。
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in &mut query {
        transform.rotate_x(rotation_step(ROTATION_SPEED, time.delta_secs()));
    }
}

/// 生成父立方体和挂在它下面的子立方体。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    // 父立方体 + 挂在它下面的子立方体
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Rotator,
        children![(
            Mesh3d(cube),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, 0.0, 3.0),
        )],
    ));

    // 灯光与相机
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 5.0, -4.0)));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotator_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_grows_proportionally() {
        assert_eq!(rotation_step(3.0, 0.0), 0.0, "没有时间流逝就不该转");
        assert_eq!(rotation_step(3.0, 0.5), 1.5, "0.5 秒应该转 1.5 弧度");
    }

    #[test]
    fn faster_speed_rotates_more() {
        assert!(rotation_step(2.0, 1.0) > rotation_step(1.0, 1.0));
        assert_eq!(rotation_step(2.0, 1.0), 2.0);
    }
}

// 提示：
// 1. 先想：角度增长和"速度 × 时间"是什么关系？
// 2. 帧时长 delta 是小数（比如 1/60 秒），用加号会让角度瞬间失控。
// 3. 改完运行 `bevylings test 0804`，两个测试全绿就过关。
