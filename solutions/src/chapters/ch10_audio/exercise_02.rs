//! # 练习 10.02 —— 音量与循环：PlaybackSettings 与音量控制
//!
//! 出处：https://bevy.org/examples-webgpu/audio/audio_control/
//!
//! ## 概念
//! `PlaybackSettings` 描述“怎么播”：`PlaybackSettings::LOOP` 表示循环播放。
//! 播放一旦开始，Bevy 会给实体挂上 `AudioSink` 组件，我们用它控制音量：
//! `sink.volume().to_linear()` 读出当前音量（1.0 是原始音量），
//! `sink.set_volume(Volume::Linear(x))` 写入新音量。
//! 音量按“百分比”调整最直觉：提高 10% 就是把音量乘以 `1 + 10/100`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1002` 观察现象，改正后运行 `bevylings test 1002` 让测试通过。
//!
//! 小贴士：`=` 键提高音量、`-` 键降低音量；降低 10% 相当于乘以 `1 - 10/100`。

use bevy::{audio::Volume, prelude::*};

/// 标记正在播放的音乐。
#[derive(Component)]
struct MyMusic;

/// 按百分比调整音量：`percent` 为正表示提高，为负表示降低。
fn adjust_volume(volume: f32, percent: f32) -> f32 {
    volume * (1.0 + percent / 100.0)
}

/// 处理音量键：`=` 提高 10%，`-` 降低 10%。
fn volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut music: Query<&mut AudioSink, With<MyMusic>>,
) {
    let Ok(mut sink) = music.single_mut() else {
        return;
    };
    if keyboard_input.just_pressed(KeyCode::Equal) {
        let current = sink.volume().to_linear();
        sink.set_volume(Volume::Linear(adjust_volume(current, 10.0)));
    } else if keyboard_input.just_pressed(KeyCode::Minus) {
        let current = sink.volume().to_linear();
        sink.set_volume(Volume::Linear(adjust_volume(current, -10.0)));
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, volume)
        .run();
}

/// 启动时生成一个循环播放的音乐。
fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
        PlaybackSettings::LOOP,
        MyMusic,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase_volume_by_ten_percent() {
        assert!((adjust_volume(1.0, 10.0) - 1.1).abs() < 1e-6, "提高 10% 应为 1.1");
        assert!((adjust_volume(0.5, 100.0) - 1.0).abs() < 1e-6, "提高 100% 应翻倍");
    }

    #[test]
    fn decrease_volume_with_negative_percent() {
        assert!((adjust_volume(1.0, -10.0) - 0.9).abs() < 1e-6, "降低 10% 应为 0.9");
        assert!((adjust_volume(1.0, -100.0) - 0.0).abs() < 1e-6, "降低 100% 应为 0");
    }
}

// 提示：
// 1. 先运行 `bevylings run 1002`，按 `=` 键听听音量是变大还是变小。
// 2. “提高 10%” 的数学表达式是 `volume * (1 + 10 / 100)`，想一想 1 后面的符号。
// 3. 改好后运行 `bevylings test 1002`，测试全绿就过关了。
