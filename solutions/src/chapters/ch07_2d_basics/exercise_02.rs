//! # 练习 07.02 —— 2D 形状（Mesh2d + MeshMaterial2d）
//!
//! 出处：https://bevy.org/examples-webgpu/2d/2d_shapes/
//!
//! ## 概念
//! 圆形、矩形这些"形状"只是一个数学描述（比如圆 = 半径，矩形 = 宽高），
//! 计算机画图时需要把它们变成由三角形拼成的**网格（Mesh）**。
//! Bevy 里 `meshes.add(Circle::new(半径))` 负责把形状存进网格仓库，
//! `Mesh2d` 把网格挂到实体上，`MeshMaterial2d(materials.add(颜色))` 负责上色。
//! 矩形注意：`Rectangle::new(a, b)` 里的 a、b 是**半宽和半高**。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0702` 观察现象，改正后运行 `bevylings test 0702` 让测试通过。
//!
//! 小贴士：`Sprite` 只能贴图片/色块，而 `Mesh2d` 可以画出任意多边形，
//! 是 2D 游戏做"矢量图形"的主力。

use bevy::prelude::*;

/// 我们支持的两种形状。
#[derive(Clone, Copy, PartialEq, Debug)]
enum ShapeKind {
    Circle,
    Rectangle,
}

/// 每种形状的"尺寸"：圆形的 x==y 表示半径，矩形的 x、y 表示半宽、半高。
fn shape_size(kind: ShapeKind) -> Vec2 {
    match kind {
        ShapeKind::Circle => Vec2::splat(50.0),
        ShapeKind::Rectangle => Vec2::new(100.0, 50.0),
    }
}

/// 把一种形状放进世界：先生成网格，再挂材质，最后放到指定位置。
fn spawn_shape(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    kind: ShapeKind,
    color: Color,
    x: f32,
) {
    let size = shape_size(kind);
    let mesh = match kind {
        ShapeKind::Circle => meshes.add(Circle::new(size.x)),
        ShapeKind::Rectangle => meshes.add(Rectangle::new(size.x, size.y)),
    };
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(x, 0.0, 0.0),
    ));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 三个形状并排放在屏幕上
    spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeKind::Circle, Color::srgb(1.0, 0.3, 0.3), -150.0);
    spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeKind::Rectangle, Color::srgb(0.3, 0.6, 1.0), 0.0);
    spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeKind::Circle, Color::srgb(0.3, 1.0, 0.5), 150.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_is_wider_than_tall() {
        assert_eq!(shape_size(ShapeKind::Rectangle), Vec2::new(100.0, 50.0));
    }

    #[test]
    fn circle_radius_is_uniform() {
        assert_eq!(shape_size(ShapeKind::Circle), Vec2::splat(50.0));
    }

    #[test]
    fn shapes_have_different_sizes() {
        assert_ne!(
            shape_size(ShapeKind::Circle),
            shape_size(ShapeKind::Rectangle)
        );
    }
}

// 提示：
// 1. 先看看矩形的"半宽、半高"概念：`Rectangle::new(a, b)` 中的 a、b 各指什么？
// 2. 设计意图是矩形横着放，宽 200、高 100，也就是半宽 100、半高 50。
// 3. 改好后运行 `bevylings test 0702`，三个测试全绿就过关了。
