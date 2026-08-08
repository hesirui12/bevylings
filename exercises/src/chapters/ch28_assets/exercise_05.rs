//! # 练习 28.05 —— 资产设置：为图片指定采样方式
//!
//! 出处：https://bevy.org/examples-webgpu/asset/asset-settings/
//!
//! ## 概念
//! 同一份图片资产可以按不同的"加载设置"读取，最常用的是采样方式（sampler）：
//! - `ImageSampler::nearest()`：最近邻采样，边缘锐利，适合像素画；
//! - `ImageSampler::linear()`：线性过滤，画面平滑，适合普通照片。
//! 用 `.load_builder().with_settings(...).load(...)` 链式调用，
//! 可以在加载时临时修改设置，不需要改动文件本身。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2805` 查看现象，改正后运行 `bevylings test 2805` 让测试通过。
//!
//! 小贴士：像素画放大后出现"糊糊的"边缘，就是因为用了线性过滤。

// I AM NOT DONE

use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;

/// 像素画加载设置：放大后边缘保持锐利。
fn configure_pixel_art(settings: &mut ImageLoaderSettings) {
    // BUG: 这里设成了线性采样，像素画放大后会发糊。
    settings.sampler = ImageSampler::linear();
}

/// 普通照片加载设置：线性过滤，画面平滑。
fn configure_photo(settings: &mut ImageLoaderSettings) {
    settings.sampler = ImageSampler::linear();
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 方式一：默认设置加载（线性过滤）
    commands.spawn((
        Sprite {
            image: asset_server.load("bevy_pixel_dark.png"),
            custom_size: Some(Vec2::new(160.0, 120.0)),
            ..default()
        },
        Transform::from_xyz(-100.0, 0.0, 0.0),
    ));

    // 方式二：with_settings 指定最近邻采样
    commands.spawn((
        Sprite {
            image: asset_server
                .load_builder()
                .with_settings(configure_pixel_art)
                .load("bevy_pixel_dark_with_settings.png"),
            custom_size: Some(Vec2::new(160.0, 120.0)),
            ..default()
        },
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));

    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_art_uses_nearest() {
        let mut settings = ImageLoaderSettings::default();
        configure_pixel_art(&mut settings);
        assert_eq!(
            settings.sampler,
            ImageSampler::nearest(),
            "像素画应该用最近邻采样"
        );
    }

    #[test]
    fn photo_uses_linear() {
        let mut settings = ImageLoaderSettings::default();
        configure_photo(&mut settings);
        assert_eq!(settings.sampler, ImageSampler::linear());
    }
}

// 提示：
// 1. 先运行 `bevylings run 2805`，左边的像素画放大了很模糊。
// 2. `configure_pixel_art` 是 `with_settings` 的回调，改的是 `settings.sampler` 字段。
// 3. 把线性采样改成最近邻采样，再运行 `bevylings test 2805`。
