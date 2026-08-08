//! # 练习 35.02 —— 日志分层：什么级别才值得打印
//!
//! 出处：https://bevy.org/examples-webgpu/app/log-layers/
//!
//! ## 概念
//! Bevy 的日志从高到低分五级：
//! `ERROR` > `WARN` > `INFO` > `DEBUG` > `TRACE`。
//! `LogPlugin` 里配置的 `level` 是"最低打印线"：消息级别**达到或超过**
//! 配置级别才会被输出，太安静的日志会被过滤掉（比如配了 INFO，
//! DEBUG / TRACE 就不打）。官方示例还演示了自定义"日志层"（Layer），
//! 可以完全接管日志的输出格式。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3502` 查看现象，改正后运行 `bevylings test 3502` 让测试通过。
//!
//! 小贴士：log crate 里**数值越小级别越高**（Error=1 < Warn=2 < Info=3 < Debug=4 < Trace=5），
//! 所以"比配置级别更严重"要写成 `level <= configured`。

// I AM NOT DONE

use bevy::log::tracing::Level;
use bevy::prelude::*;

/// 某条日志的级别够不够高，能不能被输出？
/// 规则：消息级别比配置级别更严重（log crate 里数值更小）才输出。
fn should_output(level: Level, configured: Level) -> bool {
    // BUG: 比较方向写反了。注意 log crate 里数值越小级别越高：
    // Error=1 < Warn=2 < Info=3 ...，所以"更严重"是 <= 而不是 >=。
    // 现在的写法把"严重"和"安静"完全搞反了。
    level >= configured
}

/// 示例系统：从 ERROR 到 TRACE 各打一条，观察分级效果。
fn log_system() {
    error!("something failed");
    warn!("something bad happened");
    info!("helpful information");
    debug!("helpful for debugging");
    trace!("very noisy");
}

pub fn run() {
    App::new()
        .add_plugins(bevy::log::LogPlugin {
            level: Level::INFO,
            ..default()
        })
        .add_systems(Update, log_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_or_equal_levels_are_output() {
        assert!(should_output(Level::ERROR, Level::INFO), "ERROR 应该被输出");
        assert!(should_output(Level::INFO, Level::INFO), "等于配置级别也输出");
        assert!(!should_output(Level::DEBUG, Level::INFO), "DEBUG 低于 INFO，不该输出");
    }

    #[test]
    fn trace_is_the_quietest() {
        assert!(!should_output(Level::TRACE, Level::WARN), "TRACE 太安静了");
        assert!(should_output(Level::ERROR, Level::TRACE), "ERROR 在任何配置下都输出");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3502`，看哪个断言失败了。
// 2. 级别从高到低：ERROR > WARN > INFO > DEBUG > TRACE。
// 3. "达到或超过"是 `>=`，别写成 `<=`。
