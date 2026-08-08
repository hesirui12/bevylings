//! # 练习 02.02 —— 日志级别与过滤（LogPlugin）
//!
//! 出处：https://bevy.org/examples-webgpu/application/logs/
//!
//! ## 概念
//! Bevy 的日志分几个级别，从轻到重依次是：
//! `trace` < `debug` < `info` < `warn` < `error`。
//! 用 `info!`、`warn!`、`error!` 等宏打日志；`LogPlugin` 的 `level` 字段
//! 可以设置"至少哪个级别才会显示"，`filter` 字段可以按模块细分过滤。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0202` 观察现象，改正后运行 `bevylings test 0202` 让测试通过。
//!
//! 小贴士：默认情况下 `trace` 和 `debug` 太吵，是被过滤掉的。

// I AM NOT DONE

use bevy::log::Level;
use bevy::prelude::*;

/// 当前的严重程度（0 = info，2 = warn，3 = error）。
#[derive(Resource)]
struct Severity(u32);

/// 把"严重程度数字"翻译成日志级别名称。
/// 0~1 → info，2 → warn，3 及以上 → error。
fn severity_name(severity: u32) -> &'static str {
    // BUG: 这里把 warn 和 error 两个档位写反了，
    // 导致 2 被说成 error、3 被说成 warn。
    match severity {
        0..=1 => "info",
        2 => "error",
        _ => "warn",
    }
}

/// 演示三种常用日志级别。
fn log_system(severity: Res<Severity>) {
    info!("当前严重程度：{}", severity_name(severity.0));
    info!("helpful information that is worth printing by default");
    warn!("something bad happened, but we can still continue");
    error!("something failed");
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            // 只显示 warn 及以上级别
            level: Level::WARN,
            filter: "wgpu=error,bevy_render=info".to_string(),
            ..default()
        }))
        .insert_resource(Severity(3))
        .add_systems(Update, log_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_names_are_correct() {
        assert_eq!(severity_name(0), "info");
        assert_eq!(severity_name(1), "info");
        assert_eq!(severity_name(2), "warn", "严重程度 2 应该是 warn");
        assert_eq!(severity_name(3), "error", "严重程度 3 应该是 error");
    }

    #[test]
    fn levels_are_ordered_by_severity() {
        // 严重程度 0~1 是 info，2 是 warn，3 及以上都是 error。
        let names = [
            severity_name(0),
            severity_name(1),
            severity_name(2),
            severity_name(3),
            severity_name(10),
        ];
        let expect = ["info", "info", "warn", "error", "error"];
        assert_eq!(&names[..], &expect[..]);
    }
}

// 提示：
// 1. 注释里写了规则：2 → warn，3 及以上 → error。
// 2. match 的分支 2 和 `_`（其余情况）分别应该返回什么？
// 3. 修改后运行 `bevylings test 0202`，两个测试都通过就过关了。
