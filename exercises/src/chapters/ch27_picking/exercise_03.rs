//! # 练习 27.03 —— Sprite 拾取：修改精灵颜色
//!
//! 出处：https://bevy.org/examples-webgpu/picking/sprite-picking/
//!
//! ## 概念
//! 2D 精灵（`Sprite`）默认就能被拾取（Bevy 的精灵拾取后端会按"不透明像素"
//! 判断命中）。给精灵挂上 `.observe(...)` 就能响应拾取事件。
//! 本练习在事件里通过 `Query<&mut Sprite>` 拿到精灵的可变引用，
//! 然后修改 `sprite.color`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2703` 查看现象，改正后运行 `bevylings test 2703` 让测试通过。
//!
//! 小贴士：`Query<&Sprite>` 是只读查询；想修改组件必须写成 `Query<&mut Sprite>`。

// I AM NOT DONE

use bevy::prelude::*;

/// 悬停时变青色，平时保持白色。
fn color_while_hovered(hovered: bool) -> Color {
    if hovered {
        Color::srgb(0.0, 1.0, 1.0)
    } else {
        Color::srgb(1.0, 1.0, 1.0)
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Sprite::from_image(asset_server.load("branding/bevy_bird_dark.png")),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Pickable::default(),
        ))
        .observe(recolor_on::<Pointer<Over>>(color_while_hovered(true)))
        .observe(recolor_on::<Pointer<Out>>(color_while_hovered(false)));
}

/// 生成一个"把目标实体改色"的观察者闭包。
// BUG: 查询写成了只读的 `Query<&Sprite>`，拿到的 `sprite` 是
// 不可变引用，`sprite.color = color` 这行编译不过。
fn recolor_on<E: EntityEvent + std::fmt::Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(On<E>, Query<&Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.event_target()) else {
            return;
        };
        sprite.color = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hover_turns_cyan() {
        assert_eq!(color_while_hovered(true), Color::srgb(0.0, 1.0, 1.0));
    }

    #[test]
    fn idle_stays_white() {
        assert_eq!(color_while_hovered(false), Color::srgb(1.0, 1.0, 1.0));
    }
}

// 提示：
// 1. 先修编译错误：只读查询拿到的是 `&Sprite`，改不了颜色。
// 2. 检查 `recolor_on` 的返回类型里查询是 `&` 还是 `&mut`。
// 3. 改成可变查询后运行 `bevylings run 2703`，悬停小鸟会变青色，测试也全绿。
