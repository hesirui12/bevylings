//! # 练习 21.07 —— 计数器组件：按钮点击计数
//!
//! 出处：https://bevy.org/examples-webgpu/ui/feathers_counter/
//!
//! ## 概念
//! 把按钮和状态结合起来，就能做出最简单的交互：点击按钮让一个数字 +1。
//! 数字放在 `Resource`（资源）里，按钮的点击状态放在 `Interaction` 里。
//! 查询时用 `Changed<Interaction>` 过滤——只有"状态刚刚变化"的按钮
//! 才会被处理，这样按住不放不会疯狂累加。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2107`，改正后运行 `bevylings test 2107` 让测试通过。
//!
//! 小贴士：`*interaction == Interaction::Pressed` 判断"是否被按下"；
//! 注意 `interaction` 是从查询里借来的引用，要加 `*` 才能比较。

use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// 计数器（点击次数）
#[derive(Resource, Default)]
struct Counter(i32);

/// 标记组件：指向显示计数的文本
#[derive(Component)]
struct CounterText;

/// 生成一个"点击 +1"的按钮
fn spawn_button(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Button,
            Node {
                width: px(120),
                height: px(50),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON.into()),
        ))
        .with_child((
            Text::new("+1"),
            TextFont {
                font_size: FontSize::Px(24.0),
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id()
}

/// 按钮状态变化时，把计数器 +1
fn increment_on_press(
    interaction: Query<&Interaction, Changed<Interaction>>,
    mut counter: ResMut<Counter>,
) {
    for interaction in &interaction {
        if *interaction == Interaction::Pressed {
            counter.0 += 1;
        }
    }
}

/// 把计数显示到文本上
fn update_counter_text(
    counter: Res<Counter>,
    mut query: Query<&mut Text, With<CounterText>>,
) {
    for mut text in &mut query {
        **text = format!("{}", counter.0);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_button(&mut commands);
    commands.spawn((
        Text::new("0"),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        CounterText,
        Node {
            position_type: PositionType::Absolute,
            top: px(80),
            ..default()
        },
    ));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Counter::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (increment_on_press, update_counter_text))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(Counter::default());
        app.add_systems(Startup, |mut commands: Commands| {
            let entity = spawn_button(&mut commands);
            commands.entity(entity).insert(Interaction::Pressed);
        });
        app.add_systems(Update, increment_on_press);
        app
    }

    #[test]
    fn pressing_button_increments_counter() {
        let mut app = build_app();
        app.update();
        let counter = app.world().resource::<Counter>();
        assert_eq!(counter.0, 1, "点击一次计数应为 1");
    }

    #[test]
    fn counter_only_changes_when_interaction_changes() {
        let mut app = build_app();
        app.update();
        app.update(); // Interaction 没有再变化，不应重复计数
        let counter = app.world().resource::<Counter>();
        assert_eq!(counter.0, 1, "状态没变时不应重复累加");
    }

    #[test]
    fn text_shows_counter_value() {
        let mut app = App::new();
        app.insert_resource(Counter(7));
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_counter_text);
        app.update();
        let mut query = app.world_mut().query_filtered::<&Text, With<CounterText>>();
        let text = query.single(app.world()).unwrap();
        assert_eq!(text.0, "7", "文本应显示计数器的值");
    }
}

// 提示：
// 1. 先想清楚：点击按钮是想让计数变大还是变小？
// 2. 问题在 `increment_on_press` 里的一行：`+=` 被写成了 `-=`。
// 3. 改好后运行 `bevylings test 2107`，三个测试都通过就过关了。
