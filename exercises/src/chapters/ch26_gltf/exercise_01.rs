//! # 练习 26.01 —— 加载 glTF 模型
//!
//! 出处：https://bevy.org/examples-webgpu/gltf/load_gltf/
//!
//! ## 概念
//! glTF 是游戏界常见的 3D 模型格式（`.gltf` / `.glb` 文件）。
//! 用 `asset_server.load(...)` 可以异步加载它。注意：一个 glTF 文件里
//! 可能有好几个"场景（Scene）"，所以加载时要**加标签**指明要哪个：
//! `GltfAssetLabel::Scene(0).from_asset("模型路径")` 会生成一个带
//! `#Scene0` 标签的资产路径，加载出来的是可以直接放进游戏世界的
//! `WorldAsset`（场景）。
//!
//! 如果不加标签，加载出来的就是"整个文件"（`Gltf`），而不是某个场景。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2601` 观察现象，改正后运行 `bevylings test 2601` 让测试通过。
//!
//! 小贴士：`AssetPath` 的 `label()` 方法可以读出路径上的标签，
//! 测试就是用它来检查你有没有把标签挂上去。

// I AM NOT DONE

use bevy::{asset::AssetPath, prelude::*};

/// 生成"加载 glTF 文件里第 `scene_index` 个场景"的资产路径。
fn scene_asset_path(name: &'static str, scene_index: usize) -> AssetPath<'static> {
    // BUG: 这里把场景编号的标签弄丢了——只把文件名变成了路径，
    // 后面加载出来的是整个文件，而不是第 scene_index 个场景。
    AssetPath::from(name.to_string())
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
    ));

    // 加载模型第 0 个场景并生成它（0.19 用 WorldAssetRoot 生成场景）
    commands.spawn(WorldAssetRoot(asset_server.load(scene_asset_path(
        "models/FlightHelmet/FlightHelmet.gltf",
        0,
    ))));
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
    fn scene_path_has_scene_label() {
        let path = scene_asset_path("models/model.gltf", 0);
        assert_eq!(path.label(), Some("Scene0"), "路径应该带 Scene0 标签");
    }

    #[test]
    fn scene_path_points_to_file() {
        let path = scene_asset_path("models/FlightHelmet/FlightHelmet.gltf", 2);
        assert_eq!(
            path.path().to_str(),
            Some("models/FlightHelmet/FlightHelmet.gltf"),
            "文件名部分不应该被改动"
        );
        assert_eq!(path.label(), Some("Scene2"));
    }
}

// 提示：
// 1. 先运行 `bevylings test 2601`，看看第一个测试为什么失败。
// 2. `GltfAssetLabel::Scene(scene_index)` 可以构造"第几个场景"的标签，
//    再用它的 `from_asset(路径)` 把标签挂到路径上。
// 3. 改好后运行 `bevylings test 2601`，两个测试全绿就过关了。
