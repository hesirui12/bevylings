//! # 练习 10.03 —— 音效播放：按空格发射一次性音效
//!
//! 出处：https://bevy.org/examples-webgpu/audio/play_sound_effect/
//!
//! ## 概念
//! 音效通常很短，播一次就结束。做法是：每次触发时**临时生成**一个
//! `AudioPlayer` 实体，并配上 `PlaybackSettings::DESPAWN` —— 播完它会自动销毁，
//! 我们不用手动清理。
//! 音效的句柄放在资源 `SoundEffect` 里（资源可以装任何数据，包括句柄），
//! 系统里通过 `Res<SoundEffect>` 取出来用。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1003` 观察现象，改正后运行 `bevylings test 1003` 让测试通过。
//!
//! 小贴士：资源是“包装盒”，句柄是“里面的东西”；`AudioPlayer::new` 只接受句柄。

// I AM NOT DONE

use bevy::prelude::*;

/// 音效资源：里面装的是音频句柄。
#[derive(Resource)]
struct SoundEffect(Handle<AudioSource>);

/// 音频文件在资源第一次初始化时加载（`FromWorld` 里可以访问 World 的资源）。
impl FromWorld for SoundEffect {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        SoundEffect(asset_server.load("sounds/breakout_collision.ogg"))
    }
}

/// 空格键是否刚被按下（抽成纯函数方便测试）。
fn should_play(keys: &ButtonInput<KeyCode>) -> bool {
    keys.just_pressed(KeyCode::Space)
}

/// 按空格时生成一个会自动销毁的音效播放器。
fn keyboard_event(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    sound_effect: Res<SoundEffect>,
    mut commands: Commands,
) {
    if should_play(&keyboard_input) {
        commands.spawn((
            // BUG: 这里想取出资源里装的“句柄”给 AudioPlayer，
            // 却把整个 SoundEffect 资源直接传了进去，参数类型对不上。
            AudioPlayer::new(sound_effect),
            PlaybackSettings::DESPAWN,
        ));
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SoundEffect>()
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_event)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_key_triggers_effect() {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Space);
        assert!(should_play(&keys), "刚按下空格应该触发");
        keys.release(KeyCode::Space);
        // 模拟"下一帧"：clear() 会清掉本帧的 just_pressed 记录
        keys.clear();
        assert!(!should_play(&keys), "松开后不算“刚按下”");
    }

    #[test]
    fn resource_stores_a_handle() {
        let effect = SoundEffect(Handle::default());
        let _: &Handle<AudioSource> = &effect.0;
        assert!(effect.0 == Handle::default(), "句柄应原样保存在资源里");
    }
}

// 提示：
// 1. 看看 `AudioPlayer::new` 的参数类型：它要的是 `Handle<AudioSource>`。
// 2. 资源 `SoundEffect(句柄)` 里的句柄要用 `.0` 取出来；还需要 `.clone()` 一份。
// 3. 改好后运行 `bevylings test 1003`，测试全绿就过关了。
