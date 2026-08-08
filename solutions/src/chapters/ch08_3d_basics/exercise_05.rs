//! # 练习 08.05 —— 3D 透明度：alpha 通道与混合模式
//!
//! 出处：https://bevy.org/examples-webgpu/3d/transparency-3d/
//!
//! ## 概念
//! 颜色除了 RGB 三个通道，还有一个 **alpha**（透明度）通道：
//! alpha = 1 完全不透明，alpha = 0 完全看不见。
//! `Color::srgba(r, g, b, a)` 的第四个参数就是 alpha。
//! 当颜色转成材质时，alpha 小于 1 会自动使用 `AlphaMode::Blend`（半透明混合），
//! 这样半透明的立方体就能看到它背后的地面。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0805` 查看现象，改正后运行 `bevylings test 0805` 让测试通过。
//!
//! 小贴士：`srgba` 的 a 参数表示透明度，少写一个参数编译都过不去。

use bevy::prelude::*;

/// 生成一个半透明立方体场景。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 不透明地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(6.0, 6.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // 半透明立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgba(0.5, 0.5, 1.0, 0.5))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 灯光与相机
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// 透明度随时间在 0（全透明）~ 1（不透明）之间来回变化（正弦波）。
fn fade_alpha(t: f32) -> f32 {
    (t.sin() / 2.0) + 0.5
}

/// 每帧更新所有材质的 alpha，让立方体若隐若现。
fn fade_transparency(time: Res<Time>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let alpha = fade_alpha(time.elapsed_secs());
    for (_, material) in materials.iter_mut() {
        material.base_color.set_alpha(alpha);
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, fade_transparency)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fade_starts_at_half() {
        assert_eq!(fade_alpha(0.0), 0.5, "t=0 时 sin(0)=0，透明度为 0.5");
    }

    #[test]
    fn fade_oscillates_between_0_and_1() {
        let peak = std::f32::consts::FRAC_PI_2;
        let trough = -std::f32::consts::FRAC_PI_2;
        assert!((fade_alpha(peak) - 1.0).abs() < 0.001, "峰值应接近 1（不透明）");
        assert!((fade_alpha(trough) - 0.0).abs() < 0.001, "谷值应接近 0（透明）");
    }
}

// 提示：
// 1. 编译错误会提示 `srgba` 期望 4 个参数，现在只给了 3 个。
// 2. 最后一个参数就是 alpha：0 全透明，1 不透明，中间值半透明。
// 3. 改完运行 `bevylings test 0805`，测试全绿就过关。
