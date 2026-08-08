//! # 练习 12.01 —— 内置诊断：帧时间与 FPS
//!
//! 出处：https://bevy.org/examples-webgpu/diagnostics/log_diagnostics/
//!
//! ## 概念
//! Bevy 内置了“诊断”（diagnostics）：一类可以定期记录的运行指标。
//! `FrameTimeDiagnosticsPlugin` 负责记录每帧耗时、FPS 等数据，
//! `LogDiagnosticsPlugin` 再把它们定期打印到控制台。
//! 本练习先不深入插件内部，重点理解“帧时间 → FPS”的换算：
//! 一秒钟内能跑几帧，就是 `1.0 / 每帧耗时(秒)`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1201` 观察现象，改正后运行 `bevylings test 1201` 让测试通过。
//!
//! 小贴士：Rust 里整数除以浮点数会报类型错误；算小数请用带小数点的字面量。

// I AM NOT DONE

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

/// 根据每帧耗时（秒）估算 FPS。
fn frames_per_second(frame_time_secs: f32) -> f32 {
    // BUG: 想算“1 秒 ÷ 帧耗时”，但被除数写成了整数，
    // 整数不能除以浮点数，类型对不上，编译不过。
    1_i32 / frame_time_secs
}

/// 启动时打印一个示例换算结果，帮助理解（真实诊断由插件输出）。
fn demo() {
    info!("每帧约 0.0167 秒 ≈ {:.0} FPS", frames_per_second(0.0167));
}

pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, demo)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fps_from_frame_time() {
        assert!((frames_per_second(0.5) - 2.0).abs() < 1e-6, "每帧 0.5 秒 → 2 FPS");
        assert!((frames_per_second(0.1) - 10.0).abs() < 1e-6, "每帧 0.1 秒 → 10 FPS");
    }

    #[test]
    fn fps_is_positive_and_finite() {
        let fps = frames_per_second(0.016);
        assert!(fps.is_finite(), "结果应该是有限的正数");
        assert!(fps > 0.0);
    }
}

// 提示：
// 1. 运行 `bevylings run 1201`，看看编译器说“can't divide `i32` by `f32`”在哪一行。
// 2. 被除数应该写成浮点字面量 `1.0`，注意别忘记小数点。
// 3. 改好后运行 `bevylings test 1201`，测试全绿就过关了。
