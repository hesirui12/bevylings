//! # 练习 05.01 —— 资源（Resource）与 Res：读全局数据
//!
//! 出处：https://bevy.org/learn/quick-start/getting-started/resources/
//!
//! ## 概念
//! 组件挂在"某个实体"上，而**资源（Resource）**是全游戏只有一份的全局数据，
//! 比如总分、存档、设置。用 `insert_resource` 放进世界，
//! 系统里用 `Res<Score>`（只读）或 `ResMut<Score>`（可写）取出来用。
//!
//! 本练习用一个 `Score` 资源存分数，并写一个纯函数计算"双倍分数"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0501` 查看现象（目前编译会报错），
//! 改正后运行 `bevylings test 0501` 让测试通过。
//!
//! 小贴士：`Score` 是"具名字段"结构体，字段叫 `points`，不是元组下标 `0`。

use bevy::prelude::*;

/// 全局分数。
#[derive(Resource)]
struct Score {
    points: u32,
}

/// 计算双倍分数。
fn doubled(score: &Score) -> u32 {
    score.points * 2
}

/// 每帧打印一次当前分数。
fn show_score(score: Res<Score>) {
    println!("当前分数：{}", score.points);
}

pub fn run() {
    App::new()
        .insert_resource(Score { points: 10 })
        .add_systems(Update, show_score)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_score() {
        assert_eq!(doubled(&Score { points: 10 }), 20, "10 的双倍是 20");
        assert_eq!(doubled(&Score { points: 0 }), 0, "0 的双倍是 0");
    }

    #[test]
    fn reads_points_field() {
        let mut app = App::new();
        app.insert_resource(Score { points: 5 });
        let score = app.world().resource::<Score>();
        assert_eq!(score.points, 5, "插入的分数应该能被读出来");
    }
}

// 提示：
// 1. 先运行 `bevylings run 0501`，看编译错误里"no field `0` on type `&Score`"。
// 2. 具名字段结构体要用 `score.points` 访问字段。
// 3. 改好后再运行 `bevylings test 0501`。
