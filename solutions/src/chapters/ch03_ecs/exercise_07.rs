//! # 练习 03.07 —— iter_combinations：两两配对
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/iter_combinations/
//!
//! ## 概念
//! 有时需要让实体"两两配对"（比如计算任意两点之间的距离）。
//! 普通 `for` 遍历一次只能看到一个实体；`query.iter_combinations::<K>()`
//! 一次给出 K 个不重复的实体组合。K=2 就是所有"无序点对"，
//! 3 个点正好有 3 对（AB、AC、BC）。
//!
//! 本练习有 3 个点，把每对点的距离加起来得到"总距离"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0307` 查看现象（目前编译会报错），
//! 改正后运行 `bevylings test 0307` 让测试通过。
//!
//! 小贴士：组合大小 K 写在方括号里：`[&Pos; 2]` 表示"每次拿 2 个"。

use bevy::prelude::*;

/// 平面上的一个点。
#[derive(Component)]
struct Pos(Vec2);

/// 计算所有点两两之间的距离之和。
fn total_pairwise_distance(query: &Query<&Pos>) -> f32 {
    let mut total = 0.0;
    for [a, b] in query.iter_combinations::<2>() {
        total += a.0.distance(b.0);
    }
    total
}

/// 记录总距离，方便测试观察。
#[derive(Resource, Default)]
struct TotalDistance(f32);

/// 每帧把总距离写进资源。
fn record_total(mut total: ResMut<TotalDistance>, query: Query<&Pos>) {
    total.0 = total_pairwise_distance(&query);
}

pub fn run() {
    App::new()
        .insert_resource(TotalDistance::default())
        .add_systems(Startup, setup)
        .add_systems(Update, record_total)
        .run();
}

/// 启动时生成 3 个点：间距分别是 3、4、5。
fn setup(mut commands: Commands) {
    commands.spawn(Pos(Vec2::new(0.0, 0.0)));
    commands.spawn(Pos(Vec2::new(3.0, 0.0)));
    commands.spawn(Pos(Vec2::new(0.0, 4.0)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_points_have_three_pairs() {
        // 3 个点两两配对共 3 对：距离 3 + 4 + 5 = 12。
        let mut app = App::new();
        app.insert_resource(TotalDistance::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, record_total);
        app.update();
        let total = app.world().resource::<TotalDistance>();
        assert!(
            (total.0 - 12.0).abs() < 1e-5,
            "距离和应该是 3+4+5=12，实际 {}",
            total.0
        );
    }

    #[test]
    fn two_points_give_single_pair() {
        let mut app = App::new();
        app.insert_resource(TotalDistance::default());
        app.world_mut().spawn(Pos(Vec2::new(0.0, 0.0)));
        app.world_mut().spawn(Pos(Vec2::new(6.0, 8.0)));
        app.add_systems(Update, record_total);
        app.update();
        let total = app.world().resource::<TotalDistance>();
        assert!(
            (total.0 - 10.0).abs() < 1e-5,
            "两个点只有一对，距离应该是 10，实际 {}",
            total.0
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 0307`，看编译错误里"pattern of length 2 cannot match array of length 3"。
// 2. `iter_combinations::<[&Pos; K]>()` 里 K 是"每次取几个"。
// 3. 距离是两点之间，把 K 改成 2，再运行 `bevylings test 0307`。
