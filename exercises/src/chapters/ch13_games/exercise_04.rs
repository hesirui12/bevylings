//! # 练习 13.04 —— 吃蛋糕：踩到就加分
//!
//! 出处：https://bevy.org/examples/games/alien-cake-addict/
//!
//! ## 概念
//! 官方示例里外星人在格子棋盘上走，蛋糕会随机出现在某个格子上，
//! 玩家踩到蛋糕就吃掉它：分数 +2、吃掉数量 +1、蛋糕消失。
//! 我们把它简化为一个 `Game` 资源，里面同时记着玩家和蛋糕的格子坐标。
//! 判断"踩没踩到"其实就是一个二维坐标比较：行和列都相等才算数。
//! 吃掉蛋糕的"结算"也抽成了独立函数，方便单独测试。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1304` 观察现象，改正后运行 `bevylings test 1304` 让测试通过。
//!
//! 小贴士：`i` 和 `j` 是两个不同的坐标轴，比较时千万别混着用。

// I AM NOT DONE

use bevy::prelude::*;

/// 游戏全局状态：玩家、蛋糕的位置与分数。
#[derive(Resource, Default)]
struct Game {
    score: i32,
    cake_eaten: u32,
    player_i: usize,
    player_j: usize,
    bonus_i: usize,
    bonus_j: usize,
    bonus_alive: bool,
}

/// 玩家是否正站在蛋糕上？
fn is_on_bonus(game: &Game) -> bool {
    game.bonus_alive
        && game.player_i == game.bonus_i
        // BUG: 玩家第 j 列应该和蛋糕的第 j 列比较，
        // 这里误写成了蛋糕的 i，导致站错位置也能"吃到"。
        && game.player_j == game.bonus_i
}

/// 吃掉蛋糕：加分、计数、蛋糕消失。
fn on_eat(game: &mut Game) {
    game.score += 2;
    game.cake_eaten += 1;
    game.bonus_alive = false;
}

/// 每帧检查：踩到蛋糕就吃掉。
fn eat_cake(mut game: ResMut<Game>) {
    if is_on_bonus(&game) {
        on_eat(&mut game);
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Game::default())
        .add_systems(Startup, setup)
        .add_systems(Update, eat_cake)
        .run();
}

/// 生成相机。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stands_on_cake() {
        let game = Game {
            player_i: 2,
            player_j: 5,
            bonus_i: 2,
            bonus_j: 5,
            bonus_alive: true,
            ..default()
        };
        assert!(is_on_bonus(&game), "玩家和蛋糕在同一个格子，应该能吃到");
    }

    #[test]
    fn no_cake_on_different_row() {
        let game = Game {
            player_i: 2,
            player_j: 5,
            bonus_i: 3,
            bonus_j: 5,
            bonus_alive: true,
            ..default()
        };
        assert!(!is_on_bonus(&game), "行不同，不该吃到");
    }

    #[test]
    fn eaten_cake_disappears() {
        let game = Game {
            player_i: 2,
            player_j: 5,
            bonus_i: 2,
            bonus_j: 5,
            bonus_alive: false,
            ..default()
        };
        assert!(!is_on_bonus(&game), "蛋糕已被吃掉，不再生效");
    }

    #[test]
    fn eating_rewards_score() {
        let mut game = Game {
            score: 0,
            cake_eaten: 0,
            bonus_alive: true,
            ..default()
        };
        on_eat(&mut game);
        assert_eq!(game.score, 2, "吃一个蛋糕 +2 分");
        assert_eq!(game.cake_eaten, 1, "吃掉数量 +1");
        assert!(!game.bonus_alive, "蛋糕消失");
    }
}

// 提示：
// 1. 比较坐标时，玩家的两个坐标要分别和蛋糕的两个坐标比。
// 2. 检查 BUG 行里第二个比较的右半边：是 `bonus_i` 还是 `bonus_j`？
// 3. 改完运行 `bevylings test 1304`，四个测试全绿就过关。
