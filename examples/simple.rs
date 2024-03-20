//! This example shows the simplest way to create a Perf UI.
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

    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFrameTime::default(),
        PerfUiEntryFrameTimeWorst::default(),
        PerfUiEntryFrameCount::default(),
        PerfUiEntryEntityCount::default(),
        PerfUiEntryCpuUsage::default(),
        PerfUiEntryMemUsage::default(),
        PerfUiEntryFixedTimeStep::default(),
        PerfUiEntryFixedOverstep::default(),
        PerfUiEntryRunningTime::default(),
        PerfUiEntryClock::default(),
    ));
}
