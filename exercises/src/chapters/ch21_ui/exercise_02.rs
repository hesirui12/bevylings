//! # 练习 21.02 —— 按钮：Button 与 Interaction
//!
//! 出处：https://bevy.org/examples-webgpu/ui/button/
//!
//! ## 概念
//! 要让一个 UI 节点可以被点击，只需给它加上 `Button` 组件。
//! Bevy 会自动更新它的 `Interaction` 组件：鼠标悬停是 `Hovered`、
//! 按住是 `Pressed`、平时是 `None`。我们在系统里查询这些状态，
//! 就可以改变按钮的颜色和文字。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2102` 会先遇到一个编译错误，改正后运行
//! `bevylings test 2102` 让测试通过。
//!
//! 小贴士：`Text` 是 `String` 的 newtype 外壳，`**text` 才是里面的字符串；
//! 只写一个 `*` 得到的是 `Text` 本身，类型对不上。

// I AM NOT DONE

use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// 生成一个按钮：尺寸、背景、边框，以及作为子节点的文字
fn spawn_button(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Button,
            Node {
                width: px(150),
                height: px(65),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(px(5)),
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON.into()),
            BorderColor::all(Color::BLACK),
        ))
        .with_child((
            Text::new("Button"),
            TextFont {
                font_size: FontSize::Px(24.0),
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ))
        .id()
}

/// 根据 Interaction 状态改变按钮颜色和文字
fn button_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // BUG: 这里只想修改文字内容（一个 String），但只解了一层引用，
                // 得到的是 Text 外壳本身，类型不匹配导致编译失败。
                *text = "Pressed".to_string();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                **text = "Hovered".to_string();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                **text = "Button".to_string();
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_button(&mut commands);
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 生成一个指定 Interaction 状态的按钮，跑一帧后返回 app
    fn run_with(interaction: Interaction) -> App {
        let mut app = App::new();
        app.add_systems(Startup, move |mut commands: Commands| {
            let entity = spawn_button(&mut commands);
            commands.entity(entity).insert(interaction);
        });
        app.add_systems(Update, button_system);
        app.update();
        app
    }

    #[test]
    fn pressed_button_turns_green() {
        let mut app = run_with(Interaction::Pressed);
        let mut query = app.world_mut().query_filtered::<&BackgroundColor, With<Button>>();
        assert_eq!(query.single(app.world()).unwrap().0, PRESSED_BUTTON);
        let mut text_query = app.world_mut().query::<&Text>();
        assert_eq!(text_query.single(app.world()).unwrap().0, "Pressed");
    }

    #[test]
    fn hovered_button_turns_gray() {
        let mut app = run_with(Interaction::Hovered);
        let mut query = app.world_mut().query_filtered::<&BackgroundColor, With<Button>>();
        assert_eq!(query.single(app.world()).unwrap().0, HOVERED_BUTTON);
        let mut text_query = app.world_mut().query::<&Text>();
        assert_eq!(text_query.single(app.world()).unwrap().0, "Hovered");
    }
}

// 提示：
// 1. 编译错误发生在 match 的一个分支里：`*text = ...` 的类型对不上。
// 2. `Text` 实现了 `Deref<Target = String>`，要拿到字符串需要两层解引用 `**text`。
// 3. 改好后运行 `bevylings test 2102`，两个测试都通过就过关了。
