//! # 练习 26.02 —— 生成场景：WorldAssetRoot
//!
//! 出处：https://bevy.org/examples-webgpu/gltf/load_gltf/
//!
//! ## 概念
//! 上一题我们把模型**加载**成了句柄。这一题聚焦"**生成**"：
//! 同一个 glTF 场景可以在游戏里出现多次（比如一棵树摆十棵）。
//! 0.19 里场景生成的标准做法是 `commands.spawn(WorldAssetRoot(句柄))`：
//! 把 `WorldAssetRoot` 组件和 `Transform` 一起生成，模型就会作为
//! 该实体的子物体出现在指定位置。
//!
//! 注意类型：`GltfAssetLabel::Scene(0).from_asset(...)` 加载出来的
//! 句柄是 `Handle<WorldAsset>`；如果手动把加载类型标成 `Gltf`
//! （整个文件），类型就对不上了，编译会报错。`asset_server.load`
//! 不写类型注解时，Bevy 会从使用处自动推断。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2602` 观察现象，改正后运行 `bevylings test 2602` 让测试通过。
//!
//! 小贴士：`scene_offset(index)` 用来计算第 index 份副本摆在哪，
//! 是一个纯函数，直接就能测。

// I AM NOT DONE

use bevy::{asset::AssetPath, prelude::*};

/// 标记"这棵场景稍后要整体移动"。
#[derive(Component)]
struct MovedScene;

/// 生成 glTF 第 `scene_index` 个场景的资产路径。
fn scene_asset_path(name: &'static str, scene_index: usize) -> AssetPath<'static> {
    GltfAssetLabel::Scene(scene_index).from_asset(name)
}

/// 第 `index` 份副本的水平位置：以原点为中心，两两间隔 1.5。
fn scene_offset(index: u32) -> Vec3 {
    Vec3::new((index as f32 * 2.0 - 1.0) * 1.5, 0.0, 0.0)
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 第一份：摆在左侧
    commands.spawn((
        WorldAssetRoot(asset_server.load(scene_asset_path(
            "models/FlightHelmet/FlightHelmet.gltf",
            0,
        ))),
        Transform::from_translation(scene_offset(0)),
    ));

    // 第二份：摆在右侧，并打上 MovedScene 标签
    commands.spawn((
        // BUG: 这里手动把加载类型标成了 Gltf（整个文件），
        // 而 WorldAssetRoot 需要 Handle<WorldAsset>，类型不匹配编译不过。
        WorldAssetRoot(asset_server.load::<Gltf>(scene_asset_path(
            "models/FlightHelmet/FlightHelmet.gltf",
            0,
        ))),
        Transform::from_translation(scene_offset(1)),
        MovedScene,
    ));

    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
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
    fn first_copy_is_on_the_left() {
        assert_eq!(scene_offset(0), Vec3::new(-1.5, 0.0, 0.0));
    }

    #[test]
    fn second_copy_is_on_the_right() {
        assert_eq!(scene_offset(1), Vec3::new(1.5, 0.0, 0.0));
        assert_eq!(
            scene_asset_path("models/m.gltf", 0).label(),
            Some("Scene0")
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 2602`，看编译错误提示什么类型不匹配。
// 2. 去掉 `load` 后面尖括号里的类型注解，让 Bevy 从 WorldAssetRoot 推断。
// 3. 改好后运行 `bevylings test 2602`，两个测试全绿就过关了。
