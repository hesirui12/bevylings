//! # 练习 17.02 —— 状态切换：NextState 触发转移
//!
//! 出处：https://bevy.org/examples-webgpu/state/states/
//!
//! ## 概念
//! 上一练习讲了状态的生命周期；本练习看**切换**本身。
//! 系统里声明 `mut next_state: ResMut<NextState<AppState>>`，
//! 调用 `next_state.set(新状态)` 就会在状态转移阶段切到新状态，
//! 并触发对应的 `OnExit` / `OnEnter`。
//! `in_state(AppState::InGame)` 作为 `run_if` 条件，可以让某个系统
//! "只在游戏中运行"。
//!
//! 官方示例里按空格键在"菜单 / 游戏"之间来回切换，我们把它抽成纯函数。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1702` 查看现象，改正后运行 `bevylings test 1702` 让测试通过。
//!
//! 小贴士：`State<T>` 是"当前状态"，`NextState<T>` 是"打算切到的状态"。

// I AM NOT DONE

use bevy::prelude::*;

/// 应用状态：主菜单 / 游戏中。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

/// 按一下切换键后应该进入哪个状态？
fn toggle_state(current: AppState) -> AppState {
    // BUG: 两个分支都返回了当前状态，切换键按了等于没按。
    // "菜单"应该切到"游戏"，"游戏"应该切回"菜单"。
    match current {
        AppState::Menu => AppState::Menu,
        AppState::InGame => AppState::InGame,
    }
}

/// 记录游戏状态下系统运行的次数，用来验证 run_if 条件。
#[derive(Resource, Default)]
struct GameTicks(u32);

/// 只在游戏状态下运行的系统。
fn count_game_ticks(mut ticks: ResMut<GameTicks>) {
    ticks.0 += 1;
}

/// 按空格键切换状态。
fn toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(toggle_state(*state.get()));
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Update, toggle)
        .add_systems(Update, count_game_ticks.run_if(in_state(AppState::InGame)))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn toggle_flips_state() {
        assert_eq!(toggle_state(AppState::Menu), AppState::InGame);
        assert_eq!(toggle_state(AppState::InGame), AppState::Menu);
    }

    #[test]
    fn run_if_limits_system_to_in_game() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin);
        app.init_state::<AppState>();
        app.insert_resource(GameTicks::default());
        app.add_systems(Update, count_game_ticks.run_if(in_state(AppState::InGame)));
        app.update();
        assert_eq!(
            app.world().resource::<GameTicks>().0,
            0,
            "菜单状态下游戏系统不应该运行"
        );
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update();
        assert_eq!(
            app.world().resource::<GameTicks>().0,
            1,
            "进入游戏后每帧运行一次"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings test 1702`，看第一个测试断言了什么。
// 2. `toggle_state` 的职责是"翻面"：菜单 <-> 游戏。
// 3. 把两个分支改成互相切换，再运行 `bevylings test 1702`，全绿就过关。
