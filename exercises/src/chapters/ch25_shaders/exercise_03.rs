//! # 练习 25.03 —— 2D 着色器材质
//!
//! 出处：https://bevy.org/examples-webgpu/shader/shader_material_2d/
//!
//! ## 概念
//! 之前两题是 3D 立方体用的材质（`Material`）。2D 场景用的是
//! `Material2d`（注册插件也要换成 `Material2dPlugin`），其余思路
//! 完全一样：Rust 结构体定义数据，WGSL 负责画。
//!
//! 这一题我们把纹理也加进来：`#[texture(1)]` + `#[sampler(2)]`
//! 声明一张图片，着色器里用 `textureSample` 采样，再和 uniform
//! 颜色**叠加**出最终颜色。材质里对应字段用 `Option<Handle<Image>>`，
//! 没给图时 Bevy 会自动用一张占位图。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2503` 观察现象，改正后运行 `bevylings test 2503` 让测试通过。
//!
//! 小贴士：`textureSample(纹理, 采样器, uv)` 返回图上 (uv) 处的颜色；
//! "叠加"用加号，不是减号。

// I AM NOT DONE

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};
use std::sync::OnceLock;

/// 内联的 WGSL：颜色和纹理颜色叠加。
// BUG: 下面这段 WGSL 里把"叠加(+)"写成了"相减(-)"，
// 颜色和纹理叠加后反而会变暗、发黑。
const SHADER_SOURCE: &str = r#"
@group(2) @binding(0) var<uniform> my_color: vec4<f32>;
@group(2) @binding(1) var my_texture: texture_2d<f32>;
@group(2) @binding(2) var my_sampler: sampler;

@fragment
fn fragment_main(
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    var color = my_color - textureSample(my_texture, my_sampler, uv);
    return color;
}
"#;

/// 全局缓存：内联着色器注册到 Assets<Shader> 后得到的句柄。
static INLINE_SHADER: OnceLock<Handle<Shader>> = OnceLock::new();

/// 2D 自定义材质：一块颜色 + 一张纹理。
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(INLINE_SHADER.get().cloned().unwrap())
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Mask(0.5)
    }
}

/// 把内联 WGSL 注册成着色器资产，句柄存进全局变量。
fn register_shader(asset_server: Res<AssetServer>) {
    let _ = INLINE_SHADER.set(asset_server.add(Shader::from_wgsl(
        SHADER_SOURCE,
        "custom_material_2d.wgsl",
    )));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // 相机
    commands.spawn(Camera2d);

    // 一块用自定义材质渲染的正方形
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(CustomMaterial {
            color: LinearRgba::BLUE,
            color_texture: None,
        })),
        Transform::default().with_scale(Vec3::splat(128.0)),
    ));
}

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, Material2dPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, (register_shader, setup))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_combines_color_with_texture() {
        // 正确写法是用加号把颜色和纹理叠加起来。
        assert!(
            SHADER_SOURCE.contains("my_color + textureSample"),
            "应该用加号把颜色和纹理叠加"
        );
    }

    #[test]
    fn material_uses_mask_alpha_mode() {
        let material = CustomMaterial {
            color: LinearRgba::BLUE,
            color_texture: None,
        };
        assert_eq!(Material2d::alpha_mode(&material), AlphaMode2d::Mask(0.5));
    }
}

// 提示：
// 1. 先运行 `bevylings run 2503`，观察方块的颜色（相减后通常偏黑）。
// 2. 找到 WGSL 里计算最终颜色的那一行，检查运算符。
// 3. 改好后运行 `bevylings test 2503`，两个测试全绿就过关了。
