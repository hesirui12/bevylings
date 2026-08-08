//! # 练习 28.02 —— 热重载：监听 AssetEvent
//!
//! 出处：https://bevy.org/examples-webgpu/asset/hot-asset-reloading/
//!
//! ## 概念
//! "热重载"指文件在磁盘上被修改后，游戏不用重启就能看到新内容。
//! 桌面平台靠 `file_watcher` feature 自动检测文件变化，并发出
//! `AssetEvent` 消息。本练习不依赖文件监听，用最朴素的写法演示原理：
//! 用 `MessageReader<AssetEvent<Mesh>>` 读取资产事件，
//! 其中 `AssetEvent::Modified` 就代表"资产内容变了"。
//! （Bevy 0.19 把事件改名为"消息"：读端是 `MessageReader`，写端是 `MessageWriter`。）
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2802` 查看现象，改正后运行 `bevylings test 2802` 让测试通过。
//!
//! 小贴士：`AssetEvent::Modified { id }` 的 `id` 告诉我们哪份资产变了。

// I AM NOT DONE

use bevy::prelude::*;

/// 记录触发热重载的次数。
#[derive(Resource, Default)]
struct ReloadCount(u32);

/// 判断这条资产事件是不是"资产被修改"。
fn is_modification(event: &AssetEvent<Mesh>) -> bool {
    matches!(event, AssetEvent::Modified { .. })
}

/// 监听资产事件：每当有网格被修改，计数加一。
// BUG: 这里想"读"事件，系统参数却写成了 `MessageWriter`。
// 写端只有 `write()` 方法、没有 `read()` 方法，所以编译不过。
fn track_reloads(
    mut events: MessageWriter<AssetEvent<Mesh>>,
    mut count: ResMut<ReloadCount>,
) {
    for event in events.read() {
        if is_modification(event) {
            count.0 += 1;
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ReloadCount::default())
        .add_systems(Startup, setup)
        .add_systems(Update, track_reloads)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 官方示例加载的是 torus.gltf 场景，文件改动会被自动重载；
    // 这里用普通 load 就好，事件由 track_reloads 监听。
    let scene_handle =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/torus/torus.gltf"));
    commands.spawn(WorldAssetRoot(scene_handle));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 5.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 造一个"确实存在于某份 Assets 集合里"的资产 id。
    fn make_id() -> AssetId<Mesh> {
        let mut assets = Assets::<Mesh>::default();
        assets.add(Mesh::from(Cone::new(1.0, 2.0))).id()
    }

    #[test]
    fn modification_is_reload() {
        let id = make_id();
        assert!(is_modification(&AssetEvent::Modified { id }));
    }

    #[test]
    fn other_events_are_not_reload() {
        let id = make_id();
        assert!(!is_modification(&AssetEvent::LoadedWithDependencies { id }));
    }

    #[test]
    fn reader_system_counts_modifications() {
        let mut world = World::new();
        world.init_resource::<Messages<AssetEvent<Mesh>>>();
        world.init_resource::<ReloadCount>();

        let id = make_id();
        world.write_message(AssetEvent::Modified { id });

        let mut schedule = Schedule::default();
        schedule.add_systems(track_reloads);
        schedule.run(&mut world);

        assert_eq!(world.resource::<ReloadCount>().0, 1);
    }
}

// 提示：
// 1. 读事件要用 `MessageReader`，写事件才用 `MessageWriter`。
// 2. 把参数类型改成读端之后，`events.read()` 就能编译了。
// 3. 修改后运行 `bevylings test 2802`，三个测试全绿就过关了。
