//! # 练习 29.01 —— 自定义消息：derive(Message) 与 MessageWriter
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/message/
//!
//! ## 概念
//! 系统之间传数据，除了"共享资源"，还可以用**消息**（Message）：
//! 一个系统用 `MessageWriter` 把消息"写"出去，另一个系统用
//! `MessageReader` 在同一帧（或下一帧）"读"到它。消息类型要先
//! 用 `#[derive(Message)]` 声明，再在 App 里用 `add_message::<T>()` 注册，
//! 才能被发送和接收。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2901` 会先遇到一个编译错误，改正后运行
//! `bevylings test 2901` 让测试通过。
//!
//! 小贴士：`MessageWriter::write` 就像往邮箱里投信；同一帧里"先写后读"
//! 需要用 `.chain()` 把两个系统串起来，保证执行的先后顺序。

use bevy::prelude::*;

/// 一条"得分"消息
#[derive(Message)]
struct ScoreMessage {
    points: u32,
}

/// 累计得分
#[derive(Resource, Default)]
struct TotalScore(u32);

/// 每帧发一条 10 分的消息
fn gain_score(mut writer: MessageWriter<ScoreMessage>) {
    writer.write(ScoreMessage { points: 10 });
}

/// 读取消息并累计得分
fn collect_score(
    mut reader: MessageReader<ScoreMessage>,
    mut total: ResMut<TotalScore>,
) {
    for message in reader.read() {
        total.0 += message.points;
    }
}

pub fn run() {
    App::new()
        .insert_resource(TotalScore::default())
        .add_message::<ScoreMessage>()
        .add_systems(Update, (gain_score, collect_score).chain())
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn written_message_is_read() {
        let mut app = App::new();
        app.insert_resource(TotalScore::default());
        app.add_message::<ScoreMessage>();
        app.add_systems(Update, (gain_score, collect_score).chain());
        app.update();
        let total = app.world().resource::<TotalScore>();
        assert_eq!(total.0, 10, "一帧过后应累计到 10 分");
    }

    #[test]
    fn each_message_counts_once() {
        let mut app = App::new();
        app.insert_resource(TotalScore::default());
        app.add_message::<ScoreMessage>();
        app.add_systems(Update, (send_two, collect_score).chain());
        app.update();
        let total = app.world().resource::<TotalScore>();
        assert_eq!(total.0, 30, "10 + 20 = 30 分");
    }

    /// 测试用：一次发两条消息
    fn send_two(mut writer: MessageWriter<ScoreMessage>) {
        writer.write(ScoreMessage { points: 10 });
        writer.write(ScoreMessage { points: 20 });
    }
}

// 提示：
// 1. 编译错误会提示 `ScoreMessage: Message` 这个 trait 没有实现。
// 2. 消息类型需要一个 derive 宏，就像 `#[derive(Component)]` 一样，
//    只是换成 `#[derive(Message)]`。
// 3. 改好后运行 `bevylings test 2901`，两个测试都通过就过关了。
