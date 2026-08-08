//! # 练习 34.04 —— 自定义调度：Schedules / ScheduleLabel
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/custom-schedule/
//!
//! ## 概念
//! Bevy 自带 `Update`、`Startup`、`Last` 等内置调度（schedule）。
//! 你也可以自己定义调度：用 `#[derive(ScheduleLabel)]` 声明一个"标签"，
//! 然后把系统挂上去。但注意：**注册了调度并不代表它会被执行**，
//! 还要通过 `MainScheduleOrder` 把它插进主流程，并声明执行顺序
//! （`insert_after` 在某个调度之后、`insert_before` 在之前）。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3404` 查看现象，改正后运行 `bevylings test 3404` 让测试通过。
//!
//! 小贴士：`Sequence` 资源记录系统们实际执行的先后顺序，测试就靠它验证顺序。

use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

/// 自定义调度标签：一个"晚一点跑"的更新。
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct MyUpdate;

/// 记录执行顺序，方便测试观察。
#[derive(Resource, Default)]
struct Sequence(Vec<&'static str>);

/// 挂在 Update 上的系统。
fn update_system(mut seq: ResMut<Sequence>) {
    seq.0.push("Update");
}

/// 挂在自定义调度 MyUpdate 上的系统。
fn my_update_system(mut seq: ResMut<Sequence>) {
    seq.0.push("MyUpdate");
}

/// 注册自定义调度，并把它插进主流程（在 Update 之后跑）。
fn wire_schedule(app: &mut App) {
    app.add_schedule(Schedule::new(MyUpdate));
    let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
    order.insert_after(Update, MyUpdate);
}

pub fn run() {
    let mut app = App::new();
    app.init_resource::<Sequence>();
    wire_schedule(&mut app);
    app.add_systems(Update, update_system)
        .add_systems(MyUpdate, my_update_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app(with_wire: bool) -> App {
        let mut app = App::new();
        app.init_resource::<Sequence>();
        if with_wire {
            // 先注册调度，再往调度里加系统
            wire_schedule(&mut app);
        }
        app.add_systems(Update, update_system);
        app.add_systems(MyUpdate, my_update_system);
        app
    }

    #[test]
    fn custom_schedule_runs_after_update() {
        let mut app = build_app(true);
        app.update();
        let seq = &app.world().resource::<Sequence>().0;
        assert_eq!(
            seq,
            &vec!["Update", "MyUpdate"],
            "MyUpdate 应该插在 Update 之后执行"
        );
    }

    #[test]
    fn unregistered_schedule_does_not_run() {
        let mut app = build_app(false); // 不调用 wire_schedule
        app.update();
        let seq = &app.world().resource::<Sequence>().0;
        assert_eq!(seq, &vec!["Update"], "没插入主流程的调度不会被自动执行");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3404`，看第一个测试期望的顺序是什么。
// 2. `insert_after(A, B)` 让 B 在 A 之后；`insert_before(A, B)` 让 B 在 A 之前。
// 3. 顺序改对后，两个测试都绿就过关。
