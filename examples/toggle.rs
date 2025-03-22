//! This example shows the simplest way to create a Perf UI.
//! (using defaults for everything)

use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // we want Bevy to measure these values for us:
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        // We need to order our system before PerfUiSet::Setup,
        // so that iyes_perf_ui can process any new Perf UI in the same
        // frame as we spawn the entities. Otherwise, Bevy UI will complain.
        .add_systems(Update, toggle.before(iyes_perf_ui::PerfUiSet::Setup))
        .run();
}

fn setup(mut commands: Commands) {
    // spawn a camera to be able to see anything
    commands.spawn(Camera2d);
}

fn toggle(
    mut commands: Commands,
    q_root: Query<Entity, With<PerfUiRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        if let Ok(e) = q_root.single() {
            // despawn the existing Perf UI
            commands.entity(e).despawn();
        } else {
            // create a simple Perf UI with default settings
            // and all entries provided by the crate:
            commands.spawn(PerfUiAllEntries::default());
        }
    }
}
