//! # 练习 03.01 —— 实体、组件与查询（Entity, Component, Query）
//!
//! 出处：https://bevy.org/learn/quick-start/getting-started/ecs/
//!
//! ## 概念
//! Bevy 用 ECS（实体组件系统）组织游戏数据：
//! - **实体（Entity）**：世界里的一个"东西"，本身只是编号。
//! - **组件（Component）**：挂在实体上的数据，比如名字、位置。
//! - **查询（Query）**：一次取出"满足条件"的所有实体的组件。
//!
//! 我们想给世界里的每个 `Person`（人）打招呼。小狗也有名字，
//! 但它不是 Person，不应该被当作人来问候。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0301`，改正后运行 `bevylings test 0301` 让测试通过。
//!
//! 小贴士：`Query<&Name>` 会取到**所有**带 Name 的实体；
//! 想筛选出"带 Name 并且有 Person"的实体，需要 `With<Person>` 过滤器。

// I AM NOT DONE

use bevy::prelude::*;

/// 人。只有挂了 Person 组件的实体才算"人"。
#[derive(Component)]
struct Person;

/// 名字。小狗也可以有名字，所以名字单独做成组件。
#[derive(Component)]
struct Name(String);

/// 记录系统产生的问候语，方便测试观察。
#[derive(Resource, Default)]
struct Greetings(Vec<String>);

/// 给每个"人"打招呼。
fn greet_people(
    mut greetings: ResMut<Greetings>,
    // BUG: 这里少了一个过滤器，导致小狗也被当成"人"来打招呼。
    query: Query<&Name>,
) {
    for name in &query {
        greetings.0.push(format!("hello {}!", name.0));
    }
}

pub fn run() {
    App::new()
        .insert_resource(Greetings::default())
        .add_systems(Startup, add_people)
        .add_systems(Update, greet_people)
        .run();
}

/// 启动时生成两个人和一只狗。
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Name("Fido".to_string()))); // 狗：只有名字，没有 Person
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(Greetings::default());
        app.add_systems(Startup, add_people);
        app.add_systems(Update, greet_people);
        app
    }

    #[test]
    fn greets_all_people() {
        let mut app = build_app();
        app.update();
        let greetings = app.world().resource::<Greetings>();
        assert_eq!(
            greetings.0,
            vec![
                "hello Elaina Proctor!".to_string(),
                "hello Renzo Hume!".to_string(),
            ],
            "应该只问候两个 Person"
        );
    }

    #[test]
    fn does_not_greet_dog() {
        let mut app = build_app();
        app.update();
        let greetings = app.world().resource::<Greetings>();
        assert!(
            !greetings.0.iter().any(|g| g.contains("Fido")),
            "小狗没有 Person 组件，不应该被问候，实际: {:?}",
            greetings.0
        );
    }
}

// 提示：
// 1. `Query<&Name>` 与 `Query<&Name, With<Person>>` 有什么区别？查一下 bevy 的 Query 过滤器。
// 2. `With<T>` 表示"只要带 T 组件的实体"，本身不读取 T 的数据。
// 3. 修改后运行 `bevylings test 0301`，两个测试都通过就过关了。
