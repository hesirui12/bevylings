//! # 练习 09.04 —— 颜色动画：分段混色（Color Mixing）
//!
//! 出处：https://bevy.org/examples-webgpu/animation/color-animation/
//!
//! ## 概念
//! 让颜色随时间变化，一种简单可靠的办法是**混色（mixing）**：在几个
//! 关键颜色之间做线性过渡。Bevy 的 `Mix` 特征为每种颜色空间
//! （如 `Srgba`、`Hsla`）都提供了 `mix` 方法：
//! `a.mix(&b, t)` 返回 a 和 b 按比例 t 混合后的颜色。
//!
//! 关键颜色超过两个时，要先把总进度 t 拆成"第几段 + 段内进度"：
//! 4 个颜色有 3 个区间，`start_i = floor(t * 区间数)` 决定用哪两个
//! 颜色，`local_t` 决定在这一段里走到百分之多少。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0904` 观察现象，改正后运行 `bevylings test 0904` 让测试通过。
//!
//! 小贴士：`mix` 的第二个参数类型是 `&Self`（引用），第一次写容易漏 `&`。

// I AM NOT DONE

use bevy::prelude::*;

/// 在颜色列表上按进度 t 做分段混色。
/// t=0 得到第一个颜色，t=1 得到最后一个颜色，中间平滑过渡。
fn mix_colors(colors: &[Srgba], t: f32) -> Srgba {
    let intervals = (colors.len() - 1) as f32;
    // 第几段：比如 4 个颜色有 3 段，t=0.5 落在第 1 段（从 0 数起）
    let start_i = (t * intervals).floor().min(intervals - 1.0) as usize;
    // 段内进度：把 t 换算成这一小段里的 0..1
    let local_t = t * intervals - start_i as f32;

    // BUG: mix 要求第二个参数传引用（&Srgba），这里漏了 `&`，
    // 编译会报类型不匹配，把引用补上即可。
    colors[start_i].mix(colors[start_i + 1], local_t)
}

/// 要演示的方块。
#[derive(Component)]
struct ColorFade;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_colors)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::splat(150.0)),
        ColorFade,
    ));
}

fn animate_colors(time: Res<Time>, mut sprites: Query<&mut Sprite, With<ColorFade>>) {
    let palette = [
        Srgba::new(1.0, 0.2, 0.2, 1.0), // 红
        Srgba::new(1.0, 1.0, 0.2, 1.0), // 黄
        Srgba::new(0.2, 1.0, 0.2, 1.0), // 绿
        Srgba::new(0.2, 0.6, 1.0, 1.0), // 蓝
    ];
    let t = time.elapsed_secs().rem_euclid(2.0) / 2.0;
    for mut sprite in &mut sprites {
        sprite.color = mix_colors(&palette, t).into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn palette() -> [Srgba; 3] {
        [
            Srgba::new(0.0, 0.0, 0.0, 1.0), // 黑
            Srgba::new(1.0, 1.0, 1.0, 1.0), // 白
            Srgba::new(1.0, 0.0, 0.0, 1.0), // 红
        ]
    }

    #[test]
    fn starts_at_first_color() {
        assert_eq!(mix_colors(&palette(), 0.0), Srgba::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn ends_at_last_color() {
        assert_eq!(mix_colors(&palette(), 1.0), Srgba::new(1.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn mid_segment_mixes_two_colors() {
        // t=0.25：第一段的中点，黑和白各占一半 → 灰色
        let color = mix_colors(&palette(), 0.25);
        assert!((color.red - 0.5).abs() < 1e-4, "红通道应在黑白之间，实际 {color:?}");
        assert!((color.green - 0.5).abs() < 1e-4);
        assert!((color.blue - 0.5).abs() < 1e-4);
    }
}

// 提示：
// 1. 先读编译错误，看 mix 的第二个参数期望什么类型。
// 2. `mix` 的定义是 `fn mix(&self, other: &Self, factor: f32)`，参数都要对照一下。
// 3. 修改后运行 `bevylings test 0904`，测试全绿就算过关。
