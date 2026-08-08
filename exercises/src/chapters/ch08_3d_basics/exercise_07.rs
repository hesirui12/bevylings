//! # 练习 08.07 —— 顶点颜色：给网格的每个顶点上色
//!
//! 出处：https://bevy.org/examples-webgpu/3d/vertex-colors/
//!
//! ## 概念
//! 除了给整个物体贴一种材质颜色，还可以**按顶点**上色：
//! 网格（Mesh）由许多顶点组成，每个顶点都能带一份颜色，
//! 颜色会在三角形内部平滑过渡，做出渐变色效果。
//!
//! 做法分三步：
//! 1. `Mesh::from(Cuboid::default())` 先做出一个普通网格；
//! 2. 读出顶点的位置 `Mesh::ATTRIBUTE_POSITION`；
//! 3. 按位置算出颜色，写回 `Mesh::ATTRIBUTE_COLOR`。
//!
//! 顶点颜色会和材质 base_color **相乘**，所以材质一般设成白色。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0807` 查看现象，改正后运行 `bevylings test 0807` 让测试通过。
//!
//! 小贴士：颜色是 RGBA 四通道；顶点位置在 [-1,1]，想映射到 [0,1] 的颜色，
//! 可以用公式 `(1 - 位置) / 2`。

// I AM NOT DONE

use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

/// 生成一个顶点颜色随位置变化的彩色立方体。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // 从默认立方体网格出发，给每个顶点按位置上色
    let mut colorful_cube = Mesh::from(Cuboid::default());
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        colorful_cube.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[r, g, b]| vertex_color(*r, *g, *b))
            .collect();
        colorful_cube.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    commands.spawn((
        Mesh3d(meshes.add(colorful_cube)),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 灯光与相机
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 5.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// 顶点位置 [-1, 1] → 颜色 [0, 1]，最后一位是 alpha（不透明）。
fn vertex_color(r: f32, g: f32, b: f32) -> [f32; 4] {
    // BUG: 颜色是 RGBA 四通道（R、G、B、Alpha），
    // 这里只写了三个数，编译会报"数组长度不对"。
    [(1. - r) / 2., (1. - g) / 2., (1. - b) / 2.]
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
    fn center_maps_to_mid_gray() {
        assert_eq!(vertex_color(0.0, 0.0, 0.0), [0.5, 0.5, 0.5, 1.0]);
    }

    #[test]
    fn corners_map_to_black_and_white() {
        assert_eq!(vertex_color(1.0, 1.0, 1.0), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(vertex_color(-1.0, -1.0, -1.0), [1.0, 1.0, 1.0, 1.0]);
    }
}

// 提示：
// 1. 返回值类型写着 `[f32; 4]`，数组元素个数必须和类型一致。
// 2. alpha 通道填 1.0 表示完全不透明。
// 3. 改完运行 `bevylings test 0807`，测试全绿就过关。
