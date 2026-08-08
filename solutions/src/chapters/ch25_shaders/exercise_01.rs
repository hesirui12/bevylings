//! # 练习 25.01 —— 自定义材质：着色器与 uniform
//!
//! 出处：https://bevy.org/examples-webgpu/shader/shader_material/
//!
//! ## 概念
//! 想让物体长得不一样，得给它配"自定义材质（Material）"。
//! 材质 = 一份数据（Rust 结构体）+ 一段着色器（WGSL 代码）。
//! `#[uniform(0)]` 告诉 Bevy：把 `color` 字段打包成 GPU 上的一个
//! uniform 变量，交给着色器里 `@binding(0)` 的声明使用。
//!
//! 官方示例把 WGSL 放在单独的 `.wgsl` 文件里；这里我们**直接内联**
//! 成 Rust 的字符串常量，一个文件就能看懂全部内容。因为
//! `fragment_shader()` 是静态方法（拿不到资源），我们先把内联源码
//! 注册成着色器资产，把句柄存进全局变量，再让 `fragment_shader()`
//! 返回这个句柄。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2501` 观察现象，改正后运行 `bevylings test 2501` 让测试通过。
//!
//! 小贴士：`LinearRgba` 是"线性空间"的颜色，`LinearRgba::BLUE` 是现成的蓝色常量。

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};
use std::sync::OnceLock;

/// 内联的 WGSL 片段着色器：直接把 uniform 颜色画出来。
const SHADER_SOURCE: &str = r#"
@group(3) @binding(0) var<uniform> my_color: vec4<f32>;

@fragment
fn fragment_main(
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    return my_color;
}
"#;

/// 全局缓存：内联着色器注册到 Assets<Shader> 后得到的句柄。
static INLINE_SHADER: OnceLock<Handle<Shader>> = OnceLock::new();

/// 我们的自定义材质：只有一块颜色。
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(INLINE_SHADER.get().cloned().unwrap())
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

/// 把内联 WGSL 注册成着色器资产，句柄存进全局变量。
fn register_shader(asset_server: Res<AssetServer>) {
    let _ = INLINE_SHADER.set(asset_server.add(Shader::from_wgsl(
        SHADER_SOURCE,
        "custom_material.wgsl",
    )));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // 一个用自定义材质渲染的立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::BLUE,
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, (register_shader, setup))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_mode_returns_blend() {
        let material = CustomMaterial {
            color: LinearRgba::BLUE,
            alpha_mode: AlphaMode::Blend,
        };
        assert_eq!(Material::alpha_mode(&material), AlphaMode::Blend);
    }

    #[test]
    fn shader_source_is_inlined() {
        // 内联着色器里必须同时出现 uniform 绑定和输出颜色的语句。
        assert!(SHADER_SOURCE.contains("@binding(0)"));
        assert!(SHADER_SOURCE.contains("my_color"));
        assert!(SHADER_SOURCE.contains("@fragment"));
    }
}

// 提示：
// 1. 先运行 `bevylings run 2501`，看编译错误指向哪一行。
// 2. `alpha_mode()` 的返回类型是 `AlphaMode`，返回 `self.color`（LinearRgba）类型不匹配。
// 3. 改好后运行 `bevylings test 2501`，两个测试全绿就过关了。
