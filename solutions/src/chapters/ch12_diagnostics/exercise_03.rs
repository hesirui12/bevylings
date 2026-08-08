//! # 练习 12.03 —— 日志级别：控制什么信息会被打印
//!
//! 出处：https://bevy.org/examples-webgpu/app/logs/
//!
//! ## 概念
//! Bevy 的日志按严重程度分为五级：TRACE < DEBUG < INFO < WARN < ERROR。
//! `LogPlugin { level, .. }` 可以设置“最低打印级别”：比如设成 WARN，
//! 只有 WARN 和 ERROR 会被打印，INFO 及以下全部忽略。
//! 日志宏（`info!`、`warn!`、`error!` 等）也在 prelude 里，直接就能用。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1203` 观察现象，改正后运行 `bevylings test 1203` 让测试通过。
//!
//! 小贴士：级别常量叫 `Level::WARN`（不是 WARNING），从 `bevy::log::Level` 导入。

use bevy::{log::Level, prelude::*};

/// 判断某个级别是否达到最低输出级别（达到才打印）。
fn should_log(level: Level, min_level: Level) -> bool {
    // log crate 里数值越小级别越高：Error=1, Warn=2, Info=3 ...
    level <= min_level
}

/// 打印演示：INFO 低于 WARN 级别时，这条信息不会出现在控制台。
fn log_system() {
    if should_log(Level::INFO, Level::WARN) {
        info!("这条信息不会被打印，因为 INFO 低于 WARN 级别");
    }
    warn!("这是一条警告");
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: Level::WARN,
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, log_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warn_meets_warn_threshold() {
        assert!(should_log(Level::WARN, Level::WARN), "WARN 应该达到 WARN 阈值");
        assert!(!should_log(Level::INFO, Level::WARN), "INFO 低于 WARN，不该打印");
        assert!(!should_log(Level::DEBUG, Level::WARN), "DEBUG 低于 WARN，不该打印");
    }

    #[test]
    fn higher_levels_are_logged() {
        assert!(should_log(Level::ERROR, Level::WARN), "ERROR 高于 WARN，应该打印");
        assert!(should_log(Level::WARN, Level::INFO), "WARN 高于 INFO，应该打印");
    }
}

// 提示：
// 1. 运行 `bevylings run 1203`，看看编译器提示的“no variant named `WARNING`”。
// 2. 对照 `bevy::log::Level` 的五个变体：TRACE / DEBUG / INFO / WARN / ERROR。
// 3. 改好后运行 `bevylings test 1203`，测试全绿就过关了。
