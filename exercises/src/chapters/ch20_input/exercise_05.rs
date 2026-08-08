//! # 练习 20.05 —— 修饰键：组合快捷键
//!
//! 出处：https://bevy.org/examples-webgpu/input/keyboard-modifiers/
//!
//! ## 概念
//! 游戏里常见的"组合键"（比如 Ctrl + Shift + A）需要同时判断几个键的状态。
//! 修饰键（Shift、Ctrl、Alt）在键盘左右各有一个：
//! 左 Shift 是 `KeyCode::ShiftLeft`，右 Shift 是 `KeyCode::ShiftRight`。
//! 用户按哪个都算数，所以要用 `any_pressed` 把两个位置一起检查：
//! "左右任意一个被按住，就算按住 Shift"。
//!
//! 本练习实现：当 Ctrl 和 Shift 都按住、并且 A 键**这一帧刚被按下**时，
//! 触发一次组合快捷键。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2005` 查看现象，改正后运行 `bevylings test 2005` 让测试通过。
//!
//! 小贴士：`any_pressed` 接收一个数组，如 `[KeyCode::ShiftLeft, KeyCode::ShiftRight]`，
//! 数组中任意一个被按住就返回 true。

// I AM NOT DONE

use bevy::prelude::*;

/// Shift 键（左右任意一个）被按住吗？
fn shift_held(input: &ButtonInput<KeyCode>) -> bool {
    // BUG: 只检查了左 Shift。习惯按右 Shift 的用户会发现
    // 组合键怎么按都不触发。
    input.pressed(KeyCode::ShiftLeft)
}

/// Ctrl + Shift + A 组合键：本帧 A 刚被按下，同时 Ctrl 和 Shift 都按住。
fn combo_pressed(input: &ButtonInput<KeyCode>) -> bool {
    let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    shift_held(input) && ctrl && input.just_pressed(KeyCode::KeyA)
}

/// 组合键监听系统。
fn keyboard_input_system(input: Res<ButtonInput<KeyCode>>) {
    if combo_pressed(&input) {
        info!("Just pressed Ctrl + Shift + A!");
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_input_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_shift_also_counts_as_shift() {
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::ShiftRight);
        assert!(shift_held(&input), "按的是右 Shift，也算按住 Shift");
    }

    #[test]
    fn combo_requires_ctrl_shift_and_fresh_a() {
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::ControlLeft);
        input.press(KeyCode::ShiftRight);
        input.press(KeyCode::KeyA);
        assert!(combo_pressed(&input), "三个条件都满足，组合键应该触发");

        // 模拟进入下一帧：A 还按着，但不再是"刚按下"
        input.clear_just_pressed(KeyCode::KeyA);
        assert!(!combo_pressed(&input), "A 没有刚按下，不应重复触发");
    }
}

// 提示：
// 1. `pressed` 只检查一个键；`any_pressed` 接受一个数组，检查"任意一个"。
// 2. 组合键用 `just_pressed` 判断主键，避免按住时每帧重复触发。
// 3. 修改后运行 `bevylings test 2005`，第一个测试会失败提醒你。
