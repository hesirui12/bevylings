//! # 练习 08.02 —— 多种形状与颜色
//!
//! 出处：https://bevy.org/examples-webgpu/3d/3d-shapes/
//!
//! ## 概念
//! 官方 3d_shapes 示例展示了十几种"形状原语"。形状只是一串数学参数
//! （比如球只需要一个半径），`meshes.add(...)` 会把它们转成真正的网格。
//! 本练习用其中三种：
//! - `Sphere`：球体，`Sphere::new(半径).mesh().uv(经度细分, 纬度细分)`；
//! - `Cylinder`：圆柱，`Cylinder::new(半径, 高度).mesh().resolution(侧面数)`；
//! - `Torus`：甜甜圈，`Torus::default()` 就是默认大小。
//!
//! 给每个形状单独指定一种 `base_color`，它们就五颜六色了。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0802` 查看现象，改正后运行 `bevylings test 0802` 让测试通过。
//!
//! 小贴士：`shape_x(i)` 负责把第 i 个形状放到对应的横坐标，让三个形状排成一行。

// I AM NOT DONE

use bevy::prelude::*;

/// 生成三个形状：球、圆柱、甜甜圈，排成一行。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shapes = [
        meshes.add(Sphere::new(0.5).mesh().uv(32, 18)),
        meshes.add(Cylinder::new(0.5, 2.0).mesh().resolution(50)),
        meshes.add(Torus::default()),
    ];
    let colors = [
        Color::srgb(0.9, 0.2, 0.3),
        Color::srgb(0.2, 0.8, 0.3),
        Color::srgb(0.3, 0.4, 0.9),
    ];

    for (i, mesh) in shapes.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(colors[i])),
            Transform::from_xyz(shape_x(i), 1.0, 0.0),
        ));
    }

    // 灯光与相机
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 10.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}

/// 第 i 个形状的横坐标：0、1、2 号分别放在 x = -2、0、2。
fn shape_x(column: usize) -> f32 {
    // BUG: 运算顺序错了：先乘后减，0、1、2 号被放到了 -1、1、3，
    // 三个形状挤到了同一边，而不是以 0 号为对称中心排成一行。
    column as f32 * 2.0 - 1.0
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
    fn shapes_line_up_centered() {
        assert_eq!(shape_x(0), -2.0, "0 号形状在最左边");
        assert_eq!(shape_x(1), 0.0, "1 号形状在中间");
        assert_eq!(shape_x(2), 2.0, "2 号形状在最右边");
    }

    #[test]
    fn columns_are_evenly_spaced() {
        let gap = shape_x(1) - shape_x(0);
        assert_eq!(shape_x(2) - shape_x(1), gap, "相邻间距应该相等");
    }
}

// 提示：
// 1. 想一想 `i as f32 * 2.0 - 1.0` 和 `(i as f32 - 1.0) * 2.0` 有什么不同。
// 2. 括号能改变运算顺序：先减再乘，才能以 0 号为对称中心。
// 3. 改完运行 `bevylings test 0802`，两个测试全绿就过关。
