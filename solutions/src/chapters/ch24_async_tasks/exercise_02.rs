//! # 练习 24.02 —— 异步通道：用 channel 传递结果
//!
//! 出处：https://bevy.org/examples-webgpu/async_tasks/async_channel_pattern/
//!
//! ## 概念
//! 上一题我们用 `check_ready` 每帧"问"任务做完了没。还有一种做法：
//! 任务做完后自己把结果**扔进一个通道（channel）**，主线程每帧去
//! 通道里"收件"。这样主线程完全不用关心任务什么时候完成，各干各的。
//!
//! `std::sync::mpsc::channel()` 会返回一对：发送端 `Sender` 和接收端
//! `Receiver`。注意标准库的 `Receiver` 不是 `Sync` 的，不能直接塞进
//! Bevy 资源（资源要求 Send + Sync），所以要再用 `Mutex` 包一层。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2402` 观察现象，改正后运行 `bevylings test 2402` 让测试通过。
//!
//! 小贴士：`Mutex` 里的东西要先 `.lock().unwrap()` 才能用，
//! 作用域结束（或手动 drop）时自动解锁。

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

/// 异步任务算完后的结果。
#[derive(Debug)]
struct CubeFinished {
    transform: Transform,
}

/// 连接后台任务与主线程的通道。
#[derive(Resource)]
struct CubeChannel {
    sender: Sender<CubeFinished>,
    receiver: Mutex<Receiver<CubeFinished>>,
}

/// 已经收到结果生成的位置（简化：只记录，不真的造方块网格）。
#[derive(Resource, Default)]
struct SpawnedCubes(Vec<Transform>);

/// 启动时创建通道。
fn setup_channel(mut commands: Commands) {
    let (sender, receiver) = channel();
    commands.insert_resource(CubeChannel { sender, receiver: Mutex::new(receiver) });
}

/// 派 4 个后台任务，每个算出一个位置后塞进通道。
fn spawn_tasks(channel: Res<CubeChannel>) {
    let pool = AsyncComputeTaskPool::get();
    for x in -2..2 {
        let sender = channel.sender.clone();
        pool.spawn(async move {
            let _ = sender.send(CubeFinished {
                transform: Transform::from_xyz(x as f32, 0.0, 0.0),
            });
        })
        .detach(); // 任务通过通道自己送结果，主线程不用轮询它
    }
}

/// 每帧把通道里积压的结果取出来。
fn handle_finished_cubes(channel: Res<CubeChannel>, mut spawned: ResMut<SpawnedCubes>) {
    for msg in channel.receiver.lock().unwrap().try_iter() {
        spawned.0.push(msg.transform);
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SpawnedCubes::default())
        .add_systems(Startup, (setup_channel, spawn_tasks.after(setup_channel)))
        .add_systems(Update, handle_finished_cubes)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn results_from_channel_are_collected() {
        let mut app = App::new();
        app.insert_resource(SpawnedCubes::default());
        app.add_systems(Startup, setup_channel);
        app.add_systems(Update, handle_finished_cubes);
        app.update(); // Startup：通道建好

        // 手动往通道里塞两条"后台任务的结果"
        let sender = app.world().resource::<CubeChannel>().sender.clone();
        sender
            .send(CubeFinished {
                transform: Transform::from_xyz(1.0, 0.0, 0.0),
            })
            .unwrap();
        sender
            .send(CubeFinished {
                transform: Transform::from_xyz(2.0, 0.0, 0.0),
            })
            .unwrap();

        app.update(); // Update：消费通道
        let spawned = app.world().resource::<SpawnedCubes>();
        assert_eq!(spawned.0.len(), 2, "两条结果都应该被收到");
        assert!(
            spawned.0.iter().any(|t| t.translation.x == 2.0),
            "x = 2 的结果应该在"
        );
    }

    #[test]
    fn messages_are_consumed_only_once() {
        let mut app = App::new();
        app.insert_resource(SpawnedCubes::default());
        app.add_systems(Startup, setup_channel);
        app.add_systems(Update, handle_finished_cubes);
        app.update();

        let sender = app.world().resource::<CubeChannel>().sender.clone();
        sender
            .send(CubeFinished {
                transform: Transform::from_xyz(5.0, 0.0, 0.0),
            })
            .unwrap();

        app.update();
        app.update();
        let spawned = app.world().resource::<SpawnedCubes>();
        assert_eq!(spawned.0.len(), 1, "同一条消息只应被消费一次");
    }
}

// 提示：
// 1. 先运行 `bevylings run 2402`，看编译错误指向哪一行。
// 2. 报错大概是"no method named `try_iter` found for struct `Mutex<...>`"——
//    Mutex 本身没有 try_iter，要先 `.lock().unwrap()` 拿到里面的 Receiver。
// 3. 改好后运行 `bevylings test 2402`，两个测试全绿就过关了。
