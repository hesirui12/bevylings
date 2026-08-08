//! # 练习 25.02 —— 动画着色器：在 WGSL 里读取时间
//!
//! 出处：https://bevy.org/examples-webgpu/shader/animate_shader/
//!
//! ## 概念
//! 想让画面动起来，一个常用技巧是"让颜色随时间变化"。
//! Bevy 每一帧都会把当前时间写进一个叫 `globals` 的 uniform 里，
//! 其中 `globals.time` 是程序启动以来的秒数。着色器里只要在开头写
//! `#import bevy_pbr::mesh_view_bindings::globals` 引入它，就能用它
//! 计算颜色：`sin(uv.x * 10.0 + time)` 会随着 time 增大而左右"流动"，
//! 看起来像波纹在动。
//!
//! 本练习的材质是空结构体：数据全部来自引擎提供的时间，不需要
//! 自己定义字段。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2502` 观察现象，改正后运行 `bevylings test 2502` 让测试通过。
//!
//! 小贴士：WGSL 字符串里拼错一个字母不会让 Rust 报错，但运行时
//! 着色器会编译失败；所以测试会直接检查字符串内容帮你发现。

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};
use std::sync::OnceLock;

/// 内联的 WGSL：根据 uv 和时间算出一个会"流动"的颜色。
const SHADER_SOURCE: &str = r#"
#import bevy_pbr::mesh_view_bindings::globals

@fragment
fn fragment_main(
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    let time = globals.time;
    let color = vec4(0.5 * sin(uv.x * 10.0 + time) + 0.5, 0.5, 0.5, 1.0);
    return color;
}
"#;

/// 全局缓存：内联着色器注册到 Assets<Shader> 后得到的句柄。
static INLINE_SHADER: OnceLock<Handle<Shader>> = OnceLock::new();

/// 空材质：所有数据都来自引擎提供的 globals。
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(INLINE_SHADER.get().cloned().unwrap())
    }
}

/// 把内联 WGSL 注册成着色器资产，句柄存进全局变量。
fn register_shader(asset_server: Res<AssetServer>) {
    let _ = INLINE_SHADER.set(asset_server.add(Shader::from_wgsl(
        SHADER_SOURCE,
        "animate_shader.wgsl",
    )));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // 一个用动画材质渲染的立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {})),
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
    fn shader_uses_globals_time() {
        // 正确写法是 globals.time（注意 globals 有个 s）。
        assert!(
            SHADER_SOURCE.contains("globals.time"),
            "应该读取 globals.time"
        );
    }

    #[test]
    fn shader_imports_time_uniform() {
        assert!(SHADER_SOURCE.contains("mesh_view_bindings::globals"));
        assert!(SHADER_SOURCE.contains("sin"));
    }
}

// 提示：
// 1. 先运行 `bevylings run 2502`，观察运行日志里的着色器报错。
// 2. 仔细看那一行 `let time = ...;`，与引入语句里的名字对一对。
// 3. 改好后运行 `bevylings test 2502`，两个测试全绿就过关了。
