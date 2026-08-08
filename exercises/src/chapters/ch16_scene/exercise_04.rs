//! # 练习 16.04 —— 世界序列化：把 World 存成文本再读回来
//!
//! 出处：https://bevy.org/examples-webgpu/scene/world_serialization/
//!
//! ## 概念
//! Bevy 的整个世界（World）都能序列化：实体、组件、资源都可以导出成
//! RON 文本（一种类似 JSON 的格式），之后随时再加载回来。
//! 前提是这些类型能被**反射**：derive `Reflect`，组件再加
//! `#[reflect(Component)]`，资源加 `#[reflect(Resource)]`。
//!
//! 序列化流程分两步：
//! - `DynamicWorld::from_world_with(&world, &registry)` 抓取世界里的数据；
//! - `.serialize(&registry)` 输出 RON 字符串。
//!
//! 本练习只做"抓数据 + 序列化"这两步（官方示例还会写文件、再读回来）。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1604` 查看现象，改正后运行 `bevylings test 1604` 让测试通过。
//!
//! 小贴士：序列化 = 把内存里的数据变成文本；反序列化 = 把文本变回数据。

// I AM NOT DONE

use bevy::{prelude::*, reflect::TypeRegistry};

/// 一个可序列化的组件：两个 f32 字段。
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ComponentA {
    x: f32,
    y: f32,
}

/// 一个可序列化的资源：分数。
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct ResourceA {
    score: u32,
}

/// 往一个临时世界里填入示例数据，准备序列化。
fn build_world(scene_world: &mut World, score: u32) {
    scene_world.spawn(ComponentA { x: 1.0, y: 2.0 });
    scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
    // BUG: 这里把分数写死成 0，完全忽略了传入的 score 参数，
    // 调用方传 99 也存不进去。
    scene_world.insert_resource(ResourceA { score: 0 });
}

/// 把世界序列化成 RON 文本。
fn serialize_world(world: &World, registry: &TypeRegistry) -> String {
    let dynamic = DynamicWorld::from_world_with(world, registry);
    dynamic.serialize(registry).unwrap()
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, save_world)
        .run();
}

/// 启动时把一个示例世界序列化并打印到控制台。
fn save_world(world: &mut World) {
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let mut scene_world = World::new();
    build_world(&mut scene_world, 99);
    let registry = type_registry.read();
    info!("{}", serialize_world(&scene_world, &registry));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_registry() -> AppTypeRegistry {
        let mut registry = AppTypeRegistry::default();
        registry.write().register::<ComponentA>();
        registry.write().register::<ResourceA>();
        registry
    }

    #[test]
    fn serialized_world_contains_components() {
        let registry = build_registry();
        let mut world = World::new();
        build_world(&mut world, 99);
        let ron = serialize_world(&world, &registry.read());
        assert!(ron.contains("ComponentA"), "RON 里应该有组件类型: {ron}");
        assert!(ron.contains("x: 1.0"), "RON 里应该有组件字段: {ron}");
    }

    #[test]
    fn score_parameter_is_respected() {
        let registry = build_registry();
        let mut world = World::new();
        build_world(&mut world, 99);
        let ron = serialize_world(&world, &registry.read());
        assert!(ron.contains("score: 99"), "分数应该是 99，实际: {ron}");
        assert!(!ron.contains("score: 0"), "分数不应该被写死成 0");
    }
}

// 提示：
// 1. 先运行 `bevylings test 1604`，第二个测试会失败：分数没有用上传入的参数。
// 2. `build_world` 的第二个参数 score 就是要存进 ResourceA 的分数。
// 3. 改好后 `bevylings run 1604` 会在控制台打印带 `score: 99` 的 RON 文本。
