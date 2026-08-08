//! # 练习 32.01 —— Changed<T>：组件变化才处理
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/change-detection/
//!
//! ## 概念
//! 很多系统关心的是"组件**什么时候变了**"，而不是每一帧都把数据重新处理一遍。
//! Bevy 会记录每个组件最近一次被修改的时间，`Changed<T>` 查询过滤器
//! 只返回"这一帧刚变化过"的实体。
//!
//! 本练习用一个 `ChangeLog` 记录处理过的值：只有组件真的变化时
//! 才写入日志，没变化的帧不写。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3201` 查看现象，改正后运行 `bevylings test 3201` 让测试通过。
//!
//! 小贴士：`Query<&T, Changed<T>>` 和 `Query<&T>` 的区别，就在那一个 `Changed` 过滤器上。

use bevy::prelude::*;

/// 被跟踪的组件：一个浮点数。
#[derive(Component, PartialEq, Debug)]
struct MyComponent(f32);

/// 记录"变化被处理"的日志。
#[derive(Resource, Default)]
struct ChangeLog(Vec<String>);

/// 只处理"这一帧变化过"的组件，把新值记进日志。
fn track_changes(
    query: Query<&MyComponent, Changed<MyComponent>>,
    mut log: ResMut<ChangeLog>,
) {
    for component in &query {
        log.0.push(format!("changed to {}", component.0));
    }
}

/// 只在第 1 帧修改组件（用 Local 计数保证测试可重复）。
fn modify_once(mut query: Query<&mut MyComponent>, mut frame: Local<u32>) {
    *frame += 1;
    if *frame == 1 {
        for mut component in &mut query {
            component.0 += 10.0;
        }
    }
}

pub fn run() {
    App::new()
        .insert_resource(ChangeLog::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (modify_once, track_changes).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(MyComponent(0.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(ChangeLog::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, (modify_once, track_changes).chain());
        app
    }

    #[test]
    fn logs_only_when_changed() {
        let mut app = build_app();
        app.update(); // 帧 1：修改一次，记录一次
        app.update(); // 帧 2：没有修改
        app.update(); // 帧 3：没有修改
        let log = app.world().resource::<ChangeLog>();
        assert_eq!(log.0.len(), 1, "3 帧里只改了 1 次，应该只有 1 条日志");
        assert_eq!(log.0[0], "changed to 10");
    }

    #[test]
    fn unchanged_frame_adds_nothing() {
        let mut app = build_app();
        app.update();
        let log = app.world().resource::<ChangeLog>();
        assert_eq!(log.0.len(), 1);

        app.update(); // 没有修改的一帧
        let log = app.world().resource::<ChangeLog>();
        assert_eq!(log.0.len(), 1, "没变化的帧不应该再记录");
    }
}

// 提示：
// 1. `Query<&MyComponent>` 会遍历到所有实体，不管有没有变化。
// 2. `Changed<MyComponent>` 过滤器能把查询限定在"这一帧变化过"的实体上。
// 3. 给查询补上过滤器，再运行 `bevylings test 3201`。
