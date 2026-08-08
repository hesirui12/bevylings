//! # 练习 14.03 —— 坐标轴：axes gizmo
//!
//! 出处：https://bevy.org/examples-webgpu/gizmos/axes/
//!
//! ## 概念
//! `gizmos.axes(transform, length)` 会在 `transform` 的位置画三条互相
//! 垂直的坐标轴：红 = X，绿 = Y，蓝 = Z，长度由第二个参数决定。
//!
//! 官方示例用物体的**轴对齐包围盒（AABB）**来定轴的长度：AABB 记录了
//! 物体在三个方向上各能伸展多远（半尺寸 `half_extents`），
//! 取半尺寸向量的整体长度作为坐标轴长度，物体越大、坐标轴越长，
//! 方便一眼看出物体的朝向和大小。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1403` 观察现象，改正后运行 `bevylings test 1403` 让测试通过。
//!
//! 小贴士：`Vec3::length()` 是向量长度，(x, y, z) 的长度是 √(x²+y²+z²)。

use bevy::camera::primitives::Aabb;
use bevy::prelude::*;

/// 坐标轴的长度：取 AABB 半尺寸向量的整体长度（对角线长度）。
fn axis_length(half_extents: Vec3A) -> f32 {
    half_extents.length()
}

/// 给挂了 ShowAxes 的物体画坐标轴。
fn draw_axes(mut gizmos: Gizmos, query: Query<(&Transform, &Aabb), With<ShowAxes>>) {
    for (&transform, &aabb) in &query {
        gizmos.axes(transform, axis_length(aabb.half_extents));
    }
}

/// 标记：需要画坐标轴的物体。
#[derive(Component)]
struct ShowAxes;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_axes)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        ShowAxes,
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_length_is_diagonal() {
        // 3-4-5 直角三角形：长度为 5
        assert!((axis_length(Vec3A::new(3.0, 4.0, 0.0)) - 5.0).abs() < 1e-5);
    }

    #[test]
    fn unit_cube_axis_length() {
        let len = axis_length(Vec3A::splat(1.0));
        assert!(
            (len - 3.0_f32.sqrt()).abs() < 1e-5,
            "对角线长度应为 √3，实际 {len}"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 1403`，观察坐标轴是不是明显偏短。
// 2. "整体长度"是向量方法，查一下 Vec3/Vec3A 上求长度的那个方法。
// 3. 修改后运行 `bevylings test 1403`，测试全绿就算过关。
