//! # 练习 27.02 —— 网格拾取：悬停与按下高亮
//!
//! 出处：https://bevy.org/examples-webgpu/picking/mesh-picking/
//!
//! ## 概念
//! 上一练习只改变了文字颜色；这一练习让"被拾取的网格"本身换材质颜色。
//! 网格材质是共享的资产（`Assets<StandardMaterial>`），可以先按状态
//! 准备好几份材质，再在事件里把实体手里的材质句柄换成对应的那一个。
//! 这里用一个 `PickState` 枚举表示"普通 / 悬停 / 按下"三种状态，
//! 由 `color_for_state` 决定每种状态的颜色。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2702` 查看现象，改正后运行 `bevylings test 2702` 让测试通过。
//!
//! 小贴士：`Pointer<Press>` 表示指针按下，`Pointer<Out>` 表示指针离开。

use bevy::prelude::*;

/// 拾取状态：普通、悬停、按下。
#[derive(Clone, Copy, PartialEq, Eq)]
enum PickState {
    Normal,
    Hovered,
    Pressed,
}

/// 每种状态对应的材质颜色。
fn color_for_state(state: PickState) -> Color {
    match state {
        PickState::Normal => Color::WHITE,
        PickState::Hovered => Color::srgb(0.0, 1.0, 1.0),
        PickState::Pressed => Color::srgb(1.0, 1.0, 0.0),
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
    // 按状态预先准备好三种材质
    let normal_material = materials.add(color_for_state(PickState::Normal));
    let hover_material = materials.add(color_for_state(PickState::Hovered));
    let pressed_material = materials.add(color_for_state(PickState::Pressed));

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
            MeshMaterial3d(normal_material.clone()),
            Transform::from_xyz(0.0, 0.8, 0.0),
        ))
        .observe(
            move |over: On<Pointer<Over>>, mut mats: Query<&mut MeshMaterial3d<StandardMaterial>>| {
                if let Ok(mut mat) = mats.get_mut(over.entity) {
                    mat.0 = hover_material.clone();
                }
            },
        )
        .observe(
            move |out: On<Pointer<Out>>, mut mats: Query<&mut MeshMaterial3d<StandardMaterial>>| {
                if let Ok(mut mat) = mats.get_mut(out.entity) {
                    mat.0 = normal_material.clone();
                }
            },
        )
        .observe(
            move |press: On<Pointer<Press>>, mut mats: Query<&mut MeshMaterial3d<StandardMaterial>>| {
                if let Ok(mut mat) = mats.get_mut(press.entity) {
                    mat.0 = pressed_material.clone();
                }
            },
        );

    // 灯光与相机
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 6.0, 12.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressed_color_is_yellow() {
        assert_eq!(color_for_state(PickState::Pressed), Color::srgb(1.0, 1.0, 0.0));
    }

    #[test]
    fn hover_color_is_cyan() {
        assert_eq!(color_for_state(PickState::Hovered), Color::srgb(0.0, 1.0, 1.0));
        assert_eq!(color_for_state(PickState::Normal), Color::WHITE);
    }
}

// 提示：
// 1. 运行 `bevylings run 2702`，按住立方体时颜色没有任何变化。
// 2. 对照 `PickState::Hovered` 那一支的颜色，检查 `Pressed` 分支返回了什么。
// 3. 把 `Pressed` 分支改成高亮颜色，再运行 `bevylings test 2702`。
