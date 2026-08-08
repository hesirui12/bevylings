//! # 练习 32.02 —— Added<T> 与 RemovedComponents：组件的来与去
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/removal-detection/
//!
//! ## 概念
//! `Added<T>` 过滤器只在"组件刚被加进世界"的那一帧命中实体；
//! 而 `RemovedComponents<T>` 系统参数能收到"组件被移除"的通知，
//! 用来统计或做出反应（官方示例里用观察者把移除后的精灵变色）。
//! 这两个是观察"组件生命周期"最常见的两个入口。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3202` 查看现象，改正后运行 `bevylings test 3202` 让测试通过。
//!
//! 小贴士：`RemovedComponents<T>` 是系统参数，不是资源，不能包在 `Res` 里。

use bevy::prelude::*;

/// 被跟踪的组件。
#[derive(Component)]
struct MyComponent;

/// 统计"新增"和"移除"的次数。
#[derive(Resource, Default)]
struct LifecycleLog {
    added: u32,
    removed: u32,
}

/// 数一数这一帧新加了几个 MyComponent。
fn count_added(query: Query<(), Added<MyComponent>>, mut log: ResMut<LifecycleLog>) {
    log.added += query.iter().count() as u32;
}

/// 第 2 帧移除组件（用 Local 计数，保证测试可重复）。
fn remove_on_second_frame(
    mut frame: Local<u32>,
    query: Query<Entity, With<MyComponent>>,
    mut commands: Commands,
) {
    *frame += 1;
    if *frame == 2 {
        for entity in &query {
            commands.entity(entity).remove::<MyComponent>();
        }
    }
}

/// 数一数这一帧被移除的 MyComponent。
fn count_removed(
    mut removed: RemovedComponents<MyComponent>,
    mut log: ResMut<LifecycleLog>,
) {
    log.removed += removed.read().count() as u32;
}

pub fn run() {
    App::new()
        .insert_resource(LifecycleLog::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (count_added, remove_on_second_frame, count_removed))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(MyComponent);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(LifecycleLog::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, (count_added, remove_on_second_frame, count_removed));
        app
    }

    #[test]
    fn added_is_counted_once() {
        let mut app = build_app();
        app.update(); // 帧 1：组件刚加入
        app.update(); // 帧 2：发出移除命令（帧末生效）
        app.update(); // 帧 3：移除事件被读到
        let log = app.world().resource::<LifecycleLog>();
        assert_eq!(log.added, 1, "Added 只在组件刚加入的那一帧命中");
        assert_eq!(log.removed, 1, "移除事件应该被 RemovedComponents 读到");
    }

    #[test]
    fn no_removal_no_event() {
        let mut app = build_app();
        app.update();
        let log = app.world().resource::<LifecycleLog>();
        assert_eq!(log.added, 1);
        assert_eq!(log.removed, 0, "没移除就不该有移除事件");
    }
}

// 提示：
// 1. `Added<T>` 用在查询过滤器位置，`RemovedComponents<T>` 用在系统参数位置。
// 2. 系统参数直接写 `removed: RemovedComponents<MyComponent>`，去掉 Res 包装。
// 3. 修改后运行 `bevylings test 3202`，两个测试全绿就过关了。
