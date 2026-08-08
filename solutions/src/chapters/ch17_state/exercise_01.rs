//! # 练习 17.01 —— States 基础：init_state 与 OnEnter/OnExit
//!
//! 出处：https://bevy.org/examples-webgpu/state/states/
//!
//! ## 概念
//! 游戏通常分成几个"画面"：主菜单、游戏中…… Bevy 用**状态**（States）管理。
//! - `#[derive(States)]` 把枚举变成状态类型；
//! - `app.init_state::<AppState>()` 注册状态，默认值由 `#[default]` 决定；
//! - `OnEnter(状态)` / `OnExit(状态)`：进入 / 离开该状态时各执行一次，
//!   适合做"布置画面"和"清理画面"；
//! - 想切换状态，就往 `NextState` 资源里 `set` 新值，下一帧生效。
//!
//! 本练习：进入 Menu 时挂上菜单资源，离开 Menu 时把它摘掉。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1701` 查看现象，改正后运行 `bevylings test 1701` 让测试通过。
//!
//! 小贴士：`OnEnter` 只在"进入"那一刻运行一次，而 `Update` 里的系统每帧都跑。

use bevy::prelude::*;

/// 应用级状态：主菜单 / 游戏中。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

/// 菜单画面特有的资源：进入菜单时创建，离开时删除。
#[derive(Resource, Default)]
struct MenuOpen(bool);

/// 进入 Menu 状态时调用：创建菜单资源。
fn setup_menu(mut commands: Commands) {
    commands.insert_resource(MenuOpen(true));
}

/// 离开 Menu 状态时调用：删除菜单资源。
fn cleanup_menu(mut commands: Commands) {
    commands.remove_resource::<MenuOpen>();
}

/// 菜单里点"开始"按钮后切换到游戏状态。
fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::state::app::StatesPlugin;

    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins(StatesPlugin);
        app.init_state::<AppState>();
        app.add_systems(OnEnter(AppState::Menu), setup_menu);
        app.add_systems(OnExit(AppState::Menu), cleanup_menu);
        app
    }

    #[test]
    fn entering_menu_creates_menu_resource() {
        let mut app = build_app();
        app.update(); // 第一次 update 会执行初始状态 Menu 的 OnEnter
        assert!(
            app.world().contains_resource::<MenuOpen>(),
            "进入 Menu 状态后应该创建菜单资源"
        );
    }

    #[test]
    fn leaving_menu_removes_menu_resource() {
        let mut app = build_app();
        app.update();
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update(); // 触发 Menu -> InGame 转移
        assert!(
            !app.world().contains_resource::<MenuOpen>(),
            "离开 Menu 后菜单资源应该被清理"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 1701`，编译器会提示 AppState 不满足某个 trait 的要求。
// 2. 状态枚举需要多 derive 一个 trait，它和"状态"是同一个英文单词。
// 3. 改好后运行 `bevylings test 1701`，两个测试全绿就过关。
