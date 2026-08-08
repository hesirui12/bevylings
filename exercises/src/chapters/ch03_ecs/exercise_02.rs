//! # 练习 03.02 —— Commands：生成与删除实体（spawn / despawn）
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/delayed_commands/
//!
//! ## 概念
//! `Commands` 是"世界改造申请书"：系统运行中不能直接往世界里加实体，
//! 而是把"要生成/删除哪个实体"的命令排队，等这一帧所有系统跑完后统一执行。
//! `commands.spawn(组件)` 生成实体，`commands.entity(id).despawn()` 删除实体。
//!
//! 本练习让"方块"实体的数量自动保持：没到上限就补一个，满了就全部清空。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0302` 查看现象，改正后运行 `bevylings test 0302` 让测试通过。
//!
//! 小贴士：`Query<Entity, With<Block>>` 能查到所有方块实体的编号。

// I AM NOT DONE

use bevy::prelude::*;

/// 方块：一个空的标记组件。
#[derive(Component)]
struct Block;

/// 方块数量上限。
#[derive(Resource)]
struct MaxBlocks(usize);

/// 是否该再生成一个方块？规则：当前数量 < 上限 时生成。
fn should_spawn(count: usize, max: usize) -> bool {
    // BUG: 比较符号写反了，导致数量超过上限后反而不再生成、
    // 数量很少时却一直生成。正确逻辑是"还差得远（count < max）时才补"。
    count >= max
}

/// 管理方块数量：少了就补，满了就全删。
fn manage_blocks(
    mut commands: Commands,
    query: Query<Entity, With<Block>>,
    max: Res<MaxBlocks>,
) {
    let count = query.iter().count();
    if should_spawn(count, max.0) {
        commands.spawn(Block);
    } else {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}

pub fn run() {
    App::new()
        .insert_resource(MaxBlocks(3))
        .add_systems(Update, manage_blocks)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_spawn_only_below_max() {
        assert!(should_spawn(0, 3), "数量为 0，应该生成");
        assert!(should_spawn(2, 3), "还没满 3 个，应该生成");
        assert!(!should_spawn(3, 3), "满员了就不该再生成");
        assert!(!should_spawn(5, 3), "超员了更不该生成");
    }

    #[test]
    fn blocks_grow_then_shrink() {
        let mut app = App::new();
        app.insert_resource(MaxBlocks(3));
        app.add_systems(Update, manage_blocks);

        // 第 1~3 帧各补一个：达到上限 3 个。
        for _ in 0..3 {
            app.update();
        }
        let count = app.world_mut().query::<&Block>().iter(app.world()).count();
        assert_eq!(count, 3, "前三帧应该补满到 3 个方块，实际 {count}");

        // 第 4 帧满员：全部清空。
        app.update();
        let count = app.world_mut().query::<&Block>().iter(app.world()).count();
        assert_eq!(count, 0, "满员后下一帧应该全部删除，实际 {count}");
    }
}

// 提示：
// 1. 先运行 `bevylings run 0302`，看看方块数量的变化是否符合预期。
// 2. `should_spawn` 的职责是回答"该不该生成"，想一想：数量少的时候应该返回 true 还是 false？
// 3. 把 `>=` 改成正确的比较符号，再运行 `bevylings test 0302`。
