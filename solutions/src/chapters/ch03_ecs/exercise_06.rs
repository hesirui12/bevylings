//! # 练习 03.06 —— 实体关系：把 Entity 编号存进组件
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/ecs_guide/
//!
//! ## 概念
//! 实体之间可以"互相认识"：把对方实体的编号（`Entity`）存进自己的组件里，
//! 就建立了一条关系，比如"船 ← 船长"。
//! 需要时用 `query.get(entity)` 顺着编号找到对方实体、读出它的组件。
//!
//! 本练习的世界里有一条船和一个船长，我们要打印"船的船长是谁"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0306` 查看现象，改正后运行 `bevylings test 0306` 让测试通过。
//!
//! 小贴士：`Captain(Entity)` 组件里存的是**船长**的编号，不是船自己的编号。

use bevy::prelude::*;

/// 名字。
#[derive(Component)]
struct Name(String);

/// 船长：存船长实体的编号，表示"这艘船的船长是谁"。
#[derive(Component)]
struct Captain(Entity);

/// 记录船员名单，方便测试观察。
#[derive(Resource, Default)]
struct CrewLog(Vec<String>);

/// 打印每条船的船长是谁。
fn list_crews(
    mut log: ResMut<CrewLog>,
    ships: Query<(Entity, &Name, &Captain)>,
    captains: Query<&Name>,
) {
    for (_ship_id, ship_name, captain) in &ships {
        if let Ok(captain_name) = captains.get(captain.0) {
            log.0.push(format!("{} 的船长是 {}", ship_name.0, captain_name.0));
        }
    }
}

pub fn run() {
    App::new()
        .insert_resource(CrewLog::default())
        .add_systems(Startup, setup)
        .add_systems(Update, list_crews)
        .run();
}

/// 启动时生成船长和船，并把船长的编号交给船。
fn setup(mut commands: Commands) {
    let captain = commands.spawn(Name("郑和".to_string())).id();
    commands.spawn((Name("宝船".to_string()), Captain(captain)));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(CrewLog::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, list_crews);
        app
    }

    #[test]
    fn finds_captain_by_entity() {
        let mut app = build_app();
        app.update();
        let log = app.world().resource::<CrewLog>();
        assert_eq!(
            log.0,
            vec!["宝船 的船长是 郑和".to_string()],
            "应该顺着船长编号找到郑和，实际 {:?}",
            log.0
        );
    }

    #[test]
    fn logs_exactly_one_crew() {
        let mut app = build_app();
        app.update();
        let log = app.world().resource::<CrewLog>();
        assert_eq!(log.0.len(), 1, "只有一条船，应该只有一条记录，实际 {:?}", log.0);
    }
}

// 提示：
// 1. 先运行 `bevylings run 0306`，观察打印的船长名字是不是自己。
// 2. `captains` 查询的是"有 Name 的实体"，船和船长都有 Name —— 区别在传给 get 的编号。
// 3. 应该用 `captain.0`（船长编号）而不是 `ship_id`（船自己的编号）。
