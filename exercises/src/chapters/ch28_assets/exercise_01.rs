//! # 练习 28.01 —— 资产加载：AssetServer 与 Assets 集合
//!
//! 出处：https://bevy.org/examples-webgpu/asset/asset-loading/
//!
//! ## 概念
//! `AssetServer::load` 负责"异步加载"资产：它立刻返回一个 `Handle`（句柄），
//! 但文件内容要过一会儿才到。加载完成后，资产会出现在对应的 `Assets<T>`
//! 集合里（比如网格在 `Assets<Mesh>` 里）。
//! 所以判断"加载好没有"的标准写法是：`meshes.get(&handle).is_some()`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2801` 查看现象，改正后运行 `bevylings test 2801` 让测试通过。
//!
//! 小贴士：`assets.add(...)` 是"手动往集合里塞资产"，返回一个立刻可用的 handle。

// I AM NOT DONE

use bevy::prelude::*;

/// 网格是否已经加载完成？
fn has_loaded(meshes: &Assets<Mesh>, handle: &Handle<Mesh>) -> bool {
    // BUG: 没用传入的 handle 去查，而是新建了一个空 handle，
    // 空 handle 永远不在集合里，结果永远返回 false。
    let empty = Handle::<Mesh>::default();
    meshes.get(&empty).is_some()
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load 是异步的：这里只拿到 handle，网格稍后才会进 Assets<Mesh>
    let cube_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("models/cube/cube.gltf"),
    );
    let sphere_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("models/sphere/sphere.gltf"),
    );

    commands.spawn((Mesh3d(cube_handle), Transform::from_xyz(-2.0, 0.0, 0.0)));
    commands.spawn((Mesh3d(sphere_handle), Transform::from_xyz(2.0, 0.0, 0.0)));

    // 灯光与相机
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 5.0, 4.0)));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loaded_mesh_is_found() {
        let mut meshes = Assets::<Mesh>::default();
        let handle = meshes.add(Mesh::from(Torus::new(0.8, 1.2)));
        assert!(has_loaded(&meshes, &handle), "add 进集合的网格应该能查到");
    }

    #[test]
    fn empty_collection_has_nothing() {
        let meshes = Assets::<Mesh>::default();
        let handle = Handle::<Mesh>::default();
        assert!(!has_loaded(&meshes, &handle));
    }
}

// 提示：
// 1. `Assets::get` 需要传入"要查的那个 handle"，才知道查谁。
// 2. `Handle::default()` 是个无效句柄，拿它查永远查不到。
// 3. 修改后运行 `bevylings test 2801`，两个测试全绿就过关了。
