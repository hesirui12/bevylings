//! # 练习 13.02 —— 菜单与状态：切换游戏画面
//!
//! 出处：https://bevy.org/examples/games/game-menu/
//!
//! ## 概念
//! 游戏往往有好几个"画面"：闪屏（Splash）、主菜单（Menu）、游戏（Game）。
//! Bevy 用**状态**（`States`）管理它们：状态枚举 + `NextState` 资源来切换。
//! 官方示例里闪屏等 1 秒后自动进菜单，菜单里点 "Play" 才进游戏。
//! 我们把这些"什么时候该切到哪个状态"的决策抽成小函数，
//! 状态切换就是 `next_state.set(目标状态)` 一行。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1302` 观察现象，改正后运行 `bevylings test 1302` 让测试通过。
//!
//! 小贴士：`GameState::default()` 是游戏启动时所在的状态。

// I AM NOT DONE

use bevy::prelude::*;

/// 游戏全局状态：闪屏 → 菜单 → 游戏。
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

/// 菜单按钮动作。
enum MenuAction {
    Play,
    Quit,
}

/// 闪屏计时结束后的下一个状态。
fn state_after_splash(finished: bool) -> GameState {
    if finished {
        // BUG: 闪屏（Splash）结束后应该进入主菜单（Menu），
        // 这里却直接跳进了游戏（Game），玩家会错过菜单。
        GameState::Game
    } else {
        GameState::Splash
    }
}

/// 按下菜单按钮后要切到哪个状态？`None` 表示退出。
fn state_for_action(action: MenuAction) -> Option<GameState> {
    match action {
        MenuAction::Play => Some(GameState::Game),
        MenuAction::Quit => None,
    }
}

/// 闪屏计时器：1 秒后自动切到菜单。
#[derive(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

/// 在闪屏状态计时，结束后切到菜单。
fn splash_countdown(
    mut next_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<SplashTimer>,
    time: Res<Time>,
) {
    let finished = timer.0.tick(time.delta()).is_finished();
    next_state.set(state_after_splash(finished));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SplashTimer>()
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, splash_countdown.run_if(in_state(GameState::Splash)))
        .run();
}

/// 生成相机。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splash_transitions_to_menu() {
        assert_eq!(state_after_splash(true), GameState::Menu);
        assert_eq!(state_after_splash(false), GameState::Splash);
    }

    #[test]
    fn play_button_starts_game() {
        assert_eq!(state_for_action(MenuAction::Play), Some(GameState::Game));
    }

    #[test]
    fn quit_button_leaves_menu() {
        assert_eq!(state_for_action(MenuAction::Quit), None);
    }

    #[test]
    fn default_state_is_splash() {
        assert_eq!(GameState::default(), GameState::Splash);
    }
}

// 提示：
// 1. 先看 `state_after_splash(true)` 应该返回哪个状态。
// 2. 菜单按钮 Play 之后玩家应该能玩到游戏（Game），Quit 则不切状态。
// 3. 改完运行 `bevylings test 1302`，四个测试全绿就过关。
