//! # 练习 08.01 —— 3D 场景：相机、立方体与灯光
//!
//! 出处：https://bevy.org/examples-webgpu/3d/3d-scene/
//!
//! ## 概念
//! 一个最简单的 3D 场景需要四样东西：
//! - **网格（Mesh）**：物体的形状，比如立方体 `Cuboid`；
//! - **材质（Material）**：物体表面的颜色，这里用 `StandardMaterial`；
//! - **灯光**：`PointLight` 像一个灯泡，从一点向四周发光；
//! - **相机**：`Camera3d` 是观察 3D 世界的"眼睛"。
//!
//! `meshes.add(...)` 会把形状存进资源仓库并返回一个句柄，
//! 再用 `Mesh3d` / `MeshMaterial3d` 各包一层，才能把句柄挂到实体上。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0801` 查看现象，改正后运行 `bevylings test 0801` 让测试通过。
//!
//! 小贴士：2D 里的矩形只需要宽和高两个数字，3D 里的立方体需要 x、y、z 三个维度的尺寸。

// I AM NOT DONE

use bevy::prelude::*;

/// 生成一个 3D 场景：地面、立方体、灯光、相机。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 地面：一块铺在地上的平面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // 立方体：边长 1，放在地面正中央
    commands.spawn((
        // BUG: Cuboid 需要三个尺寸（x、y、z），这里只写了两个，
        // 编译时会报"参数个数不对"，把缺失的维度补上即可。
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, cube_center_y(1.0), 0.0),
    ));

    // 灯光：像灯泡一样照亮四周
    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // 相机：从斜上方看向原点
    commands.spawn((Camera3d::default(), camera_transform()));
}

/// 立方体中心的高度：边长的一半（另一半"埋"在地面之下）。
fn cube_center_y(size: f32) -> f32 {
    size / 2.0
}

/// 相机的位置与朝向。
fn camera_transform() -> Transform {
    Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y)
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
    fn cube_sits_on_ground() {
        assert_eq!(cube_center_y(1.0), 0.5, "边长 1 的立方体中心高度应是 0.5");
        assert_eq!(cube_center_y(2.0), 1.0);
    }

    #[test]
    fn camera_looks_from_above_front() {
        let cam = camera_transform();
        assert_eq!(cam.translation, Vec3::new(-2.5, 4.5, 9.0));
        assert!(cam.translation.y > 0.0, "相机应该在场景上方");
    }
}

// 提示：
// 1. 运行 `bevylings run 0801`，先读一下编译错误。
// 2. 3D 形状的尺寸参数个数是固定的：Cuboid 需要三个数（x、y、z）。
// 3. 改完后运行 `bevylings test 0801`，测试全绿就算过关。
