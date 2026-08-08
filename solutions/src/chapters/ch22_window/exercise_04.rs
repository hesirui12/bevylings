//! # 练习 22.04 —— 清屏颜色：ClearColor 资源
//!
//! 出处：https://bevy.org/examples-webgpu/window/clear-color/
//!
//! ## 概念
//! 每帧渲染前，Bevy 先把整个窗口涂成一种颜色（叫"清屏颜色" ClearColor）。
//! 没被任何物体画到的像素就会保持这个颜色，所以它看起来像"背景色"。
//!
//! `ClearColor` 是一个资源：启动时用
//! `insert_resource(ClearColor(Color::srgb(r, g, b)))` 设置初始值；
//! 系统里用 `ResMut<ClearColor>` 在运行时改它。
//!
//! 本练习实现：按一次空格，把背景从淡蓝色换成紫色。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2204` 查看现象，改正后运行 `bevylings test 2204` 让测试通过。
//!
//! 小贴士：`Color::srgb(r, g, b)` 的三个参数都是 0.0~1.0，
//! 比如 (0.5, 0.5, 0.9) 是淡蓝色。

use bevy::{color::palettes::css::PURPLE, prelude::*};

/// 按空格时把清屏颜色换成紫色。
fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = PURPLE.into();
    }
}

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Update, change_clear_color)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)));
        app.init_resource::<ButtonInput<KeyCode>>();
        app.add_systems(Update, change_clear_color);
        app
    }

    #[test]
    fn space_switches_to_purple() {
        let mut app = build_app();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Space);
        app.update();
        let clear = app.world().resource::<ClearColor>();
        assert_eq!(clear.0, PURPLE.into(), "按空格后应该变成紫色");
    }

    #[test]
    fn nothing_happens_without_space() {
        let mut app = build_app();
        app.update();
        let clear = app.world().resource::<ClearColor>();
        assert_eq!(
            clear.0,
            Color::srgb(0.5, 0.5, 0.9),
            "没按空格，颜色不应该变"
        );
    }
}

// 提示：
// 1. 现在"按空格没反应，不按反而变色"——想想取反符号 `!` 放在哪。
// 2. 修改后运行 `bevylings test 2204`，两个测试会分别失败提醒你。
