//! # 练习 26.04 —— 查询 glTF 图元：按名字改材质
//!
//! 出处：https://bevy.org/examples-webgpu/gltf/query_gltf_primitives/
//!
//! ## 概念
//! 一个 glTF 网格可能由多个"图元（primitive）"组成，每个图元各自
//! 带一份材质。加载 glTF 时，Bevy 会给每个材质实体挂上
//! `GltfMaterialName` 组件（装着材质在文件里的名字）。我们可以用查询
//! `Query<(&MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>`
//! 找到它们，按名字定位要改的材质，再通过 `Assets<StandardMaterial>`
//! 拿到材质本体修改属性。
//!
//! 官方示例专门找名为 "Top" 的材质做动画。本练习简化成：
//! 把名为 "Top" 的材质统一染成红色，其余的一律不动。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2604` 观察现象，改正后运行 `bevylings test 2604` 让测试通过。
//!
//! 小贴士：`MeshMaterial3d<StandardMaterial>` 解引用后就是
//! `Handle<StandardMaterial>`，可以直接传给 `assets.get_mut`。

// I AM NOT DONE

use bevy::{gltf::GltfMaterialName, prelude::*};

/// 把名为 "Top" 的材质全部染成红色，其余不动。
fn tint_top_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>,
) {
    for (material_handle, name) in &query {
        // BUG: 条件写反了——现在会把"不是 Top"的材质染红，
        // 而真正叫 Top 的反而躲过一劫。
        if name.0 != "Top" {
            if let Some(mut material) = materials.get_mut(material_handle) {
                material.base_color = Color::srgb(1.0, 0.0, 0.0);
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 4.0, 12.0).looking_at(Vec3::new(0.0, 0.0, 0.5), Vec3::Y),
    ));

    // 加载并生成模型（多个图元、多种材质）
    commands.spawn(WorldAssetRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("models/GltfPrimitives/gltf_primitives.glb"),
    )));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, tint_top_materials)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_material_turns_red() {
        let mut app = App::new();
        let mut assets = Assets::<StandardMaterial>::default();
        let top_handle = assets.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            ..default()
        });
        let other_handle = assets.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            ..default()
        });
        app.insert_resource(assets);
        app.world_mut().spawn((
            MeshMaterial3d(top_handle.clone()),
            GltfMaterialName("Top".to_string()),
        ));
        app.world_mut().spawn((
            MeshMaterial3d(other_handle.clone()),
            GltfMaterialName("Other".to_string()),
        ));
        app.add_systems(Update, tint_top_materials);
        app.update();

        let assets = app.world().resource::<Assets<StandardMaterial>>();
        let top = assets.get(&top_handle).unwrap();
        assert_eq!(
            top.base_color,
            Color::srgb(1.0, 0.0, 0.0),
            "Top 材质应该被染红"
        );
        let other = assets.get(&other_handle).unwrap();
        assert_eq!(
            other.base_color,
            Color::srgb(0.0, 1.0, 0.0),
            "Other 材质不该被动"
        );
    }

    #[test]
    fn every_top_primitive_gets_tinted() {
        let mut app = App::new();
        let mut assets = Assets::<StandardMaterial>::default();
        let a = assets.add(StandardMaterial::default());
        let b = assets.add(StandardMaterial::default());
        app.insert_resource(assets);
        app.world_mut().spawn((
            MeshMaterial3d(a.clone()),
            GltfMaterialName("Top".to_string()),
        ));
        app.world_mut().spawn((
            MeshMaterial3d(b.clone()),
            GltfMaterialName("Top".to_string()),
        ));
        app.add_systems(Update, tint_top_materials);
        app.update();

        let assets = app.world().resource::<Assets<StandardMaterial>>();
        assert_eq!(
            assets.get(&a).unwrap().base_color,
            Color::srgb(1.0, 0.0, 0.0),
            "第一个 Top 图元也该被染红"
        );
        assert_eq!(
            assets.get(&b).unwrap().base_color,
            Color::srgb(1.0, 0.0, 0.0),
            "第二个 Top 图元也该被染红"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings test 2604`，看哪个断言先失败。
// 2. 检查 `name.0` 和字符串 "Top" 比较时的条件方向。
// 3. 改好后运行 `bevylings test 2604`，两个测试全绿就过关了。
