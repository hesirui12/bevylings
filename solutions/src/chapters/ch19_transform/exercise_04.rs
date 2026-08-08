//! # 练习 19.04 —— 朝向：用 aligned_by 对齐方向
//!
//! 出处：https://bevy.org/examples-webgpu/transforms/align/
//!
//! ## 概念
//! 让物体"朝向"某个方向，最简单的办法是 `Transform::from_xyz(...).looking_at(目标, 上方向)`。
//! 但有时想要的不是"看着某个点"，而是"某个局部轴**精确**对准某个方向"，
//! 这就是 `Transform::aligned_by` 的用武之地：
//! `aligned_by(主轴, 主方向, 副轴, 副方向)` 会把局部主轴精确转到目标方向，
//! 副轴再尽量对齐，物体就不会"躺倒"。
//!
//! 方向用 `Dir3` 表示：单位方向向量，比如 `Dir3::Z`、`Dir3::Y`。
//! 本练习的"飞船"约定机头朝局部 -Z 方向。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1904` 查看现象，改正后运行 `bevylings test 1904` 让测试通过。
//!
//! 小贴士：`aligned_by` 的参数个数是固定的——主轴、主方向、副轴、副方向，一个都不能少。

use bevy::prelude::*;

/// 让"飞船"的机头（局部 -Z 轴）精确对准主方向，X 轴尽量对齐副方向。
fn ship_alignment(primary: Dir3, secondary: Dir3) -> Transform {
    Transform::IDENTITY.aligned_by(Vec3::NEG_Z, primary, Vec3::X, secondary)
}

/// 生成一艘朝向 (0,0,1) 的"飞船"、地面、灯光和相机。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.6, 0.6, 1.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.5, 0.2))),
        ship_alignment(Dir3::Z, Dir3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, -4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
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
    fn nose_points_at_primary_direction() {
        let t = ship_alignment(Dir3::Z, Dir3::Y);
        let forward = t.rotation * Vec3::NEG_Z;
        assert!(
            (forward - Vec3::Z).length() < 1e-4,
            "机头（局部 -Z）应对准主方向 (0,0,1)"
        );
        assert!(
            (t.rotation * Vec3::X).dot(Vec3::Y).abs() > 0.99,
            "X 轴应尽量对齐副方向"
        );
    }

    #[test]
    fn already_aligned_stays_put() {
        let t = ship_alignment(Dir3::NEG_Z, Dir3::X);
        assert!((t.rotation * Vec3::NEG_Z - Vec3::NEG_Z).length() < 1e-4);
    }
}

// 提示：
// 1. 数一数 `aligned_by(...)` 括号里现在有几个参数。
// 2. 函数签名 `ship_alignment(primary: Dir3, secondary: Dir3)` 告诉你有两个方向可用。
// 3. 补上缺失的参数后运行 `bevylings test 1904`，测试全绿就过关。
