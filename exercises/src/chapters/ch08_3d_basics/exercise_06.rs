//! # 练习 08.06 —— 正交相机：没有透视的 3D
//!
//! 出处：https://bevy.org/examples-webgpu/3d/orthographic/
//!
//! ## 概念
//! 默认的 3D 相机是**透视投影**：近大远小，像人眼。
//! **正交投影**（Orthographic）没有远近变化，不管离相机多远，大小都一样，
//! 适合做俯视角游戏（比如模拟经营类）和 CAD 软件。
//!
//! Bevy 里给相机换成正交投影：
//! `Projection::from(OrthographicProjection { scaling_mode: ..., ..OrthographicProjection::default_3d() })`。
//! `ScalingMode::FixedVertical { viewport_height }` 表示"窗口高度对应多少世界单位"，
//! 这个数字越小，看到的世界越少，相当于放大。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0806` 查看现象，改正后运行 `bevylings test 0806` 让测试通过。
//!
//! 小贴士：立方体边长 1，中心在 y=0.5 时恰好"坐"在地面上。

// I AM NOT DONE

use bevy::{camera::ScalingMode, prelude::*};

/// 生成一个正交视角的 3D 场景。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 正交相机
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: camera_viewport_height(),
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // 四个立方体摆成 2×2 方阵
    for x in [-1.5, 1.5] {
        for z in [-1.5, 1.5] {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::default())),
                MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
                Transform::from_translation(grid_position(x, z)),
            ));
        }
    }

    // 灯光
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
}

/// 立方体中心的位置：边长 1 的立方体要"坐"在地面上（y = 0.5）。
fn grid_position(x: f32, z: f32) -> Vec3 {
    // BUG: 立方体应该"坐"在地面上，中心只抬到边长的一半；
    // 这里把 y 写成了整个边长，立方体会悬在半空。
    Vec3::new(x, 1.0, z)
}

/// 窗口高度对应的世界单位数（这个数越小，画面越"放大"）。
fn camera_viewport_height() -> f32 {
    6.0
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
    fn cubes_sit_on_the_plane() {
        assert_eq!(grid_position(1.5, -1.5), Vec3::new(1.5, 0.5, -1.5));
        assert_eq!(grid_position(-1.5, 1.5), Vec3::new(-1.5, 0.5, 1.5));
    }

    #[test]
    fn zoom_uses_fixed_viewport_height() {
        assert_eq!(camera_viewport_height(), 6.0);
        assert!(camera_viewport_height() > 0.0, "视口高度必须是正数");
    }
}

// 提示：
// 1. 想想"边长 1、底边贴地"的立方体，中心离地多高？
// 2. 只改 y 分量，x、z 都不要动。
// 3. 改完运行 `bevylings test 0806`，测试全绿就过关。
