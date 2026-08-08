//! # 练习 08.03 —— 三种灯光：点光源、平行光与环境光
//!
//! 出处：https://bevy.org/examples-webgpu/3d/lighting/
//!
//! ## 概念
//! Bevy 3D 里常见的灯光有三种，用途完全不同：
//! - `PointLight`：像灯泡，从**一个点**向四面八方发光，越远越暗；
//! - `DirectionalLight`：像太阳，光**平行**射过来，只有方向没有位置；
//! - `GlobalAmbientLight`：环境光，**均匀地**照亮场景里每个物体（连阴影里也有），
//!   它是一个全局资源（Resource），用 `commands.insert_resource` 塞进世界。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0803` 查看现象，改正后运行 `bevylings test 0803` 让测试通过。
//!
//! 小贴士：Bevy 的光源组件有很多可选字段（阴影、衰减……），
//! 用 `..default()` 就能让"其余字段都用默认值"。

use bevy::prelude::*;

use std::f32::consts::PI;

/// 生成一个被三种灯光照亮的场景。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 地面与立方体
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 环境光：资源，均匀照亮所有物体
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(1.0, 0.5, 0.2).into(),
        brightness: ambient_brightness(),
        ..default()
    });

    // 点光源：红色灯泡
    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            color: Color::srgb(1.0, 0.2, 0.2).into(),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 2.0, 0.0),
    ));

    // 平行光：像太阳一样从斜上方射下来
    commands.spawn((
        DirectionalLight {
            illuminance: 50_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: sun_rotation(),
            ..default()
        },
    ));

    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// 环境光的亮度（单位：坎德拉/平方米）。
fn ambient_brightness() -> f32 {
    200.0
}

/// 平行光的方向：绕 X 轴转 -45°，从斜上方照下来。
fn sun_rotation() -> Quat {
    Quat::from_rotation_x(-PI / 4.0)
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ambient_light_is_bright() {
        assert_eq!(ambient_brightness(), 200.0);
        assert!(ambient_brightness() > 0.0, "亮度应为正数");
    }

    #[test]
    fn sun_shines_from_above() {
        let rotation = sun_rotation();
        // 绕 X 轴转 -45° 时，四元数为 (x, y, z, w) = (sin(-22.5°), 0, 0, cos(-22.5°))
        assert!(
            (rotation.x + 0.3827).abs() < 0.01,
            "x 分量应约为 sin(-22.5°)≈-0.3827，实际 {}",
            rotation.x
        );
        assert!(
            (rotation.w - 0.9239).abs() < 0.01,
            "w 分量应约为 cos(22.5°)≈0.9239，实际 {}",
            rotation.w
        );
    }
}

// 提示：
// 1. 编译错误会列出 PointLight 缺失的字段，数一数有多少个。
// 2. 用 `..default()` 补上"其余字段"是最地道的 Bevy 写法（struct update syntax）。
// 3. 改完运行 `bevylings test 0803`，测试全绿就过关。
