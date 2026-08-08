//! # 练习 05.02 —— ResMut：修改共享数据
//!
//! 出处：https://bevy.org/learn/quick-start/getting-started/resources/
//!
//! ## 概念
//! 只读资源用 `Res<T>`；**修改**资源要用 `ResMut<T>`（Mut = Mutable）。
//! 资源是"全局共享"的，每个系统拿到的都是同一份数据，改完其他系统马上能看到。
//!
//! 本练习每帧给总分加 10 分。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0502` 查看现象，改正后运行 `bevylings test 0502` 让测试通过。
//!
//! 小贴士：加分是 `+=`；写成 `-=` 分数会越加越少。

// I AM NOT DONE

use bevy::prelude::*;

/// 全局分数。
#[derive(Resource)]
struct Score(u32);

/// 给分数加上 amount。
fn add_points(score: &mut Score, amount: u32) {
    // BUG: 运算符写反了：这是"加分"，却用了减号，分数越加越少。
    score.0 -= amount;
}

/// 每帧加 10 分。
fn gain_points(mut score: ResMut<Score>) {
    add_points(&mut score, 10);
}

pub fn run() {
    App::new()
        .insert_resource(Score(100))
        .add_systems(Update, gain_points)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_points_to_score() {
        let mut score = Score(5);
        add_points(&mut score, 3);
        assert_eq!(score.0, 8, "5 + 3 = 8");
        add_points(&mut score, 2);
        assert_eq!(score.0, 10, "再加 2 变成 10");
    }

    #[test]
    fn gains_ten_per_frame() {
        let mut app = App::new();
        app.insert_resource(Score(100));
        app.add_systems(Update, gain_points);
        app.update();
        app.update();
        let score = app.world().resource::<Score>();
        assert_eq!(score.0, 120, "跑 2 帧、每帧 +10，100 应该变成 120，实际 {}", score.0);
    }
}

// 提示：
// 1. 先运行 `bevylings run 0502`，观察分数是涨还是跌。
// 2. `add_points` 想表达"加上 amount"，想想该用哪个运算符。
// 3. 改好后再运行 `bevylings test 0502`。
