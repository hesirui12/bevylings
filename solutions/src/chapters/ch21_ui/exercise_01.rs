//! # 练习 21.01 —— 文本 UI：Text、TextFont 与 Node
//!
//! 出处：https://bevy.org/examples-webgpu/ui/text/
//!
//! ## 概念
//! UI（用户界面）在 Bevy 里也是由实体和组件构成的：每个 UI 元素是一个实体，
//! 上面挂着 `Node`（尺寸与布局）、`Text`（文本内容）、`TextFont`（字体大小）等组件。
//! 修改文本内容就像改普通 Rust 值：在系统里拿到 `&mut Text`，把新字符串写进去，
//! 画面就会在下一帧更新。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2101` 观察文本何时更新，改正后运行
//! `bevylings test 2101` 让测试通过。
//!
//! 小贴士：`Text` 是 `String` 的"外壳"（newtype），它实现了 `Deref`，
//! 所以 `**text = "xxx".to_string()` 可以直接改写里面的字符串。

use bevy::prelude::*;

/// 标记组件：用来找到"帧数文本"这个 UI 实体（而不是其它 Text）
#[derive(Component)]
struct FrameText;

/// 记录已经跑过的帧数
#[derive(Resource, Default)]
struct Frames(u32);

/// 启动时生成相机和一段文本
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new("0"),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextColor(Color::WHITE),
        FrameText,
    ));
}

/// 每帧把文本更新为当前帧数（只在偶数帧更新）
fn update_frame_text(
    mut frames: ResMut<Frames>,
    mut query: Query<&mut Text, With<FrameText>>,
) {
    frames.0 += 1;
    for mut text in &mut query {
        if should_display(frames.0) {
            **text = format!("frame {}", frames.0);
        }
    }
}

/// 只在偶数帧显示计数
fn should_display(frame: u32) -> bool {
    frame % 2 == 0
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Frames::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_frame_text)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_frames_should_display() {
        assert!(should_display(2), "第 2 帧是偶数帧，应该更新文本");
        assert!(should_display(10), "第 10 帧是偶数帧，应该更新文本");
        assert!(!should_display(3), "第 3 帧是奇数帧，不应该更新文本");
    }

    #[test]
    fn text_updated_on_second_frame() {
        let mut app = App::new();
        app.insert_resource(Frames::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_frame_text);
        app.update(); // 第 1 帧：奇数帧，文本保持 "0"
        app.update(); // 第 2 帧：偶数帧，文本应该更新
        let mut query = app.world_mut().query::<&Text>();
        let text = query.single(app.world()).unwrap();
        assert_eq!(text.0, "frame 2", "第 2 帧后文本应显示 frame 2");
        let frames = app.world().resource::<Frames>();
        assert_eq!(frames.0, 2, "跑了 2 帧，帧数计数应该是 2");
    }
}

// 提示：
// 1. 先运行 `bevylings run 2101`，观察文本只在哪类帧更新。
// 2. 想想 `%` 运算符：`x % 2` 的结果是 0 还是 1 代表偶数？
// 3. 改好后运行 `bevylings test 2101`，两个测试都通过就过关了。
