//! # 练习 01.04 —— Local：系统自己的本地状态
//!
//! 出处：https://bevy.org/examples-webgpu/application/headless/
//!
//! ## 概念
//! 普通 Rust 函数的局部变量在函数结束后就销毁了。可系统每次被调用都想
//! 记住"上一次数到几"，怎么办？用 `Local<T>` 系统参数！
//! 它会在系统多次调用之间保留一份**只属于这个系统**、别人碰不到的状态。
//! 这里仿照官方的 counter 示例：每帧计数加 1，每满 60 帧打印一次。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0104` 观察现象，改正后运行 `bevylings test 0104` 让测试通过。
//!
//! 小贴士：`Local<T>` 要求 `T` 实现 `Default`，系统第一次运行时用默认值初始化。

// I AM NOT DONE

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use core::time::Duration;

/// 只属于 counter 系统的本地状态。
#[derive(Default)]
struct CounterState {
    count: u32,
}

/// 记录每帧的计数，方便测试观察 Local 是否跨帧保留。
#[derive(Resource, Default)]
struct History(Vec<u32>);

/// 每帧把计数加 1；满 60 帧打印一次。
fn counter(mut state: Local<CounterState>, mut history: ResMut<History>) {
    state.count += 1;
    history.0.push(state.count);
    if should_log(state.count) {
        println!("frame {}", state.count);
    }
}

/// 什么时候该打印？—— 每 60 帧打印一次。
fn should_log(count: u32) -> bool {
    // BUG: 这里的比较条件写反了，导致几乎每一帧都在打印。
    count % 60 != 0
}

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(16),
        )))
        .insert_resource(History::default())
        .add_systems(Update, counter)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_every_sixty_frames() {
        assert!(should_log(60), "第 60 帧应该打印");
        assert!(should_log(120), "第 120 帧应该打印");
        assert!(!should_log(61), "第 61 帧不应该打印");
    }

    #[test]
    fn local_state_persists_across_frames() {
        let mut app = App::new();
        app.insert_resource(History::default());
        app.add_systems(Update, counter);
        app.update();
        app.update();
        app.update();
        let history = app.world().resource::<History>();
        assert_eq!(history.0, vec![1, 2, 3], "Local 状态应该在帧之间保留");
    }
}

// 提示：
// 1. "每 60 帧打印一次"用取模运算怎么写？先写出正确的条件。
// 2. 现在代码里用的是 `!=`，想一想它和正确条件是什么关系。
// 3. 修改后运行 `bevylings test 0104`，两个测试都通过就过关了。
