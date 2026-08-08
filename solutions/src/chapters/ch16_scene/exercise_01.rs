//! # 练习 16.01 —— BSN 场景基础：bsn_list! 与 bsn!
//!
//! 出处：https://bevy.org/examples-webgpu/scene/bsn/
//!
//! ## 概念
//! BSN（Bevy Scene Notation）是 Bevy 0.19 用来"描述场景"的新语法：
//! - `bsn! { ... }` 描述**一个实体**（它的组件、子节点），返回一个 `Scene`；
//! - `bsn_list![...]` 把多个 Scene 打包成一个 `SceneList`；
//! - `fn scene() -> impl SceneList` 定义整张场景，`scene.spawn()`
//!   把它变成一个"启动时运行"的系统。
//!
//! 官方示例用 BSN 描述了一个相机加一排按钮的 UI。
//! 本练习简化为：一个 2D 相机 + "开始 / 退出"两个按钮。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1601` 查看现象，改正后运行 `bevylings test 1601` 让测试通过。
//!
//! 小贴士：UI 里表示尺寸要写 `px(150)`（像素）或 `percent(100)`（百分比），
//! 它们返回 `Val` 类型；直接写裸数字 `150` 类型对不上。

use bevy::prelude::*;

/// 整张场景：一个 2D 相机 + 一组按钮。
fn scene() -> impl SceneList {
    bsn_list![Camera2d, ui()]
}

/// 根 UI 容器：铺满整个屏幕，按钮横向排列。
fn ui() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: px(5),
        }
        Children [
            (button("开始")),
            (button("退出")),
        ]
    }
}

/// 一个按钮实体：Button 组件 + 居中排版 + 底色。
fn button(label: &str) -> impl Scene {
    bsn! {
        Button
        Node {
            width: px(150),
            height: px(65),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(button_color(label))
        Children [(
            Text(label)
            TextColor(Color::srgb(0.9, 0.9, 0.9))
        )]
    }
}

/// 不同按钮用不同底色："退出"是暗红色，其余是深灰色。
fn button_color(label: &str) -> Color {
    if label == "退出" {
        Color::srgb(0.4, 0.15, 0.15)
    } else {
        Color::srgb(0.15, 0.15, 0.15)
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, scene.spawn())
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quit_button_uses_dark_red() {
        assert_eq!(button_color("退出"), Color::srgb(0.4, 0.15, 0.15));
    }

    #[test]
    fn normal_button_uses_dark_gray() {
        assert_eq!(button_color("开始"), Color::srgb(0.15, 0.15, 0.15));
    }

    #[test]
    fn buttons_are_visually_distinct() {
        assert_ne!(button_color("开始"), button_color("退出"));
    }
}

// 提示：
// 1. 先运行 `bevylings run 1601`，编译器会指出按钮宽度那一行类型不对。
// 2. 想一想：UI 里的"宽度"是什么类型？用什么函数能构造出这种类型？
// 3. 修好后运行 `bevylings test 1601`，三个测试全绿就过关。
