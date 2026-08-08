//! # 练习 22.02 —— 窗口调整：响应 WindowResized 消息
//!
//! 出处：https://bevy.org/examples-webgpu/window/window-resizing/
//!
//! ## 概念
//! 用户拖拽窗口边缘改变大小时，Bevy 会发布一条 `WindowResized` 消息，
//! 里面带着新的宽和高（`width`、`height` 两个 f32 字段）。
//! 系统用 `MessageReader<WindowResized>` 读取，然后更新 UI 文字，
//! 让屏幕上的提示跟着窗口一起变。
//!
//! 我们还准备了一个"分辨率预设"资源：按数字键 1/2/3 可以把窗口切成
//! 小/中/大三档分辨率。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2202` 查看现象，改正后运行 `bevylings test 2202` 让测试通过。
//!
//! 小贴士：`format!("{:.1} x {:.1}", 宽, 高)` 会显示成 "800.0 x 600.0"。

use bevy::{
    prelude::*,
    window::WindowResized,
};

/// 标记"显示当前分辨率的文字"的组件。
#[derive(Component)]
struct ResolutionText;

/// 三档预设分辨率。
#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

/// 按键选择分辨率：1/2/3 分别对应小/中/大。
fn pick_resolution(settings: &ResolutionSettings, key: KeyCode) -> Option<Vec2> {
    match key {
        KeyCode::Digit1 => Some(settings.small),
        KeyCode::Digit2 => Some(settings.medium),
        KeyCode::Digit3 => Some(settings.large),
        _ => None,
    }
}

/// 响应窗口大小变化：把新分辨率写到屏幕上。
fn on_resize_system(
    mut text: Single<&mut Text, With<ResolutionText>>,
    mut resize_reader: MessageReader<WindowResized>,
) {
    for e in resize_reader.read() {
        text.0 = format!("{:.1} x {:.1}", e.width, e.height);
    }
}

/// 启动时生成显示分辨率的文字。
fn setup_ui(mut commands: Commands) {
    commands.spawn((Text::new("Resolution"), ResolutionText));
}

pub fn run() {
    App::new()
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_ui)
        .add_systems(Update, on_resize_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.add_message::<WindowResized>();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Text::new("?"), ResolutionText));
        });
        app.add_systems(Update, on_resize_system);
        app
    }

    #[test]
    fn resize_updates_text_with_both_dimensions() {
        let mut app = build_app();
        app.world_mut()
            .resource_mut::<Messages<WindowResized>>()
            .write(WindowResized {
                window: Entity::PLACEHOLDER,
                width: 800.0,
                height: 600.0,
            });
        app.update();
        let mut text_query = app
            .world_mut()
            .query_filtered::<&Text, With<ResolutionText>>();
        let text = text_query.single(app.world()).unwrap();
        assert_eq!(text.0, "800.0 x 600.0", "文字应该同时显示宽和高");
    }

    #[test]
    fn picks_resolution_by_number_key() {
        let settings = ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        };
        assert_eq!(
            pick_resolution(&settings, KeyCode::Digit2),
            Some(Vec2::new(800.0, 600.0))
        );
        assert_eq!(pick_resolution(&settings, KeyCode::KeyQ), None);
    }
}

// 提示：
// 1. `format!` 里写了几个占位符 `{:.1}`，就对应几个参数，顺序要对上。
// 2. 比较一下错误版显示 "800.0 x 800.0" 和应该显示的 "800.0 x 600.0"。
// 3. 修改后运行 `bevylings test 2202`，第一个测试会失败提醒你。
