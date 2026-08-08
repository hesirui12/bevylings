//! # 练习 06.04 —— 计分资源：命中砖块 +1
//!
//! 出处：https://bevy.org/examples/games/breakout/
//!
//! ## 概念
//! 分数是全局共享的一份数据，适合做成**资源**（Resource）：
//! `#[derive(Resource)] struct Score(usize)`。
//! 官方示例用 `Deref` / `DerefMut` 把 `Score` 伪装成 `usize`，
//! 这样 `**score += 1` 就能直接给分数加 1（外面一层 `*` 穿过 `ResMut`，
//! 里面一层 `*` 穿过 `DerefMut`）。
//! 简化版里省略了球和砖块的碰撞检测，每一帧都当作"命中了一块砖"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0604` 查看现象（目前会编译报错），
//! 改正后运行 `bevylings test 0604` 让测试通过。
//!
//! 小贴士：`ResMut<T>` 只是 T 的"可变句柄"，想改 T 内部的值要先 `*` 解引用。

// I AM NOT DONE

use bevy::prelude::*;

/// 分数：一个全局资源，里面只有一个 usize。
#[derive(Resource, Deref, DerefMut, Default)]
struct Score(usize);

/// 命中一块砖：分数 +1（简化演示：每帧当作命中一次）。
fn on_brick_hit(mut score: ResMut<Score>) {
    // BUG: `score` 是 `ResMut<Score>`，要先 `*` 一层变成 `Score`；
    // 而 `Score` 本身还实现了 `DerefMut`，所以一共要两层解引用 `**score`。
    // 这里只写了一层，类型对不上。
    *score += 1;
}

/// 把分数打印出来看看。
fn print_score(score: Res<Score>) {
    info!("Score: {}", score.0);
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(Update, (on_brick_hit, print_score).chain())
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
    fn score_increases_each_frame() {
        let mut app = App::new();
        app.insert_resource(Score(0));
        app.add_systems(Update, on_brick_hit);
        app.update();
        app.update();
        let score = app.world().resource::<Score>();
        assert_eq!(score.0, 2, "每帧命中一块砖，两帧后分数应为 2");
    }

    #[test]
    fn deref_mut_updates_inner_value() {
        let mut score = Score(10);
        *score += 5; // DerefMut 让我们可以直接修改内部的 usize
        assert_eq!(score.0, 15);
    }

    #[test]
    fn default_score_is_zero() {
        assert_eq!(Score::default().0, 0);
    }
}

// 提示：
// 1. 运行 `bevylings run 0604`，看编译错误里说 "cannot add integer to `Score`"。
// 2. 想清楚这里一共有几层"包装"：ResMut 一层、Score 自己一层。
// 3. 官方源码里就是 `**score += 1;`，参考 https://bevy.org/examples/games/breakout/
