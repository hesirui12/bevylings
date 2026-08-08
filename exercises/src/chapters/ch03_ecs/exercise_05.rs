//! # 练习 03.05 —— Query 过滤器：With / Without / Or
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/ecs_guide/
//!
//! ## 概念
//! 过滤条件写在 Query 的第二个参数里，用来"挑出子集"：
//! - `With<T>`：只挑**带** T 组件的实体；
//! - `Without<T>`：只挑**不带** T 组件的实体；
//! - `Or<(A, B)>`：带 A **或**带 B 都行（元组里可以放更多）。
//!
//! 本练习的世界里有人类、机器人和小狗。我们要问候人类和机器人，但**不**问候小狗。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0305` 查看现象，改正后运行 `bevylings test 0305` 让测试通过。
//!
//! 小贴士：`Or<(Person, Robot)>` 会把"人类或机器人"都选进来；小狗两种都不是。

// I AM NOT DONE

use bevy::prelude::*;

/// 人类。
#[derive(Component)]
struct Person;
/// 机器人。
#[derive(Component)]
struct Robot;
/// 小狗。
#[derive(Component)]
struct Dog;
/// 名字（谁都可能有名字）。
#[derive(Component)]
struct Name(String);

/// 记录问候语，方便测试观察。
#[derive(Resource, Default)]
struct Greetings(Vec<String>);

/// 问候所有"人"和"机器人"（不问候小狗）。
fn greet(query: Query<&Name, With<Person>>, mut greetings: ResMut<Greetings>) {
    // BUG: 过滤器只选了 Person，机器人（Robot）永远不会被问候。
    // 应该用 Or<(Person, Robot)> 把两类都包含进来。
    for name in &query {
        greetings.0.push(format!("你好，{}！", name.0));
    }
}

pub fn run() {
    App::new()
        .insert_resource(Greetings::default())
        .add_systems(Startup, setup)
        .add_systems(Update, greet)
        .run();
}

/// 启动时生成人类、机器人、小狗各一个。
fn setup(mut commands: Commands) {
    commands.spawn((Person, Name("小明".to_string())));
    commands.spawn((Robot, Name("R2".to_string())));
    commands.spawn((Dog, Name("旺财".to_string())));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(Greetings::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, greet);
        app
    }

    #[test]
    fn greets_person_and_robot() {
        let mut app = build_app();
        app.update();
        let greetings = app.world().resource::<Greetings>();
        assert!(
            greetings.0.iter().any(|g| g.contains("小明")),
            "人类应该被问候，实际 {:?}",
            greetings.0
        );
        assert!(
            greetings.0.iter().any(|g| g.contains("R2")),
            "机器人也应该被问候，实际 {:?}",
            greetings.0
        );
    }

    #[test]
    fn does_not_greet_dog() {
        let mut app = build_app();
        app.update();
        let greetings = app.world().resource::<Greetings>();
        assert!(
            !greetings.0.iter().any(|g| g.contains("旺财")),
            "小狗不应该被问候，实际 {:?}",
            greetings.0
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 0305`，观察哪些实体被问候了。
// 2. `With<Person>` 只含人类；`Or<(Person, Robot)>` 才含人类和机器人。
// 3. 把过滤器改成 `Or<(Person, Robot)>`，再运行 `bevylings test 0305`。
