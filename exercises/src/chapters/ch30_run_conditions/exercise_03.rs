//! # 练习 30.03 —— 系统管道：pipe 串联
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/system-piping/
//!
//! ## 概念
//! 系统之间除了各跑各的，还能用 `.pipe()` **串联**：第一个系统算出一个值，
//! 作为第二个系统的输入接着处理，就像 Rust 里的函数链 `a().then(b)`。
//!
//! 规则：第二个系统必须用 `In<T>` 声明输入，并且 `T` 要跟第一个系统的
//! **输出类型完全一致**。本练习里第一个系统解析字符串得到 `Result<u32, ParseIntError>`，
//! 所以第二个系统的输入类型必须也是 `Result<u32, ParseIntError>`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3003` 查看现象，改正后运行 `bevylings test 3003` 让测试通过。
//!
//! 小贴士：解析字符串可能失败（"abc" 不是数字），所以管道里传递的是 `Result`。

// I AM NOT DONE

use std::num::ParseIntError;

use bevy::prelude::*;

/// 存着一条消息，内容是一串数字文本。
#[derive(Resource, Deref)]
struct Message(String);

/// 记录解析出来的数字，方便测试观察。
#[derive(Resource, Default)]
struct Results(Vec<u32>);

/// 第一个系统：把消息文本解析成数字（可能失败，所以返回 Result）。
fn parse_message(message: Res<Message>) -> Result<u32, ParseIntError> {
    message.parse::<u32>()
}

/// 第二个系统：接收第一个系统的输出，把数字存起来（失败就记 0）。
fn handler_system(
    // BUG: 管道传过来的是 Result<u32, ParseIntError>，这里却声明成了 u32，
    // 和第一个系统的输出类型对不上，`.pipe()` 编译失败。
    In(result): In<u32>,
    mut results: ResMut<Results>,
) {
    match result {
        Ok(value) => results.0.push(value),
        Err(_) => results.0.push(0),
    }
}

pub fn run() {
    App::new()
        .insert_resource(Message("42".to_string()))
        .init_resource::<Results>()
        .add_systems(Update, parse_message.pipe(handler_system))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app(message: &str) -> App {
        let mut app = App::new();
        app.insert_resource(Message(message.to_string()));
        app.init_resource::<Results>();
        app.add_systems(Update, parse_message.pipe(handler_system));
        app
    }

    #[test]
    fn pipe_parses_valid_message() {
        let mut app = build_app("42");
        app.update();
        let results = &app.world().resource::<Results>().0;
        assert_eq!(results, &vec![42], "42 应该被解析出来");
    }

    #[test]
    fn pipe_handles_parse_error() {
        let mut app = build_app("not a number");
        app.update();
        let results = &app.world().resource::<Results>().0;
        assert_eq!(results, &vec![0], "解析失败时记 0");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3003`，看编译报错说"类型不匹配"在哪。
// 2. `parse_message` 的输出是 `Result<u32, ParseIntError>`，
//    所以 `In<...>` 里的类型要原样写这个 Result。
// 3. `In<T>` 里的 T 必须和上游输出完全一致，多包一层、少包一层都不行。
