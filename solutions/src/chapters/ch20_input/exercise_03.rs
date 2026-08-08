//! # 练习 20.03 —— 触摸：Touches 资源与触摸阶段
//!
//! 出处：https://bevy.org/examples-webgpu/input/touch-input/
//!
//! ## 概念
//! 触摸屏和鼠标不一样：可能同时有好几根手指按在屏幕上，
//! 所以 Bevy 用 `Touches` 资源管理"所有手指"，每根手指有唯一的 `id`。
//! 每根手指都会经历几个阶段（`TouchPhase`）：
//! `Started`（刚按下）→ `Moved`（滑动）→ `Ended`（松开）或 `Canceled`（被系统取消）。
//!
//! 本练习用两个不同的迭代器区分"刚按下的手指"和"所有正按着的手指"：
//! - `iter_just_pressed()`：只遍历**这一帧刚按下**的手指。
//! - `iter()`：遍历**所有当前正按着**的手指（包括上一帧就在按的）。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2003` 查看现象，改正后运行 `bevylings test 2003` 让测试通过。
//!
//! 小贴士：手指按住了 3 秒，`just_pressed` 只有第 1 帧为真；`iter` 则 3 秒里每帧都有它。

use bevy::{input::InputPlugin, prelude::*};

/// 记录"刚按下的手指数"和"当前按着的手指数"，方便测试观察。
#[derive(Resource, Default)]
struct TouchCount {
    just_pressed: u32,
    pressed: u32,
}

/// 统计当前触摸状态。
fn count_touches(touches: Res<Touches>, mut count: ResMut<TouchCount>) {
    count.just_pressed = touches.iter_just_pressed().count() as u32;
    count.pressed = touches.iter().count() as u32;
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TouchCount::default())
        .add_systems(Update, count_touches)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::touch::TouchPhase;

    fn build_app() -> App {
        let mut app = App::new();
        // InputPlugin 负责把 TouchInput 消息整理进 Touches 资源
        app.add_plugins(InputPlugin);
        app.insert_resource(TouchCount::default());
        app.add_systems(Update, count_touches);
        app
    }

    fn press_finger(app: &mut App, id: u64) {
        app.world_mut()
            .resource_mut::<Messages<TouchInput>>()
            .write(TouchInput {
                phase: TouchPhase::Started,
                position: Vec2::new(1.0, 2.0),
                window: Entity::PLACEHOLDER,
                force: None,
                id,
            });
    }

    #[test]
    fn just_pressed_only_on_the_first_frame() {
        let mut app = build_app();
        press_finger(&mut app, 7);
        app.update(); // 第一帧：手指刚按下
        let count = app.world().resource::<TouchCount>();
        assert_eq!(count.just_pressed, 1, "刚按下 1 根手指");

        app.update(); // 第二帧：手指还按着，但没有新事件
        let count = app.world().resource::<TouchCount>();
        assert_eq!(count.just_pressed, 0, "没有新按下，不算 just_pressed");
        assert_eq!(count.pressed, 1, "手指仍然按着，pressed 还是 1");
    }

    #[test]
    fn counts_multiple_fingers() {
        let mut app = build_app();
        press_finger(&mut app, 1);
        press_finger(&mut app, 2);
        app.update();
        let count = app.world().resource::<TouchCount>();
        assert_eq!(count.just_pressed, 2);
        assert_eq!(count.pressed, 2);
    }
}

// 提示：
// 1. `iter()` 和 `iter_just_pressed()` 名字很像，但语义不同：一个看"现在"，一个看"本帧新发生"。
// 2. 第二帧手指还按着：`iter()` 仍然数得到它，`iter_just_pressed()` 数不到。
// 3. 修改后运行 `bevylings test 2003`，第一个测试会失败提醒你。
