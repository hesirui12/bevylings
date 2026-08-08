# bevylings 🦀🎮

**用 Bevy 官方示例 + 官方学习指南做成的 rustlings 式练习工具。**

面向：只学过 Rust 基础、没接触过 Bevy 和游戏开发的小白。

> 所有练习代码都源自 [bevy.org/examples-webgpu](https://bevy.org/examples-webgpu/)（275 个官方示例）
> 和 [bevy.org/learn](https://bevy.org/learn/)（官方快速入门指南），每个练习**故意改错一处**，
> 你的任务就是找到并修复它。共 **35 章 151 个练习**，从"你好，Bevy！"到"自定义着色器"循序渐进。
>
> **已验证**：151 个练习的参考答案全部编译通过（343 个单元测试全绿）；
> 每个练习的 bug 均被验证——65 个编译错误型、86 个逻辑错误型（测试捕获）。

![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Bevy](https://img.shields.io/badge/Bevy-0.19-9cf.svg)
![Rust](https://img.shields.io/badge/Rust-Edition%202021-orange.svg)
![Exercises](https://img.shields.io/badge/exercises-151-green.svg)

## 为什么用 bevylings？

- **rustlings 式流程**：`run` → 看 bug → 改代码 → `test` 通过即完成，进度自动记录
- **循序渐进**：35 章从最小 App 到着色器、glTF、ECS 高级特性，覆盖 Bevy 主流 API
- **真实代码**：全部改编自 Bevy 官方示例与官方指南，不是玩具代码
- **无头测试**：练习测试全部 headless，不开窗口、不依赖外部文件，秒级反馈
- **中文讲解**：每个练习文件顶部有概念讲解、任务说明和 `// BUG:` 标注

## 快速开始

```bash
# 1. 安装 Rust（https://rustup.rs）
# 2. 克隆本项目
git clone https://github.com/hesirui12/bevylings.git && cd bevylings

# 3. 编译 CLI（第一次会编译 Bevy，需要几分钟）
cargo build -p bevylings

# 4. 查看练习总览
./target/debug/bevylings list
```

**Windows 用户**：也可以直接双击 `bevylings.bat`（首次运行自动编译 CLI），之后
`bevylings list` 即可。也可将 `bevylings.bat` 所在目录加入 PATH，全局使用 `bevylings` 命令。

## 如何使用

| 命令 | 作用 |
|---|---|
| `bevylings list` | 列出全部章节与练习、完成状态 |
| `bevylings next` | 显示下一个未完成的练习（含代码） |
| `bevylings run <id>` | 编译并运行该练习（错误版）——先看看 bug 长什么样 |
| `bevylings test <id>` | 运行该练习的单元测试——**测试全绿 = 完成** |
| `bevylings verify <id>` | 运行参考答案的测试，展示"正确行为" |
| `bevylings hint <id>` | 打印该练习的提示 |
| `bevylings status` | 查看进度 |
| `bevylings reset` | 重置进度 |

> `<id>` 取练习编号的数字部分即可，如 `0301` 或完整形式 `ex_0301` 均可。

## 练习流程（rustlings 惯例）

1. `bevylings next` 找到下一个练习，打开对应的练习文件。
2. 文件顶部有概念讲解和任务说明，代码里有 `// BUG:` 标出的一处错误。
3. `bevylings run 0301` 看现象（编译报错 / 运行异常）。
4. 修改 `exercises/src/chapters/ch03_ecs/exercise_01.rs`。
5. `bevylings test 0301` —— 测试通过后自动标记完成 ✅。

```bash
bevylings run 0301     # 编译报错或行为错误，找到 BUG
# 编辑 exercises/src/chapters/ch03_ecs/exercise_01.rs
bevylings test 0301    # ✅ 通过，标记完成
bevylings next         # 下一个！
```

遇到卡住的练习，`bevylings verify 0301` 看看参考答案的正确行为，
`bevylings hint 0301` 获取提示。实在做不出也没关系，直接对照
`solutions/src/chapters/ch03_ecs/exercise_01.rs` 的答案。

## 目录结构

```
bevylings/
├── bevylings/          # CLI 运行器（list/run/test/verify/hint/next/status）
├── exercises/          # 练习题库（故意改错的代码，Bevy 0.19）
│   └── src/chapters/   # 35 个章节，每章 3~7 个练习
├── solutions/          # 参考答案（正确代码）
├── _sources/           # 官方源码快照（bevyengine/bevy）+ learn 页面
├── _specs/             # 练习作者规范（AUTHORING.md）
├── _tools/             # 内容生成与校验脚本（gen.py / verify_*.py 等）
├── _tasks/             # 任务清单
├── exercises.manifest.json  # 章节/练习清单
└── bevylings.bat       # Windows 启动器
```

## 章节总览（35 章 / 151 练习）

| 章节 | 内容 |
|---|---|
| 01 你好，Bevy！ | App、插件、系统、日志、退出 |
| 02 App 与应用 | 无窗口运行、日志级别、插件组、线程池 |
| 03 ECS 实体组件系统 | Entity、Component、Query、Bundle、Commands |
| 04 插件 | Plugin trait、插件组、配置 |
| 05 资源 | Res/ResMut、init/insert/remove |
| 06 打砖块小游戏 | 挡板、球、碰撞、计分 |
| 07 2D 基础 | Sprite、形状、移动、旋转、缩放 |
| 08 3D 基础 | 相机、立方体、灯光、父子、透明 |
| 09 动画 | 缓动、AnimatedTransform、颜色动画 |
| 10 音频 | AudioPlayer、音量、音效、空间音频 |
| 11 摄像机 | 跟随、环绕、缩放、控制器 |
| 12 诊断与日志 | FPS、自定义诊断、日志级别 |
| 13 小游戏合集 | 弹跳球、菜单、加载屏、吃蛋糕 |
| 14 调试绘制 | Gizmos 线/圆/矩形/轴 |
| 15 数学 | 向量、样条、边界、原语 |
| 16 场景 | BSN、SceneRoot、世界序列化 |
| 17 状态管理 | States、OnEnter/OnExit、子状态 |
| 18 时间与计时器 | Time、Timer、Stopwatch、虚拟时间 |
| 19 变换 | Transform、旋转、缩放、朝向 |
| 20 输入 | 键盘、鼠标、触摸、手柄 |
| 21 UI 用户界面 | 文本、按钮、布局、边框 |
| 22 窗口 | 标题、分辨率、多窗口、清屏色 |
| 23 移动 | 平滑跟随、固定步长、转向 |
| 24 异步任务 | AsyncComputeTaskPool、通道 |
| 25 着色器入门 | 自定义材质、WGSL、2D 着色器 |
| 26 glTF 模型 | 加载、SceneRoot、遍历、材质 |
| 27 拾取点击 | 悬停、点击、拖拽 |
| 28 资产管理 | AssetServer、热重载、嵌入资产 |
| 29 事件与观察者 | Message、MessageReader、Observer |
| 30 运行条件 | run_if、条件组合、系统管道 |
| 31 固定时间步 | FixedUpdate、Time\<Fixed\> |
| 32 变更检测 | Changed/Added、RemovedComponents |
| 33 层级与父子关系 | Parent/Children、add_child |
| 34 高级 ECS | 一次性系统、泛型系统、并行查询 |
| 35 实用技巧 | 冷却、日志分层、无 winit |

## 技术说明

- 引擎版本：**Bevy 0.19**（默认 feature，无需额外配置）
- 每个练习是独立 feature 门控的模块：`cargo test -p exercises --features ex_0301`
- 练习测试均为无头（headless）模式，不开窗口、不依赖外部文件
- 学习路径参考了官方示例分类（2D/3D/UI/音频/相机...）与快速入门指南（ECS/插件/资源...）

## 磁盘空间管理

- `run / test / verify` 每次检验后会自动**删除本次新增的可执行文件**（每个 Bevy debug 二进制 40~60MB），
  同时保留 rlib/rmeta 编译缓存，下次做题仍是增量编译，速度几乎不受影响。
- 想彻底清空编译产物释放空间：`cargo clean`（下次检验会全量重编 Bevy，需几分钟）。

## 常见问题

**Q：第一次做题编译要等很久？**
A：Bevy 依赖较大，冷编译需几分钟，属正常现象。之后的增量编译很快。

**Q：`bevylings run` 打开了一个游戏窗口？**
A：部分练习（如小游戏）确实会开窗口运行。若在无图形环境，可改用 `bevylings test` 验证。

**Q：做题进度存在哪？**
A：`.bevylings/state.json`（已被 gitignore，不影响仓库）。`bevylings reset` 可重置。

**Q：为什么编译产物总在增长？**
A：见上文「磁盘空间管理」。每次检验自动清理本次产生的可执行文件。

**Q：想自己加练习题？**
A：见 `_specs/AUTHORING.md` 作者规范，用 `_tools/gen_*.py` 生成，`_tools/verify_*.py` 校验。

## 贡献

欢迎提交练习修正、新章节、翻译改进或工具优化：

1. Fork 本仓库并创建新分支
2. 修改练习代码时请保证：错误版能被测试捕获、参考答案测试全绿
3. 跑一遍 `_tools/verify_exercises.py` 与 `_tools/verify_solutions.py` 验证
4. 提交 PR，说明改动与验证结果

## License

MIT License，详见 [LICENSE](LICENSE)。

- 所有练习代码改编自 Bevy 官方仓库（MIT 协议）：<https://github.com/bevyengine/bevy/tree/latest/examples>
- 文档与教程来自 <https://bevy.org/learn/>
