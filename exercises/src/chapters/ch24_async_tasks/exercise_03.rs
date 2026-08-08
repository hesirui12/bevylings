//! # 练习 24.03 —— 外部线程：用 std::thread 持续产数据
//!
//! 出处：https://bevy.org/examples-webgpu/async_tasks/external_source_external_thread/
//!
//! ## 概念
//! 上一题的任务跑在 Bevy 自己的任务池里。但有些数据来自 Bevy 之外——
//! 比如传感器、网络连接。这时可以用标准库的 `std::thread::spawn`
//! 开一个**独立的外部线程**，让它自己生产数据塞进通道；主线程的
//! 系统每帧把通道里的数据取出来处理。
//!
//! 为了演示，我们约定：**只有偶数才是有效样本**，读到奇数直接丢弃。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2403` 观察现象，改正后运行 `bevylings test 2403` 让测试通过。
//!
//! 小贴士：`try_iter()` 只取"现在已经在通道里"的消息，不会阻塞等待。

// I AM NOT DONE

use bevy::prelude::*;
use std::sync::mpsc::{channel, Receiver};
use std::sync::Mutex;

/// 外部线程往这个通道里塞数据。
#[derive(Resource)]
struct StreamReceiver(Mutex<Receiver<u32>>);

/// 主线程收集到的有效样本。
#[derive(Resource, Default)]
struct CollectedValues(Vec<u32>);

/// 判断一个样本是否有效（约定：偶数才有效）。
fn is_valid_sample(value: u32) -> bool {
    // BUG: 奇偶判断写反了，导致有效样本被丢掉、无效样本被收进来。
    value % 2 == 1
}

/// 启动时开一个外部线程，模拟传感器持续产出数据。
fn setup(mut commands: Commands) {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        // 真实项目里这里可能是网络连接或传感器；我们简单发 10 个数。
        for i in 0..10 {
            let _ = tx.send(i);
        }
    });
    commands.insert_resource(StreamReceiver(Mutex::new(rx)));
}

/// 每帧把通道里积压的样本读出来，过滤后收集。
fn read_stream(receiver: Res<StreamReceiver>, mut collected: ResMut<CollectedValues>) {
    for value in receiver.0.lock().unwrap().try_iter() {
        if is_valid_sample(value) {
            collected.0.push(value);
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CollectedValues::default())
        .add_systems(Startup, setup)
        .add_systems(Update, read_stream)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_even_samples_are_valid() {
        assert!(is_valid_sample(4), "4 是偶数，应该有效");
        assert!(!is_valid_sample(7), "7 是奇数，应该无效");
    }

    #[test]
    fn stream_is_read_and_filtered() {
        // 自己控制通道，不依赖外部线程，保证测试确定性。
        let (tx, rx) = channel();
        tx.send(10).unwrap();
        tx.send(11).unwrap();
        tx.send(12).unwrap();

        let mut app = App::new();
        app.insert_resource(StreamReceiver(Mutex::new(rx)));
        app.insert_resource(CollectedValues::default());
        app.add_systems(Update, read_stream);
        app.update();

        let collected = app.world().resource::<CollectedValues>();
        assert_eq!(collected.0, vec![10, 12], "只有偶数样本会被收集");
    }
}

// 提示：
// 1. 先运行 `bevylings run 2403`，观察它收集到的是奇数还是偶数。
// 2. `value % 2` 只有两种结果：0（偶数）和 1（奇数）。
// 3. 改好后运行 `bevylings test 2403`，两个测试全绿就过关了。
