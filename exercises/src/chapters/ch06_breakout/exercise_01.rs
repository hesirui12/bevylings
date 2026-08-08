//! # 练习 06.01 —— 挡板：用键盘控制左右移动
//!
//! 出处：https://bevy.org/examples/games/breakout/
//!
//! ## 概念
//! 打砖块里玩家操控的是**挡板**（paddle），一块只能左右移动的横条。
//! 它本身只是一个空的组件标签，真正的数据（位置）存在 `Transform` 里。
//! 每个游戏循环（帧）我们读一次键盘：按左/右箭头就给一个"方向"，
//! 然后按 `新位置 = 旧位置 + 方向 × 速度 × 帧时间` 更新挡板。
//! 帧时间 `time.delta_secs()` 是这一帧经过的秒数，用它乘速度，
//! 游戏就不会因为电脑快慢不同而移动快慢不同。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0601` 查看现象（目前会编译报错），
//! 改正后运行 `bevylings test 0601` 让测试通过。
//!
//! 小贴士：`clamp(min, max)` 能把数值限制在 [min, max] 区间内。

// I AM NOT DONE

use bevy::prelude::*;

/// 挡板移动速度：每秒 500 像素。
const PADDLE_SPEED: f32 = 500.0;
/// 挡板左右能到达的最远位置。
const LEFT_BOUND: f32 = -410.0;
const RIGHT_BOUND: f32 = 410.0;

/// 挡板：一个空的组件，用来标记"这个实体是玩家控制的挡板"。
#[derive(Component)]
struct Paddle;

/// 计算挡板新的 x 坐标：当前位置 + 方向 × 速度 × 帧时间，并限制在边界内。
fn new_paddle_x(current: f32, direction: f32, delta_secs: f32) -> f32 {
    let next = current + direction * PADDLE_SPEED * delta_secs;
    // BUG: 最后一行末尾多了一个分号。多了这个分号，
    // clamp 的结果就变成了"语句"，函数体以语句结尾会返回 `()`，
    // 和声明的返回值 `f32` 对不上。
    next.clamp(LEFT_BOUND, RIGHT_BOUND);
}

/// 读键盘，移动挡板。
fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }
    paddle.translation.x = new_paddle_x(paddle.translation.x, direction, time.delta_secs());
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_paddle)
        .run();
}

/// 生成相机和挡板。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Paddle,
        Sprite::from_color(Color::srgb(0.3, 0.3, 0.7), Vec2::new(120.0, 20.0)),
        Transform::from_xyz(0.0, -240.0, 0.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_right_when_direction_positive() {
        assert_eq!(new_paddle_x(0.0, 1.0, 0.5), 250.0);
        assert_eq!(new_paddle_x(100.0, 1.0, 0.5), 350.0);
    }

    #[test]
    fn moves_left_when_direction_negative() {
        assert_eq!(new_paddle_x(0.0, -1.0, 0.5), -250.0);
    }

    #[test]
    fn clamps_within_bounds() {
        assert_eq!(new_paddle_x(400.0, 1.0, 1.0), 410.0, "贴右边界后不能再往右");
        assert_eq!(new_paddle_x(-400.0, -1.0, 1.0), -410.0, "贴左边界后不能再往左");
    }
}

// 提示：
// 1. 先运行 `bevylings run 0601`，读一读编译错误提示。
// 2. 函数的返回值是最后一个表达式的值；如果最后一行是"语句"，返回的就是 `()`。
// 3. 对比"表达式"和"语句"的区别（结尾有没有分号）。
