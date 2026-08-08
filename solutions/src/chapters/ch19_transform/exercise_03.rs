//! # 练习 19.03 —— 缩放：scale 与局部缩放
//!
//! 出处：https://bevy.org/examples-webgpu/transforms/scale/
//!
//! ## 概念
//! `Transform.scale` 是一个 `Vec3`（三个分量），分别代表 x、y、z 方向的放大倍数。
//! - 三个分量一样大 → **均匀缩放**：`Vec3::splat(2.0)` 表示整体放大 2 倍；
//! - 只改一个分量 → **局部缩放**：比如只把 X 轴拉长，物体变成"长条"。
//!
//! 本练习里立方体沿某个方向慢慢长大：每帧 `scale += 方向 × 速度 × 帧时长`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1903` 查看现象，改正后运行 `bevylings test 1903` 让测试通过。
//!
//! 小贴士：`Vec3::X * 2.0` 会得到 `Vec3::new(2.0, 0.0, 0.0)`——标量可以和向量相乘。

use bevy::prelude::*;

use std::f32::consts::PI;

/// 缩放信息：朝哪个方向、以多快的速度缩放。
#[derive(Component)]
struct Scaling {
    scale_direction: Vec3,
    scale_speed: f32,
}

/// 每帧缩放一步：原缩放 + 方向 × 速度 × 帧时长。
fn scale_step(scale: Vec3, direction: Vec3, speed: f32, delta: f32) -> Vec3 {
    scale + direction * speed * delta
}

/// 每帧给所有带 Scaling 的实体加一点缩放。
fn scale_cube(time: Res<Time>, mut query: Query<(&mut Transform, &Scaling)>) {
    for (mut transform, cube) in &mut query {
        transform.scale =
            scale_step(transform.scale, cube.scale_direction, cube.scale_speed, time.delta_secs());
    }
}

/// 生成一个旋转 45° 的立方体、相机和灯光。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_y(PI / 4.0)),
        Scaling {
            scale_direction: Vec3::X,
            scale_speed: 2.0,
        },
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
        .add_systems(Update, scale_cube)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grows_along_the_direction() {
        assert_eq!(
            scale_step(Vec3::ONE, Vec3::X, 2.0, 0.5),
            Vec3::new(2.0, 1.0, 1.0)
        );
        assert_eq!(scale_step(Vec3::ONE, Vec3::Y, 1.0, 1.0), Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn no_time_means_no_scale() {
        assert_eq!(scale_step(Vec3::ONE, Vec3::X, 5.0, 0.0), Vec3::ONE);
    }
}

// 提示：
// 1. 运行 `bevylings run 1903` 观察：物体是在变大还是在变小？
// 2. `scale_step` 想表达"加上一步"，想一想符号该放在哪里。
// 3. 改完运行 `bevylings test 1903`，测试全绿就过关。
