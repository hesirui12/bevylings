//! # 练习 05.04 —— remove_resource：资源的生命周期
//!
//! 出处：https://bevy.org/learn/quick-start/getting-started/resources/
//!
//! ## 概念
//! 资源也有"生命周期"：`insert_resource` 放入、`remove_resource::<T>()` 移除。
//! 资源被移除后，用 `Res<T>` 访问会 panic；用 `Option<Res<T>>` 访问会得到 `None`。
//! 移除操作和生成实体一样，也要通过 `Commands` 排队执行。
//!
//! 本练习每帧推进一个回合，满 3 回合后把"新手加成"资源删掉。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0504` 查看现象，改正后运行 `bevylings test 0504` 让测试通过。
//!
//! 小贴士：`rounds.0 >= 3` 是"满 3 回合就删"；`>` 会晚一帧。

use bevy::prelude::*;

/// 新手加成：只有前几回合有效。
#[derive(Resource)]
struct Bonus(u32);

/// 当前回合数。
#[derive(Resource, Default)]
struct Rounds(u32);

/// 每帧推进一回合；满 3 回合后移除 Bonus 资源。
fn advance(mut commands: Commands, mut rounds: ResMut<Rounds>, bonus: Option<Res<Bonus>>) {
    rounds.0 += 1;
    if rounds.0 >= 3 {
        commands.remove_resource::<Bonus>();
    }
    if let Some(bonus) = bonus {
        println!("第 {} 回合，新手加成还有效：{}", rounds.0, bonus.0);
    }
}

pub fn run() {
    App::new()
        .insert_resource(Bonus(10))
        .init_resource::<Rounds>()
        .add_systems(Update, advance)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_removed_after_three_rounds() {
        let mut app = App::new();
        app.insert_resource(Bonus(10));
        app.init_resource::<Rounds>();
        app.add_systems(Update, advance);
        for _ in 0..3 {
            app.update();
        }
        assert!(
            app.world().get_resource::<Bonus>().is_none(),
            "满 3 回合后 Bonus 应该已被移除"
        );
    }

    #[test]
    fn bonus_kept_while_rounds_are_few() {
        let mut app = App::new();
        app.insert_resource(Bonus(10));
        app.init_resource::<Rounds>();
        app.add_systems(Update, advance);
        app.update();
        app.update();
        assert!(
            app.world().get_resource::<Bonus>().is_some(),
            "前两回合 Bonus 应该还在"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 0504`，数一数新手加成在第几个回合消失。
// 2. "满 3 回合"包括第 3 回合本身，想一想 `>` 和 `>=` 的区别。
// 3. 把 `>` 改成 `>=`，再运行 `bevylings test 0504`。
