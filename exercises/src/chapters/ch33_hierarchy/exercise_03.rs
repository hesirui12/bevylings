//! # 练习 33.03 —— 关系组件：自定义关系与反向查询
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/relationships/
//!
//! ## 概念
//! 除了内置的父子关系（`ChildOf` / `Children`），Bevy 还允许你定义**自己的关系**。
//! 做法是定义一对组件：
//! - `Targeting(Entity)`：记录"我盯上了谁"，它是关系的"源头"；
//! - `TargetedBy(Vec<Entity>)`：自动维护"谁在盯我"（反向关系）。
//!
//! 只要给组件加上 `#[relationship(...)]` 派生属性，Bevy 就会自动更新反向关系：
//! 你往某个实体上插入 `Targeting` 时，对方的 `TargetedBy` 会被自动追加。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3303` 查看现象，改正后运行 `bevylings test 3303` 让测试通过。
//!
//! 小贴士：统计时，"目标被几个人盯上"要看 `TargetedBy` 列表的长度 `len()`。

// I AM NOT DONE

use bevy::prelude::*;

/// 关系组件：这个实体正在盯着的目标。
#[derive(Component, Debug)]
#[relationship(relationship_target = TargetedBy)]
struct Targeting(Entity);

/// 反向关系组件：所有正在盯着这个实体的实体（由 Bevy 自动维护）。
#[derive(Component, Debug)]
#[relationship_target(relationship = Targeting)]
struct TargetedBy(Vec<Entity>);

/// 统计结果，方便测试观察。
#[derive(Resource, Default)]
struct TargetReport(usize);

fn setup(mut commands: Commands) {
    let alice = commands.spawn(Name::new("Alice")).id();
    let bob = commands.spawn((Name::new("Bob"), Targeting(alice))).id();
    let charlie = commands.spawn((Name::new("Charlie"), Targeting(bob))).id();
    commands.spawn((Name::new("Devon"), Targeting(charlie))).id();
    // 关系组件随时可以插入，反向关系会自动更新。
    commands.entity(bob).insert(Targeting(charlie));
}

/// 数一数"每个目标一共被几个人盯上"的总数。
fn count_targets(
    targeting_query: Query<&Targeting>,
    targeted_query: Query<&TargetedBy>,
    mut report: ResMut<TargetReport>,
) {
    let mut total = 0;
    for targeting in &targeting_query {
        let Ok(targeted_by) = targeted_query.get(targeting.0) else {
            continue;
        };
        // BUG: 目标被多少人盯上，就要把这个数量（列表长度）累加上去，
        // 这里却每次只加 1，导致总数偏小。
        total += 1;
    }
    report.0 = total;
}

pub fn run() {
    App::new()
        .init_resource::<TargetReport>()
        .add_systems(Startup, setup)
        .add_systems(Update, count_targets)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.init_resource::<TargetReport>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, count_targets);
        app
    }

    #[test]
    fn counts_all_target_relationships() {
        let mut app = build_app();
        app.update();
        let report = app.world().resource::<TargetReport>();
        // Bob→Alice(1 人盯), Charlie→Bob(1 人盯), Devon→Charlie(1 人盯),
        // Bob→Charlie(此时 Charlie 被 Devon 和 Bob 2 人盯) → 1+1+1+2 = 5
        assert_eq!(report.0, 5, "应该把每个目标被盯上的数量都加上");
    }

    #[test]
    fn reverse_relationship_is_auto_maintained() {
        let mut app = build_app();
        app.update();
        let mut q = app.world_mut().query::<(&Name, &TargetedBy)>();
        let charlie_targeted_by: Vec<usize> = q
            .iter(app.world())
            .filter(|(name, _)| name.as_str() == "Charlie")
            .map(|(_, targeted)| targeted.0.len())
            .collect();
        assert_eq!(charlie_targeted_by, vec![2], "Charlie 应该被 2 个实体盯着");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3303`，看总数为什么偏小。
// 2. 一个目标可能被好几个人盯，累加时要加上 `targeted_by.0.len()`。
// 3. `TargetedBy` 是自动维护的，你不需要自己往里面塞数据。
