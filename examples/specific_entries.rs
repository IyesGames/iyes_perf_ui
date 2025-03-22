//! This example shows the simplest way to create a Perf UI with specific entries.
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

        .run();
}

fn setup(mut commands: Commands) {
    // spawn a camera to be able to see anything
    commands.spawn(Camera2d);

    // Customize your Perf UI by explicitly listing the entries you want.
    commands.spawn((
        // when we have lots of entries, we have to group them
        // into tuples, because of Bevy Rust syntax limitations
        (
            PerfUiWidgetBar::new(PerfUiEntryFPS::default()),
            PerfUiWidgetBar::new(PerfUiEntryFPSWorst::default()),
            PerfUiWidgetBar::new(PerfUiEntryFrameTime::default()),
            PerfUiWidgetBar::new(PerfUiEntryFrameTimeWorst::default()),
            PerfUiWidgetBar::new(PerfUiEntryRenderCpuTime::default()),
            PerfUiWidgetBar::new(PerfUiEntryRenderGpuTime::default()),
            PerfUiWidgetBar::new(PerfUiEntryEntityCount::default()),
            PerfUiWidgetBar::new(PerfUiEntryCpuUsage::default()),
            PerfUiWidgetBar::new(PerfUiEntryMemUsage::default()),
            PerfUiEntryFrameCount::default(),
        ),
        (
            PerfUiEntryFixedTimeStep::default(),
            PerfUiWidgetBar::new(PerfUiEntryFixedOverstep::default()),
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
