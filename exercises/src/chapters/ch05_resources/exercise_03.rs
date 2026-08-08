//! # 练习 05.03 —— init_resource 与 Option<Res>：资源可能不存在
//!
//! 出处：https://bevy.org/learn/quick-start/getting-started/resources/
//!
//! ## 概念
//! 资源不是必须存在的。`init_resource::<T>()` 会"没有就自动补一个默认值"，
//! 它要求 T 实现 `Default`。而系统参数写成 `Option<Res<T>>` 时，
//! 资源存在就是 `Some`、不存在就是 `None`，可以安全处理、不会报错。
//!
//! 反过来，直接写 `Res<T>` 而资源又不存在的话，Bevy 会在运行时直接 panic。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0503` 查看现象（目前编译会报错），
//! 改正后运行 `bevylings test 0503` 让测试通过。
//!
//! 小贴士：`match score { Some(s) => ..., None => ... }` 两种情况都要处理。

// I AM NOT DONE

use bevy::prelude::*;

/// 全局分数（默认 0 分）。
#[derive(Resource, Default)]
struct Score {
    points: u32,
}

/// 记录每次读取到的分数，方便测试观察。
#[derive(Resource, Default)]
struct ScoreLog(Vec<u32>);

/// 读分数并记下来；分数资源不存在时按 0 处理。
fn report(mut log: ResMut<ScoreLog>, score: Res<Score>) {
    // BUG: 参数写成了 Res<Score>，它不是 Option，
    // 下面的 Some/None 模式根本匹配不了（编译错误）。
    // 资源可能不存在，应该写成 Option<Res<Score>>。
    let points = match score {
        Some(s) => s.points,
        None => 0,
    };
    log.0.push(points);
}

pub fn run() {
    App::new()
        .init_resource::<Score>()
        .insert_resource(ScoreLog::default())
        .add_systems(Update, report)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(ScoreLog::default());
        app.add_systems(Update, report);
        app
    }

    #[test]
    fn reports_zero_when_missing() {
        // 不插入 Score：资源不存在时按 0 处理，不能 panic。
        let mut app = build_app();
        app.update();
        let log = app.world().resource::<ScoreLog>();
        assert_eq!(log.0, vec![0], "没有分数资源时应该记 0");
    }

    #[test]
    fn init_resource_provides_default() {
        let mut app = build_app();
        app.init_resource::<Score>();
        app.update();
        let log = app.world().resource::<ScoreLog>();
        assert_eq!(log.0, vec![0], "init_resource 会补一个默认 Score{{points: 0}}");
    }

    #[test]
    fn reports_actual_score_when_present() {
        let mut app = build_app();
        app.insert_resource(Score { points: 25 });
        app.update();
        let log = app.world().resource::<ScoreLog>();
        assert_eq!(log.0, vec![25], "资源存在时应该读出真实的分数");
    }
}

// 提示：
// 1. 先运行 `bevylings run 0503`，看编译错误里"mismatched types ... expected enum `Option`"。
// 2. `Res<T>` 表示"资源必须存在"；`Option<Res<T>>` 表示"资源可有可无"。
// 3. 把参数类型改成 `Option<Res<Score>>`，再运行 `bevylings test 0503`。
