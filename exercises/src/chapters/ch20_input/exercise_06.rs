//! # 练习 20.06 —— 键盘事件：KeyboardInput 消息
//!
//! 出处：https://bevy.org/examples-webgpu/input/keyboard-input-events/
//!
//! ## 概念
//! 除了 `ButtonInput<KeyCode>` 这种"当前状态"，Bevy 还把每一次按键都发布成一条
//! **消息（Message）**：`KeyboardInput`。消息里带着丰富的信息：
//! - `state`：这次是"按下"（`ButtonState::Pressed`）还是"松开"（`Released`）。
//! - `logical_key`：按下的**逻辑键**，比如 `Key::Character('a')` 表示字符 a，
//!   `Key::Escape` 表示 Esc。
//! - `repeat`：是不是长按触发的系统自动重复。
//!
//! 系统用 `MessageReader<KeyboardInput>` 读取本帧的消息，用法类似 `for ... in ...`。
//! 本练习实现：把**刚按下**的字符收集进缓冲区（松开的、非字符键都不算）。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2006` 查看现象，改正后运行 `bevylings test 2006` 让测试通过。
//!
//! 小贴士：`event.state.is_pressed()` 问"这是一次按下事件吗"；松开的返回 false。

// I AM NOT DONE

use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

/// 记录收集到的字符（方便测试观察）。
#[derive(Resource, Default)]
struct CharLog(Vec<String>);

/// 收集所有"刚按下"的字符。
fn collect_pressed_chars(
    mut reader: MessageReader<KeyboardInput>,
    mut log: ResMut<CharLog>,
) {
    for event in reader.read() {
        // BUG: 跳过条件写反了：现在反而是"松开"的事件才继续往下走。
        // 我们只想要"按下"的事件，松开、重复的都该跳过。
        if event.state.is_pressed() {
            continue;
        }
        if let Key::Character(ch) = &event.logical_key {
            log.0.push(ch.to_string());
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CharLog::default())
        .add_systems(Update, collect_pressed_chars)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::ButtonState;

    fn build_app() -> App {
        let mut app = App::new();
        app.add_message::<KeyboardInput>();
        app.insert_resource(CharLog::default());
        app.add_systems(Update, collect_pressed_chars);
        app
    }

    fn send_key(app: &mut App, logical: Key, state: ButtonState) {
        app.world_mut()
            .resource_mut::<Messages<KeyboardInput>>()
            .write(KeyboardInput {
                key_code: KeyCode::KeyA,
                logical_key: logical,
                state,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
    }

    #[test]
    fn only_pressed_characters_are_collected() {
        let mut app = build_app();
        send_key(&mut app, Key::Character("a".into()), ButtonState::Pressed);
        send_key(&mut app, Key::Character("b".into()), ButtonState::Released);
        app.update();
        let log = app.world().resource::<CharLog>();
        assert_eq!(log.0, vec!["a".to_string()], "只收集按下的 a，松开的 b 不要");
    }

    #[test]
    fn non_character_keys_are_ignored() {
        let mut app = build_app();
        send_key(&mut app, Key::Escape, ButtonState::Pressed);
        app.update();
        let log = app.world().resource::<CharLog>();
        assert!(log.0.is_empty(), "Esc 不是字符，不该进缓冲区");
    }
}

// 提示：
// 1. `continue` 表示"跳过这一条"，想想什么条件下该跳过。
// 2. `is_pressed()` 的返回值是"这次是按下"；想要按下的事件，需要取反后跳过。
// 3. 修改后运行 `bevylings test 2006`，第一个测试会失败提醒你。
