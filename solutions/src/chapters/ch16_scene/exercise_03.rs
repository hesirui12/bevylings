//! # 练习 16.03 —— 加载外部场景：WorldAssetRoot
//!
//! 出处：https://bevy.org/examples-webgpu/scene/bsn/
//!
//! ## 概念
//! 除了用 BSN 在代码里"手写"场景，我们还能加载**外部文件**里的场景，
//! 比如 glTF 模型文件。0.19 的做法：
//! - `asset_server.load(路径)` 得到场景句柄（Handle）；
//! - `WorldAssetRoot(句柄)` 组件告诉引擎"把加载好的场景生成进世界"；
//! - glTF 文件里可能有多个场景，用带标签的路径精确选择：
//!   `GltfAssetLabel::Scene(0).from_asset("models/xxx.gltf")`
//!   等价于路径 `"models/xxx.gltf#Scene0"`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1603` 查看现象，改正后运行 `bevylings test 1603` 让测试通过。
//!
//! 小贴士：旧版本 Bevy 用 `SceneRoot` 组件，0.19 改名成 `WorldAssetRoot`；
//! 用旧名字编译会直接报 "cannot find"。

use bevy::{asset::AssetPath, prelude::*};

/// 要加载的头盔模型：第 0 号场景的带标签路径。
fn helmet_scene_path() -> AssetPath<'static> {
    GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")
}

/// 启动时生成相机，并挂载加载好的外部场景。
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn(WorldAssetRoot(asset_server.load(helmet_scene_path())));
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
    fn path_points_to_helmet_scene_zero() {
        let path = helmet_scene_path().to_string();
        assert!(
            path.starts_with("models/FlightHelmet/FlightHelmet.gltf"),
            "路径前缀应该是模型文件，实际: {path}"
        );
        assert!(
            path.contains("Scene0"),
            "应该带 #Scene0 标签选择第 0 号场景，实际: {path}"
        );
    }
}

// 提示：
// 1. 编译器会告诉你 `SceneRoot` 这个名字不存在（0.19 已移除）。
// 2. 在官方 glTF 示例里搜一搜，加载场景用的是哪个组件？
// 3. 改好后运行 `bevylings test 1603`，测试通过就过关。
