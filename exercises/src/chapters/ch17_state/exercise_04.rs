//! # 练习 17.04 —— 子状态：SubStates
//!
//! 出处：https://bevy.org/examples-webgpu/state/sub_states/
//!
//! ## 概念
//! **子状态**（SubStates）是"只在某个状态里存在"的状态。
//! 用 `#[derive(SubStates)]` + `#[source(AppState = AppState::InGame)]`
//! 声明：只有根状态 AppState 处于 InGame 时，`IsPaused` 这个状态才存在。
//!
//! 这比把所有情况都揉进一个枚举更清晰：菜单里谈"暂停"没有意义，
//! 所以菜单状态下 `IsPaused` 压根不存在；进入游戏后它自动出现，
//! 默认值是 `Running`（运行中）。
//!
//! 官方示例按空格键切换"运行 / 暂停"，我们抽成纯函数 `toggle_pause`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1704` 查看现象，改正后运行 `bevylings test 1704` 让测试通过。
//!
//! 小贴士：子状态和普通状态一样，用 `NextState<IsPaused>` 切换。

// I AM NOT DONE

use bevy::prelude::*;

/// 根状态：菜单 / 游戏中。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

/// 子状态：只在游戏中存在。Running = 运行中，Paused = 已暂停。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
enum IsPaused {
    #[default]
    Running,
    Paused,
}

/// 按切换键后，暂停状态变成什么？
fn toggle_pause(current: IsPaused) -> IsPaused {
    // BUG: 两个分支都直接返回当前值，切换键按了等于没按。
    // "运行中"应该变成"已暂停"，反之亦然。
    match current {
        IsPaused::Running => IsPaused::Running,
        IsPaused::Paused => IsPaused::Paused,
    }
}

/// 按空格键切换暂停状态（只在游戏中生效）。
fn toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<IsPaused>>,
    mut next_state: ResMut<NextState<IsPaused>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(toggle_pause(*state.get()));
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_sub_state::<IsPaused>()
        .add_systems(Update, toggle.run_if(in_state(AppState::InGame)))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn toggle_flips_pause() {
        assert_eq!(toggle_pause(IsPaused::Running), IsPaused::Paused);
        assert_eq!(toggle_pause(IsPaused::Paused), IsPaused::Running);
    }

    #[test]
    fn substate_only_exists_in_game() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin);
        app.init_state::<AppState>();
        app.add_sub_state::<IsPaused>();
        app.update(); // 菜单：子状态不存在
        assert!(
            !app.world().contains_resource::<State<IsPaused>>(),
            "菜单状态下不应该有 IsPaused"
        );
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update(); // 进入游戏：子状态出现，默认 Running
        let state = app.world().resource::<State<IsPaused>>();
        assert_eq!(state.get(), &IsPaused::Running);
    }
}

// 提示：
// 1. 先运行 `bevylings test 1704`，第一个测试会失败：切换键没有改变状态。
// 2. `toggle_pause` 的职责是"翻面"：Running <-> Paused。
// 3. 把两个分支改成互相切换，测试全绿就过关。
