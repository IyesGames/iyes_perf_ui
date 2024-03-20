//! This example shows how to customize the appearance of your Perf UIs.

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

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    // spawn a camera to be able to see anything
    commands.spawn(Camera2dBundle::default());

    // Let's create multiple Perf UIs!

    // Put common values in a variable so we don't repeat ourselves
    let root_config = PerfUiRoot {
        background_color: Color::WHITE.with_a(0.75),
        inner_background_color: Color::BLACK.with_a(0.25),
        inner_background_color_highlight: Color::YELLOW.with_a(0.75),
        text_err: "Unavailable!".into(),
        err_color: Color::BLACK.with_a(0.25),
        default_value_color: Color::DARK_GRAY,
        label_color: Color::GRAY,
        fontsize_label: 20.0,
        fontsize_value: 24.0,
        margin: 4.0,
        padding: 2.0,
        inner_margin: 2.0,
        inner_padding: 4.0,
        values_col_width: Some(128.0),
        font_label: ass.load("Ubuntu-B.ttf"),
        font_value: ass.load("Ubuntu-R.ttf"),
        font_highlight: ass.load("Ubuntu-RI.ttf"),
        ..default()
    };

    // Perf UI #1: Framerate/Frametime
    commands.spawn((
        PerfUiRoot {
            position: PerfUiPosition::TopLeft,
            z_index: ZIndex::Global(i32::MAX),
            ..root_config.clone()
        },
        PerfUiEntryFPS {
            // let's say we *really* care about high framerates‼
            threshold_bad: 90.0,
            threshold_normal: 144.0,
            threshold_good: 240.0,
            digits: 5,
            precision: 2,
            ..default()
        },
        PerfUiEntryFPSWorst {
            threshold_bad: 90.0,
            threshold_normal: 144.0,
            threshold_good: 240.0,
            digits: 5,
            precision: 2,
            ..default()
        },
        PerfUiEntryFrameTime {
            threshold_bad: 5.0,
            threshold_normal: 2.0,
            threshold_good: 1.0,
            digits: 2,
            precision: 4,
            ..default()
        },
        PerfUiEntryFrameTimeWorst {
            threshold_bad: 5.0,
            threshold_normal: 2.0,
            threshold_good: 1.0,
            digits: 2,
            precision: 4,
            ..default()
        },
    ));

    // Perf UI #2: ECS stats + System CPU/RAM usage
    commands.spawn((
        PerfUiRoot {
            position: PerfUiPosition::BottomLeft,
            // always display this Perf UI below the other one
            z_index: ZIndex::Global(i32::MAX - 1),
            ..root_config.clone()
        },
        PerfUiEntryEntityCount {
            // disable color and highlighting for this one
            enable_color: false,
            enable_highlight: false,
            digits: 4,
            ..default()
        },
        PerfUiEntryCpuUsage {
            // and we want to keep the cpu usage low
            threshold_high: 50.0,
            threshold_normal: 20.0,
            threshold_low: 5.0,
            precision: 1,
            ..default()
        },
        PerfUiEntryMemUsage {
            threshold_high: 25.0,
            threshold_normal: 15.0,
            threshold_low: 10.0,
            precision: 2,
            ..default()
        },
    ));
    // Perf UI #3: Clock + running time
    commands.spawn((
        PerfUiRoot {
            // let's not have labels for this one
            display_labels: false,
            position: PerfUiPosition::BottomRight,
            // always display this Perf UI below the other two
            z_index: ZIndex::Global(i32::MAX - 2),
            ..root_config.clone()
        },
        PerfUiEntryRunningTime {
            format_hms: true,
            precision: 3,
            sort_key: 1, // we can manually control the order of the entries
            ..default()
        },
        PerfUiEntryClock {
            // always show time in UTC
            prefer_utc: true,
            precision: 1,
            sort_key: 0, // we can manually control the order of the entries
            ..default()
        },
    ));
}
