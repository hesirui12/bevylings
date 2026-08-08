Resources  Quick Start  Main Menu Quick StartIntroductionBreakoutGetting Started SetupAppsECSPluginsResourcesTracking Time with ResourcesFalling SandNext Steps3D Puzzle GameBuilding Bevy's EcosystemTroubleshooting Getting Started  Learn  News  Community  Foundation  Assets  Examples Donate    IntroductionBreakoutGetting Started SetupAppsECSPluginsResourcesTracking Time with ResourcesFalling SandNext Steps3D Puzzle GameBuilding Bevy's EcosystemTroubleshootingResourcesThe Entity and Component data types are great for representing complex, query-able groups of data. But most Apps will also require "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using the Resource trait.Here are some examples of data that could be encoded as a Resource:Elapsed TimeAsset Collections (sounds, textures, meshes)RenderersTracking Time with Resources #Let's solve our App's "hello spam" problem by only printing "hello" once every two seconds. We'll do this by using the Time resource, which is automatically added to our App via add_plugins(DefaultPlugins).For simplicity, remove the hello_world system from your App. This way we only need to adapt the greet_people system.Resources are accessed in much the same way that we access components. You can access the Time resource in your system like this:
```
fn greet_people(time: Res<Time>, query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
```
Res and ResMut pointers provide read and write access (respectively) to resources.The delta field on Time gives us the time that has passed since the last update. But in order to run our system once every two seconds, we must track the amount of time that has passed over a series of updates. To make this easier, Bevy provides the Timer type. Let's create a new Resource to track elapsed time with a Timer:
```
#[derive(Resource)]
struct GreetTimer(Timer);
```
And use it in our system:
```
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
```
Now all that's left is adding a GreetTimer Resource to our HelloPlugin. Use TimerMode::Repeating to make the timer repeat.
```
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}
```
Now cargo run the App. It should now greet people at a reasonable rate. Next Next Steps   Previous Plugins    Improve this page   Report issue               