//! # 练习 32.03 —— EntityDisabled：禁用实体
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/entity-disabling/
//!
//! ## 概念
//! 想暂时"藏起"一个实体，除了用 `Visibility` 隐藏画面，还可以给它挂上
//! `Disabled` 组件（来自 `bevy::ecs::entity_disabling`）。被禁用的实体会被
//! **所有默认查询跳过**——它仍然存在，只是普通系统看不到它。
//! 想找到禁用实体，必须显式把 `Disabled` 写进查询，比如 `Query<Entity, With<Disabled>>`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3203` 查看现象，改正后运行 `bevylings test 3203` 让测试通过。
//!
//! 小贴士：重新启用 = 移除 `Disabled` 组件。注意：`Query<Entity>` 本身也会跳过禁用实体！

use bevy::ecs::entity_disabling::Disabled;
use bevy::prelude::*;

/// 名字组件（用来观察实体是否可见）。
#[derive(Component)]
struct MyName(String);

/// 记录"默认查询能看到的名字"。
#[derive(Resource, Default)]
struct NameLog(Vec<String>);

/// 收集默认查询能看到的所有名字。
fn list_names(query: Query<&MyName>, mut log: ResMut<NameLog>) {
    log.0 = query.iter().map(|name| name.0.clone()).collect();
}

/// 重新启用所有被禁用的实体（移除 Disabled 组件）。
fn reenable_disabled(mut commands: Commands, disabled: Query<Entity, With<Disabled>>) {
    for entity in &disabled {
        commands.entity(entity).remove::<Disabled>();
    }
}

pub fn run() {
    App::new()
        .insert_resource(NameLog::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (list_names, reenable_disabled))
        .run();
}

/// 生成两个实体：一个一开始就被禁用，一个正常。
fn setup(mut commands: Commands) {
    commands.spawn((MyName("hidden".into()), Disabled));
    commands.spawn(MyName("visible".into()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(NameLog::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, (list_names, reenable_disabled));
        app
    }

    #[test]
    fn disabled_entities_are_skipped_by_default() {
        let mut app = build_app();
        app.update();
        let log = app.world().resource::<NameLog>();
        assert_eq!(
            log.0,
            vec!["visible".to_string()],
            "默认查询应该跳过禁用实体"
        );
    }

    #[test]
    fn reenable_lets_it_be_seen_again() {
        let mut app = build_app();
        app.update(); // 帧 1：发出"移除 Disabled"的命令
        app.update(); // 帧 2：命令生效，实体重新可见
        let log = app.world().resource::<NameLog>();
        assert!(
            log.0.iter().any(|name| name == "hidden"),
            "重新启用后应该能再次看到它，实际: {:?}",
            log.0
        );
    }

    #[test]
    fn explicit_filter_can_find_disabled() {
        let mut app = App::new();
        app.add_systems(Startup, setup);
        app.update();
        let mut query = app.world_mut().query_filtered::<Entity, With<Disabled>>();
        assert_eq!(
            query.iter(app.world()).count(),
            1,
            "显式 With<Disabled> 能找到禁用实体"
        );
    }
}

// 提示：
// 1. 默认查询会跳过带 `Disabled` 的实体——这正是"禁用"的含义。
// 2. 想找到禁用实体，查询里要显式写 `With<Disabled>`。
// 3. 修改后运行 `bevylings test 3203`，三个测试全绿就过关了。
