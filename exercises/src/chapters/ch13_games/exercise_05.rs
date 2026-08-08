//! # 练习 13.05 —— 胜利判定：清空砖块即过关
//!
//! 出处：https://bevy.org/examples/games/breakout/
//!
//! ## 概念
//! 打砖块的胜利条件很朴素：把场上所有砖块都打掉就赢了。
//! 官方示例里砖块被球碰到就 `despawn`（从世界移除），
//! 我们用 `Query<Entity, With<Brick>>` 找到砖块、每帧打掉一块（简化掉碰撞检测），
//! 再用 `Query<(), With<Brick>>` 数一数还剩几块。
//! 本章结尾把"清空砖块"和"胜利判定"这两个知识点串起来，组合成一个小游戏。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1305` 观察现象，改正后运行 `bevylings test 1305` 让测试通过。
//!
//! 小贴士：`Query<(), With<Brick>>` 不读任何数据，只用来"数数"。

// I AM NOT DONE

use bevy::prelude::*;

/// 砖块：被打掉就从世界移除。
#[derive(Component)]
struct Brick;

/// 打掉一块砖（简化：每帧消灭第一块剩下的砖，省略碰撞检测）。
fn destroy_brick(mut commands: Commands, query: Query<Entity, With<Brick>>) {
    if let Some(entity) = query.iter().next() {
        commands.entity(entity).despawn();
    }
}

/// 胜利条件：场上没有砖块了。
fn is_victory(bricks_left: usize) -> bool {
    // BUG: 比较写反了：还有砖的时候反而返回 true（胜利），
    // 应该是在"一块砖都不剩"时才胜利。
    bricks_left > 0
}

/// 每帧检查是否已经胜利。
fn check_win(query: Query<(), With<Brick>>) {
    if is_victory(query.iter().count()) {
        info!("🎉 胜利！所有砖块都被打掉了");
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_bricks))
        .add_systems(Update, (destroy_brick, check_win).chain())
        .run();
}

/// 生成相机。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// 开局生成三块砖。
fn spawn_bricks(mut commands: Commands) {
    for x in [-200.0, 0.0, 200.0] {
        commands.spawn((Brick, Transform::from_xyz(x, 200.0, 0.0)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn victory_when_all_bricks_destroyed() {
        assert!(is_victory(0), "一块砖都不剩时才算胜利");
    }

    #[test]
    fn not_victory_while_bricks_remain() {
        assert!(!is_victory(2), "还剩 2 块砖，不能算胜利");
        assert!(!is_victory(1), "还剩 1 块砖，也不能算胜利");
    }

    #[test]
    fn bricks_disappear_one_per_frame() {
        let mut app = App::new();
        app.add_systems(Startup, spawn_bricks);
        app.add_systems(Update, destroy_brick);
        app.update();
        app.update();
        let mut query = app.world_mut().query_filtered::<(), With<Brick>>();
        assert_eq!(query.iter(app.world()).count(), 1, "两帧后应只剩 1 块砖");
    }

    #[test]
    fn three_bricks_spawn_at_start() {
        let mut app = App::new();
        app.add_systems(Startup, spawn_bricks);
        app.update();
        let mut query = app.world_mut().query_filtered::<(), With<Brick>>();
        assert_eq!(query.iter(app.world()).count(), 3, "开局应有 3 块砖");
    }
}

// 提示：
// 1. 想清楚"几块砖的时候才算胜利"：是 0、1、还是负数？
// 2. `>` 换成正确的比较符即可，改完跑 `bevylings test 1305`。
// 3. 所有测试全绿后，把 `destroy_brick` 换成真实的球-砖碰撞就是你自己的打砖块了。
