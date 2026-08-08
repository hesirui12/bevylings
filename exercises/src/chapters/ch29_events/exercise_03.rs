//! # 练习 29.03 —— 观察者：add_observer 与 commands.trigger
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/observers/
//!
//! ## 概念
//! 观察者（Observer）是一种"监听事件"的系统：用 `add_observer` 注册后，
//! 一旦别处用 `commands.trigger(...)` 触发对应事件，观察者就会自动运行。
//! 它和消息（Message）的区别是：消息要显式地在系统里"读"，
//! 而观察者是"收到即触发"，不需要轮询。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2903` 观察得分，改正后运行
//! `bevylings test 2903` 让测试通过。
//!
//! 小贴士：观察者函数的第一个参数写 `On<事件类型>`，通过 Deref
//! 可以直接访问事件的字段，比如 `event.points`。

// I AM NOT DONE

use bevy::prelude::*;

/// 一个自定义事件：得分
#[derive(Event)]
struct ScoreEvent {
    points: u32,
}

/// 总分
#[derive(Resource, Default)]
struct Score(u32);

/// 记录是否已经触发过事件（保证测试可重复、确定）
#[derive(Resource, Default)]
struct Fired(bool);

/// 观察者：每次 ScoreEvent 被触发时，把分数加进 Score
fn on_score(event: On<ScoreEvent>, mut score: ResMut<Score>) {
    // BUG: 符号写反了：得分事件反而让总分减少。
    score.0 -= event.points;
    info!("score is now {}", score.0);
}

/// 第一次运行时触发两条得分事件（3 + 7）
fn fire_scores(
    mut commands: Commands,
    mut fired: ResMut<Fired>,
) {
    if !fired.0 {
        commands.trigger(ScoreEvent { points: 3 });
        commands.trigger(ScoreEvent { points: 7 });
        fired.0 = true;
    }
}

pub fn run() {
    App::new()
        .insert_resource(Score::default())
        .insert_resource(Fired::default())
        .add_observer(on_score)
        .add_systems(Update, fire_scores)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(Score::default());
        app.insert_resource(Fired::default());
        app.add_observer(on_score);
        app.add_systems(Update, fire_scores);
        app
    }

    #[test]
    fn triggered_events_increase_score() {
        let mut app = build_app();
        app.update();
        let score = app.world().resource::<Score>();
        assert_eq!(score.0, 10, "3 + 7 = 10 分");
    }

    #[test]
    fn observer_runs_only_when_triggered() {
        let mut app = build_app();
        app.update();
        app.update();
        let score = app.world().resource::<Score>();
        assert_eq!(score.0, 10, "没有新事件时，分数不应该变");
    }
}

// 提示：
// 1. 观察者的职责是"加分"，看看 `+=` 是不是被写成了 `-=`。
// 2. `Fired` 资源保证事件只触发一次，第二个测试验证"不会重复触发"。
// 3. 改好后运行 `bevylings test 2903`，两个测试都通过就过关了。
