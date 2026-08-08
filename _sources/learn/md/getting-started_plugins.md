Plugins  Quick Start  Main Menu Quick StartIntroductionBreakoutGetting Started SetupAppsECSPluginsBevy's Default PluginsCreating Your First PluginResourcesFalling SandNext Steps3D Puzzle GameBuilding Bevy's EcosystemTroubleshooting Getting Started  Learn  News  Community  Foundation  Assets  Examples Donate    IntroductionBreakoutGetting Started SetupAppsECSPluginsBevy's Default PluginsCreating Your First PluginResourcesFalling SandNext Steps3D Puzzle GameBuilding Bevy's EcosystemTroubleshootingPluginsOne of Bevy's core principles is modularity. All Bevy engine features are implemented as plugins---collections of code that modify an App. This includes internal features like the renderer, but games themselves are also implemented as plugins! This empowers developers to pick and choose which features they want. Don't need a UI? Don't register the UiPlugin. Want to build a headless server? Don't register the RenderPlugin.This also means you are free to replace any components you don't like. If you feel the need, you are welcome to build your own UiPlugin, but consider contributing it back to Bevy if you think it would be useful!Those not contributed back into Bevy and instead released separately are third-party plugins. These are useful and easy to use additions created by fellow developers that can help you avoid re-inventing the wheel. To use them all you have to do is:Find a third party Bevy plugin (like those at the Assets page).Add it to your Cargo.toml as a crate under [dependencies].Import the code definitions (like use third_party::prelude::*;) from the crate to add the items to your workspace.Add the plugin to your app (like app.add_plugins(third_party_plugin)).However, most developers don't need a custom experience and just want the "full engine" experience with no hassle. For this, Bevy provides a set of DefaultPlugins.Bevy's Default Plugins #Let's make our app more interesting by adding Bevy's DefaultPlugins which are a PluginGroup containing core engine features. (For those needing minimal features, MinimalPlugins exists). add_plugins(DefaultPlugins) adds the features most people expect from an engine, such as a 2D / 3D renderer, asset loading, a UI system, windows, and input.
```
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
```
Once again run cargo run.You should hopefully notice two things:A window should pop up. This is because we now have WindowPlugin, which defines the window interface (but doesn't actually know how to make windows), and WinitPlugin which uses the winit library to create a window using your OS's native window API.Your console is now full of "hello" messages. This is because DefaultPlugins adds an "event loop" to our application. Our App's ECS Schedule now runs in a loop once per "frame". We will resolve the console spam in a moment.Creating Your First Plugin #For better organization, let's move all of our "hello" logic to a plugin. To create a plugin we just need to implement the Plugin interface. Add the following code to your main.rs file:
```
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
    }
}
```
Then register the plugin in your App like this:
```
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
```
Note add_plugins can add any number of plugins (or plugin groups like DefaultPlugins) by passing in a tuple of them. Now all that's left is to move our systems into HelloPlugin, which is just a matter of cut and paste. The app variable in our plugin's build() function is the same builder type we use in our main() function:
```
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
```
Try running the app again. It should do exactly what it did before. In the next section, we'll fix the "hello" spam using Resources. Next Resources   Previous ECS    Improve this page   Report issue               