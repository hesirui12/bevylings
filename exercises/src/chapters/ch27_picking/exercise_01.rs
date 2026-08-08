//! # 练习 27.01 —— 基础拾取：Pointer<Over> 悬停变色
//!
//! 出处：https://bevy.org/examples-webgpu/picking/simple-picking/
//!
//! ## 概念
//! 鼠标移到某个实体上，这叫"悬停（Hover）"。Bevy 用"观察者（Observer）"
//! 来响应拾取事件：给实体挂一个 `.observe(...)`，事件发生时闭包就会被调用。
//! 本练习用到两个事件：`Pointer<Over>`（指针进入实体）和 `Pointer<Out>`（指针离开实体）。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2701` 查看现象，改正后运行 `bevylings test 2701` 让测试通过。
//!
//! 小贴士：`MeshPickingPlugin` 让 3D 网格也能被鼠标拾取；文字本身由 UI 拾取后端处理。

// I AM NOT DONE

use bevy::prelude::*;

/// 悬停状态 → 文字颜色：悬停时显示青色，平时白色。
fn color_for_hover(hovered: bool) -> Color {
    // BUG: 悬停和平时两种颜色写反了。
    if hovered {
        Color::WHITE
    } else {
        Color::srgb(0.0, 1.0, 1.0)
    }
}

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 提示文字：鼠标悬停会变色
    commands
        .spawn((
            Text::new("Hover me!"),
            Node {
                position_type: PositionType::Absolute,
                top: percent(12),
                left: percent(12),
                ..default()
            },
        ))
        .observe(
            |over: On<Pointer<Over>>, mut texts: Query<&mut TextColor>| {
                if let Ok(mut color) = texts.get_mut(over.entity) {
                    color.0 = color_for_hover(true);
                }
            },
        )
        .observe(|out: On<Pointer<Out>>, mut texts: Query<&mut TextColor>| {
            if let Ok(mut color) = texts.get_mut(out.entity) {
                color.0 = color_for_hover(false);
            }
        });

    // 一个可被拾取的立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 灯光与相机
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hover_turns_cyan() {
        assert_eq!(color_for_hover(true), Color::srgb(0.0, 1.0, 1.0));
    }

    #[test]
    fn idle_stays_white() {
        assert_eq!(color_for_hover(false), Color::WHITE);
    }
}

// 提示：
// 1. 运行 `bevylings run 2701`，悬停时文字颜色是反的（平时青色、悬停白色）。
// 2. 回到 `color_for_hover`，看看两个分支分别返回了什么颜色。
// 3. 把两个分支的颜色对调，再运行 `bevylings test 2701`。
