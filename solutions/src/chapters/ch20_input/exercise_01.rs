//! # 练习 20.01 —— 键盘输入：ButtonInput<KeyCode>
//!
//! 出处：https://bevy.org/examples-webgpu/input/keyboard-input/
//!
//! ## 概念
//! 电脑键盘上的每个键都有一个编号，叫做 `KeyCode`（例如 `KeyCode::KeyW` 就是 W 键）。
//! Bevy 把"哪些键正被按住"存进资源 `ButtonInput<KeyCode>`，系统把它作为参数取出来，
//! 常用三个查询方法：
//! - `pressed(键)`：这个键当前是否被按住（按住期间每帧都返回 true）。
//! - `just_pressed(键)`：这个键是否**这一帧刚被按下**（只在按下瞬间那一帧返回 true）。
//! - `just_released(键)`：这个键是否**这一帧刚被松开**。
//!
//! 我们写一个函数把按键状态翻译成中文描述，帮助理解三者的区别。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2001` 查看现象（目前编译会报错），
//! 改正后运行 `bevylings test 2001` 让测试通过。
//!
//! 小贴士：`ButtonInput` 像一袋"当前有效的按键集合"：查询用只读方法，
//! 千万别把写方法（`press` / `release`）当成查询用。

use bevy::prelude::*;

/// 把某个键的状态翻译成中文描述。
fn key_state(input: &ButtonInput<KeyCode>, key: KeyCode) -> &'static str {
    if input.just_pressed(key) {
        "刚刚按下"
    } else if input.pressed(key) {
        "正在按住"
    } else {
        "没有按下"
    }
}

/// 键盘监听系统：A 键刚按下 / 刚松开时各打印一次。
fn keyboard_system(input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyA) {
        info!("'A' 刚刚被按下");
    }
    if input.just_released(KeyCode::KeyA) {
        info!("'A' 刚刚被松开");
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_press_is_just_pressed() {
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyW);
        assert_eq!(key_state(&input, KeyCode::KeyW), "刚刚按下");
        // 没按过的键应该返回"没有按下"
        assert_eq!(key_state(&input, KeyCode::KeyA), "没有按下");
    }

    #[test]
    fn held_key_is_no_longer_just_pressed_next_frame() {
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyW);
        // 模拟进入下一帧："刚按下"标记被清掉，但键仍然处于按住状态
        input.clear_just_pressed(KeyCode::KeyW);
        assert_eq!(key_state(&input, KeyCode::KeyW), "正在按住");

        input.release(KeyCode::KeyW);
        assert_eq!(key_state(&input, KeyCode::KeyW), "没有按下");
    }
}

// 提示：
// 1. 先看 `// BUG:` 那一行：`press` 是"注册按下"还是"查询状态"？
// 2. 查询方法只借用数据（`&self`），而 `press` / `release` 要修改数据（`&mut self`）。
// 3. 改成只读的查询方法后运行 `bevylings test 2001`，两个测试都通过就过关了。
