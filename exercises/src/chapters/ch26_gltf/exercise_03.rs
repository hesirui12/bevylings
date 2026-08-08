//! # 练习 26.03 —— 更新场景：遍历并移动子物体
//!
//! 出处：https://bevy.org/examples-webgpu/gltf/update_gltf_scene/
//!
//! ## 概念
//! 用 `WorldAssetRoot` 生成的场景，模型实体是根实体的**子孙**。
//! 想整体操控它们，先找到带某个标记组件的根，然后用
//! `children.iter_descendants(根)` 遍历所有子孙，逐个修改它们的
//! `Transform`——这样完全不用知道模型里到底有多少零件。
//!
//! 官方示例让场景里的零件随时间做**余弦**起伏：t = 0 时
//! z 正好是 cos(0)/20 = 1/20 = 0.05。系统每一帧都重算位置。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2603` 观察现象，改正后运行 `bevylings test 2603` 让测试通过。
//!
//! 小贴士：`iter_descendants` 从 `Query<&Children>` 上调用，
//! 返回的是根实体之外的所有后代。

// I AM NOT DONE

use bevy::prelude::*;

/// 标记"这个场景的零件要被移动"。
#[derive(Component)]
struct MovedScene;

fn setup(mut commands: Commands) {
    // 一棵"被标记"的场景树：根 + 一个子零件
    let root = commands.spawn(MovedScene).id();
    let part = commands.spawn(Transform::from_xyz(1.0, 2.0, 3.0)).id();
    commands.entity(root).add_child(part);

    // 一棵没被标记的场景树：它的零件不该被移动
    let other_root = commands.spawn_empty().id();
    let other_part = commands.spawn(Transform::from_xyz(1.0, 2.0, 3.0)).id();
    commands.entity(other_root).add_child(other_part);

    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-0.5, 0.9, 1.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// 每帧让 MovedScene 的所有子孙零件做余弦起伏。
fn move_scene_entities(
    time: Res<Time>,
    moved_scene: Query<Entity, With<MovedScene>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
) {
    for root in &moved_scene {
        for entity in children.iter_descendants(root) {
            if let Ok(mut transform) = transforms.get_mut(entity) {
                // BUG: z 应该随时间做余弦运动（t=0 时 z=1/20），
                // 这里却写成了正弦（t=0 时 z=0），起伏节奏完全不对。
                transform.translation = Vec3::new(
                    0.0,
                    0.0,
                    time.elapsed_secs().sin() / 20.0,
                );
            }
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_scene_entities)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_tagged_scene_entities_move() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_scene_entities);
        app.update(); // Startup + 第一帧（elapsed 约等于 0）

        // 第一帧里 z 应该接近 cos(0)/20 = 0.05 的实体，只能有一个
        let moved: Vec<Vec3> = app
            .world_mut()
            .query::<&Transform>()
            .iter(app.world())
            .filter(|t| (t.translation.z - 0.05).abs() < 1e-2)
            .map(|t| t.translation)
            .collect();
        assert_eq!(moved.len(), 1, "只有带 MovedScene 标记的树的零件会被移动");
        assert!(
            (moved[0].z - 0.05).abs() < 1e-2,
            "z 应约为 cos(0)/20 = 0.05，实际 {}",
            moved[0].z
        );
    }

    #[test]
    fn untagged_scene_entities_keep_position() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_scene_entities);
        app.update();

        // 没被标记的树的零件保持原位置 (1, 2, 3)，只剩它一个还在那里
        let kept: Vec<Vec3> = app
            .world_mut()
            .query::<&Transform>()
            .iter(app.world())
            .filter(|t| t.translation == Vec3::new(1.0, 2.0, 3.0))
            .map(|t| t.translation)
            .collect();
        assert_eq!(kept.len(), 1, "没被标记的零件应该原地不动");
    }
}

// 提示：
// 1. 先运行 `bevylings run 2603`，看看零件的起伏是不是"从 0 开始"。
// 2. t = 0 时 sin(0) = 0，cos(0) = 1；题目要求 z 的起点是 1/20。
// 3. 改好后运行 `bevylings test 2603`，两个测试全绿就过关了。
