//! # 练习 04.02 —— 插件添加资源与系统（带配置字段）
//!
//! 出处：https://bevy.org/examples-webgpu/application/plugin/
//!
//! ## 概念
//! 插件可以带上**配置字段**，比如"每隔多久打印、打印什么内容"。
//! 注意 `build(&self, ...)` 只拿到 `&self`（共享借用），所以想把配置里的
//! 数据放进资源时，必须**克隆**一份（`.clone()`），不能直接"拿走"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0402` 观察现象，改正后运行 `bevylings test 0402` 让测试通过。
//!
//! 小贴士：看到 "cannot move out of ... behind a shared reference" 这类报错，
//! 想想是不是该用 `.clone()`。

// I AM NOT DONE

use bevy::prelude::*;
use core::time::Duration;

/// 一个"每隔一段时间打印消息"的插件，配置项是消息内容和间隔。
struct PrintMessagePlugin {
    message: String,
    interval: Duration,
}

/// 插件运行时保存的状态（资源）。
#[derive(Resource)]
struct PrintMessageState {
    message: String,
    timer: Timer,
}

impl Plugin for PrintMessagePlugin {
    fn build(&self, app: &mut App) {
        let state = PrintMessageState {
            // BUG: 这里想把配置里的 message 移进状态资源，
            // 但 build 只拿到了 &self（共享借用），不能移走，编译会报错。
            message: self.message,
            timer: Timer::new(self.interval, TimerMode::Repeating),
        };
        app.insert_resource(state).add_systems(Update, print_message_system);
    }
}

/// 每隔一段时间打印一次配置里的消息。
fn print_message_system(mut state: ResMut<PrintMessageState>, time: Res<Time>) {
    if state.timer.tick(time.delta()).is_finished() {
        info!("{}", state.message);
    }
}

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(PrintMessagePlugin {
            message: "This is an example plugin".to_string(),
            interval: Duration::from_secs(1),
        })
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins); // 提供 Time 资源
        app.add_plugins(PrintMessagePlugin {
            message: "hi".to_string(),
            interval: Duration::from_secs(1),
        });
        app
    }

    #[test]
    fn plugin_config_reaches_state() {
        let mut app = build_app();
        app.update();
        let state = app.world().resource::<PrintMessageState>();
        assert_eq!(state.message, "hi", "配置里的消息应该被克隆进状态资源");
    }

    #[test]
    fn state_has_a_repeating_timer() {
        let mut app = build_app();
        app.update();
        let state = app.world().resource::<PrintMessageState>();
        assert_eq!(state.timer.mode(), TimerMode::Repeating, "定时器应该是重复模式");
    }
}

// 提示：
// 1. 报错大意：cannot move out of `self.message` which is behind a shared
//    reference。也就是说不能从 &self 里把值移走。
// 2. String 实现了一个"复制内容"的方法，想想是什么（和"移动"相对）。
// 3. 修改后运行 `bevylings test 0402`，两个测试都通过就过关了。
