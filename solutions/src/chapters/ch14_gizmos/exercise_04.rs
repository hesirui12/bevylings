//! # 练习 14.04 —— 变换 Gizmo：Translate / Rotate / Scale
//!
//! 出处：https://bevy.org/examples-webgpu/gizmos/transform-gizmo/
//!
//! ## 概念
//! 变换 Gizmo 是 3D 编辑器（如 Blender、Unity）里常见的"拖拽手柄"：
//! 选中一个物体后，可以拖动手柄来平移、旋转、缩放它。Bevy 提供
//! `TransformGizmoPlugin` 实现它，几个关键类型：
//! - `TransformGizmoFocus`：标记当前选中的物体；
//! - `TransformGizmoMode`：当前是平移（`Translate`）、旋转（`Rotate`）
//!   还是缩放（`Scale`）；
//! - `TransformGizmoSpace`：用世界坐标轴还是物体局部坐标轴。
//!
//! 官方示例按 1 / 2 / 3 键切换模式，按 X 键在世界/局部之间切换。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1404` 观察现象，改正后运行 `bevylings test 1404` 让测试通过。
//!
//! 小贴士：`TransformGizmoMode` 的变体是 `Translate` / `Rotate` / `Scale`，
//! 别把 `Translate` 拼成 `Translation`。

use bevy::gizmos::transform_gizmo::{TransformGizmoMode, TransformGizmoSpace};
use bevy::prelude::*;

/// 按 1→2→3 循环切换操作模式。
fn next_mode(mode: TransformGizmoMode) -> TransformGizmoMode {
    match mode {
        TransformGizmoMode::Translate => TransformGizmoMode::Rotate,
        TransformGizmoMode::Rotate => TransformGizmoMode::Scale,
        TransformGizmoMode::Scale => TransformGizmoMode::Translate,
    }
}

/// 在世界坐标轴与物体局部坐标轴之间切换。
fn toggle_space(space: TransformGizmoSpace) -> TransformGizmoSpace {
    match space {
        TransformGizmoSpace::World => TransformGizmoSpace::Local,
        TransformGizmoSpace::Local => TransformGizmoSpace::World,
    }
}

/// 当前 gizmo 状态：模式 + 坐标系。
#[derive(Resource)]
struct GizmoState {
    mode: TransformGizmoMode,
    space: TransformGizmoSpace,
}

/// 处理键盘：1/2/3 直接切模式，Tab 循环切换，X 切换坐标系。
fn gizmo_mode_keys(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<GizmoState>) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        state.mode = TransformGizmoMode::Translate;
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        state.mode = TransformGizmoMode::Rotate;
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        state.mode = TransformGizmoMode::Scale;
    }
    if keyboard.just_pressed(KeyCode::Tab) {
        state.mode = next_mode(state.mode);
    }
    if keyboard.just_pressed(KeyCode::KeyX) {
        state.space = toggle_space(state.space);
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GizmoState {
            mode: TransformGizmoMode::Translate,
            space: TransformGizmoSpace::World,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, gizmo_mode_keys)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_cycles_forward() {
        assert_eq!(
            next_mode(TransformGizmoMode::Translate),
            TransformGizmoMode::Rotate
        );
        assert_eq!(
            next_mode(TransformGizmoMode::Rotate),
            TransformGizmoMode::Scale
        );
        assert_eq!(
            next_mode(TransformGizmoMode::Scale),
            TransformGizmoMode::Translate
        );
    }

    #[test]
    fn space_toggles_back_and_forth() {
        assert_eq!(
            toggle_space(TransformGizmoSpace::World),
            TransformGizmoSpace::Local
        );
        assert_eq!(
            toggle_space(TransformGizmoSpace::Local),
            TransformGizmoSpace::World
        );
    }
}

// 提示：
// 1. 先读编译错误，看看它说"找不到哪个变体"。
// 2. 枚举变体名要完全一致：`Translate` 表示平移，别和"翻译"的 `Translation` 混了。
// 3. 修改后运行 `bevylings test 1404`，测试全绿就算过关。
