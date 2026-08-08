//! # 练习 27.04 —— 拖拽拾取：拖动与放置
//!
//! 出处：https://bevy.org/examples-webgpu/picking/drag-drop-picking/
//!
//! ## 概念
//! 拾取事件里除了点击、悬停，还有一整套拖拽事件：
//! `Pointer<DragStart>`（开始拖）、`Pointer<DragEnd>`（结束拖）、
//! `Pointer<DragDrop>`（在目标上松开）。事件里带有 `dragged`（被拖的实体）
//! 和 `dropped`（松开的实体）字段，用来判断"这次拖拽是不是我发起的"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2704` 查看现象，改正后运行 `bevylings test 2704` 让测试通过。
//!
//! 小贴士：官方代码里用 `event.dragged == *button` 判断"拖的是不是自己的按钮"。

// I AM NOT DONE

use bevy::prelude::*;

/// 标记"可以从这里拖出元素"的按钮。
#[derive(Component)]
struct DraggableButton;

/// 判断这次拖拽是否来自我们的按钮。
/// 官方代码：`event.dragged == *button`。
fn should_accept(dragged: Entity, button: Entity) -> bool {
    // BUG: 判断反了，导致"不是从按钮发起的拖拽"也被当成自己的。
    dragged != button
}

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 红色按钮：从这里开始拖
    commands
        .spawn((
            DraggableButton,
            Node {
                width: px(150.0),
                height: px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
        ))
        .with_child((
            Text::new("Drag from me"),
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        ))
        .observe(on_drag_start);

    // 绿色放置区：拖到它上面松手就算"放下"
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.4, 0.1))),
            Transform::from_xyz(300.0, 0.0, 0.0),
        ))
        .observe(on_drag_drop);
}

/// 开始拖拽：如果是我们的按钮，把按钮颜色改亮一点。
fn on_drag_start(
    mut event: On<Pointer<DragStart>>,
    button: Single<Entity, With<DraggableButton>>,
    mut button_color: Single<&mut BackgroundColor, With<DraggableButton>>,
) {
    if should_accept(event.event_target(), *button) {
        button_color.0 = Color::srgb(1.0, 0.5, 0.0);
    }
    event.propagate(false);
}

/// 在放置区上松开：如果是我们的按钮，在落点生成一个小圆片。
fn on_drag_drop(
    mut event: On<Pointer<DragDrop>>,
    button: Single<Entity, With<DraggableButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if should_accept(event.dropped, *button) {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(25.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.6))),
            Transform::from_translation(event.hit.position.unwrap_or(Vec3::ZERO)),
            Pickable::IGNORE,
        ));
        event.propagate(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_drag_from_own_button() {
        let button = Entity::from_raw_u32(7).unwrap();
        assert!(
            should_accept(button, button),
            "从自己按钮发起的拖拽应该被接受"
        );
    }

    #[test]
    fn rejects_drag_from_others() {
        let button = Entity::from_raw_u32(7).unwrap();
        let other = Entity::from_raw_u32(8).unwrap();
        assert!(
            !should_accept(other, button),
            "别的实体发起的拖拽不该被接受"
        );
    }
}

// 提示：
// 1. `dragged` / `dropped` 存的是"被拖/被放下的实体"，`button` 是按钮实体的编号。
// 2. 拖拽应该**只有**来自按钮时才被接受，想想 `==` 和 `!=` 哪个是对的。
// 3. 修改后运行 `bevylings test 2704`，两个测试全绿就过关了。
