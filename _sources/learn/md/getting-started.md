Getting Started  Quick Start  Main Menu Quick StartIntroductionBreakoutGetting StartedInstalling BevyIn-Engine Examples SetupAppsECSPluginsResourcesFalling SandNext Steps3D Puzzle GameBuilding Bevy's EcosystemTroubleshooting Getting Started  Learn  News  Community  Foundation  Assets  Examples Donate    IntroductionBreakoutGetting StartedInstalling BevyIn-Engine Examples SetupAppsECSPluginsResourcesFalling SandNext Steps3D Puzzle GameBuilding Bevy's EcosystemTroubleshootingGetting StartedWelcome to Bevy!This page will help you get started on your journey as quickly as possible. You can start by either immediately creating a Bevy project, or examine some examples that showcase a number of features and capabilities that Bevy provides.After this, the rest of this section will focus on helping you setup your development environment and begin writing your own Bevy applications.Installing Bevy #Bevy is built in pure Rust, which gives us the benefit of only needing a working Rust installation to get start with the engine itself. However, additional operating system specific dependencies are needed when we want to interact with different parts of our system, like creating application windows, playing audio, or reading input from peripherals. For these additional dependencies, please see the Installing OS Dependencies section on the Setup page.Bevy is available as a library on crates.io.After making a new Rust project with cargo init or cargo new my_project, the easiest way to add it to your project is to use cargo add:
```
cargo add bevy
```
Alternatively, you can manually add it to your project's Cargo.toml like this:
```
[dependencies]
bevy = "0.19"
```
Make sure to use the latest bevy crate version ().In-Engine Examples #If you would like to see Bevy in action before starting a new project, you can clone the Bevy Engine repo and run a number of examples:Clone the Bevy repo: 
```
git clone https://github.com/bevyengine/bevy
```
Navigate to the new "bevy" folder 
```
cd bevy
```
Switch to the correct Bevy version (as the default is the git main development branch) 
```
# use the latest Bevy release
git checkout latest
# or a specific version
git checkout v0.19.0
```
Try the examples in the examples folder 
```
cargo run --example breakout
```
 Next Setup   Previous Introduction    Improve this page   Report issue               