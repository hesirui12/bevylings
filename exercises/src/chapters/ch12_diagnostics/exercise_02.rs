//! # 练习 12.02 —— 自定义诊断：记录你自己的指标
//!
//! 出处：https://bevy.org/examples-webgpu/diagnostics/custom_diagnostic/
//!
//! ## 概念
//! 除了内置诊断，我们也可以自己定义指标：
//! 先用 `DiagnosticPath::const_new("唯一路径")` 起一个唯一的名字，
//! 再用 `app.register_diagnostic(Diagnostic::new(路径).with_suffix("单位"))` 注册，
//! 之后在系统里用 `Diagnostics::add_measurement(&路径, || 数值)` 每帧记录一个值。
//! 所有记录都存进 `DiagnosticsStore`（按路径查询），配合 `LogDiagnosticsPlugin`
//! 就能在控制台里看到自己的指标了。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1202` 观察现象，改正后运行 `bevylings test 1202` 让测试通过。
//!
//! 小贴士：`add_measurement` 的第二个参数是闭包（`|| 数值`），
//! 只有当该诊断被启用时才会真正计算，所以“数值的计算”可以很贵。

// I AM NOT DONE

use bevy::{
    diagnostic::{
        Diagnostic, DiagnosticPath, Diagnostics, DiagnosticsStore, LogDiagnosticsPlugin,
        RegisterDiagnostic,
    },
    prelude::*,
};

/// 这个诊断的唯一标识：表示“系统迭代次数”。
const SYSTEM_ITERATION_COUNT: DiagnosticPath =
    DiagnosticPath::const_new("system_iteration_count");

/// 每次系统运行完成的工作单位数。
const WORK_UNITS_PER_RUN: f64 = 10.0;

/// 每帧给我们的自定义诊断记一个测量值。
fn my_system(mut diagnostics: Diagnostics) {
    diagnostics.add_measurement(&SYSTEM_ITERATION_COUNT, || {
        // BUG: 常量的本意是“每次运行 10 个工作单位”，这里却把数值减半了，
        // 导致记录进诊断的值不对。
        WORK_UNITS_PER_RUN / 2.0
    });
}

pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
        ))
        .register_diagnostic(Diagnostic::new(SYSTEM_ITERATION_COUNT).with_suffix(" iterations"))
        .add_systems(Update, my_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measurement_is_recorded_per_frame() {
        let mut app = App::new();
        app.register_diagnostic(Diagnostic::new(SYSTEM_ITERATION_COUNT).with_suffix(" iterations"));
        app.add_systems(Update, my_system);
        app.update();
        app.update();
        let store = app.world().resource::<DiagnosticsStore>();
        let diagnostic = store
            .get(&SYSTEM_ITERATION_COUNT)
            .expect("应该能找到注册过的诊断");
        assert_eq!(diagnostic.value(), Some(10.0), "每帧应该记录 10.0");
        assert_eq!(diagnostic.values().count(), 2, "跑了两帧应该累积两个测量值");
    }

    #[test]
    fn path_is_unique_and_stable() {
        assert_eq!(SYSTEM_ITERATION_COUNT.as_str(), "system_iteration_count");
        let other = DiagnosticPath::const_new("system_iteration_count");
        assert_eq!(SYSTEM_ITERATION_COUNT, other, "相同路径应该判等");
    }
}

// 提示：
// 1. 运行 `bevylings run 1202`，观察控制台里打印的迭代次数是不是 10。
// 2. `add_measurement` 记录的是闭包算出的值，对照常量想想应该是多少。
// 3. 改好后运行 `bevylings test 1202`，测试全绿就过关了。
