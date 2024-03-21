//! This example shows the simplest way to create a Perf UI with specific entries.
//! (using defaults for everything)

use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // we want Bevy to measure these values for us:
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)

        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)

        .run();
}

fn setup(mut commands: Commands) {
    // spawn a camera to be able to see anything
    commands.spawn(Camera2dBundle::default());

    // Instead of using `PerfUiCompleteBundle`,
    // spawn an entity with `PerfUiRoot` + whatever entries you want!
    commands.spawn((
        PerfUiRoot::default(),
        // when we have lots of entries, we have to group them
        // into tuples, because of Bevy Rust syntax limitations
        (
            PerfUiEntryFPS::default(),
            PerfUiEntryFPSWorst::default(),
            PerfUiEntryFrameTime::default(),
            PerfUiEntryFrameTimeWorst::default(),
            PerfUiEntryFrameCount::default(),
            PerfUiEntryEntityCount::default(),
            PerfUiEntryCpuUsage::default(),
            PerfUiEntryMemUsage::default(),
        ),
        (
            PerfUiEntryFixedTimeStep::default(),
            PerfUiEntryFixedOverstep::default(),
            PerfUiEntryRunningTime::default(),
            PerfUiEntryClock::default(),
        ),
        (
            PerfUiEntryCursorPosition::default(),
            PerfUiEntryWindowResolution::default(),
            PerfUiEntryWindowScaleFactor::default(),
            PerfUiEntryWindowMode::default(),
            PerfUiEntryWindowPresentMode::default(),
        ),
    ));
}
