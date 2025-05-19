//! This example shows how to customize the appearance of your Perf UIs.

use bevy::prelude::*;
use iyes_perf_ui::prelude::*;
use iyes_perf_ui::widgets::bar::{BarFillDirection, BarTextPosition};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // we want Bevy to measure these values for us:
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)

        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)

        .run();
}

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    // spawn a camera to be able to see anything
    commands.spawn(Camera2d);

    // Let's create multiple Perf UIs!

    // Put common values in a variable so we don't repeat ourselves
    let root_config = PerfUiRoot {
        background_color: Color::srgba(0.9, 0.95, 0.99, 0.75),
        inner_background_color: Color::srgba(0.05, 0.1, 0.15, 0.25),
        inner_background_color_highlight: Color::srgba(1.0, 0.9, 0.25, 0.75),
        text_err: "Unavailable!".into(),
        err_color: Color::srgba(0.15, 0.07, 0.03, 0.25),
        default_value_color: Color::srgb(0.4, 0.4, 0.4),
        label_color: Color::srgb(0.3, 0.4, 0.5),
        fontsize_label: 20.0,
        fontsize_value: 24.0,
        margin: 4.0,
        padding: 2.0,
        inner_margin: 2.0,
        inner_padding: 4.0,
        values_col_width: 128.0,
        font_label: ass.load("Ubuntu-B.ttf"),
        font_value: ass.load("Ubuntu-R.ttf"),
        font_highlight: ass.load("Ubuntu-RI.ttf"),
        ..default()
    };

    // Perf UI #1: Framerate/Frametime
    commands.spawn((
        PerfUiRoot {
            position: PerfUiPosition::TopLeft,
            z_index: GlobalZIndex(i32::MAX),
            ..root_config.clone()
        },
        PerfUiEntryFPS {
            // provide a custom label string
            label: "Frame Rate (current)".into(),
            // let's say we *really* care about high framerates‼
            color_gradient: ColorGradient::new()
                .with_stop(90.0, Color::srgb(0.8, 0.0, 0.2))
                .with_stop(240.0, Color::srgb(0.2, 0.9, 0.4)),
            threshold_highlight: Some(60.0),
            digits: 5,
            precision: 2,
            ..default()
        },
        PerfUiEntryFPSWorst {
            label: "Frame Rate (worst)".into(),
            color_gradient: ColorGradient::new()
                .with_stop(90.0, Color::srgb(0.8, 0.0, 0.2))
                .with_stop(240.0, Color::srgb(0.2, 0.9, 0.4)),
            threshold_highlight: Some(60.0),
            digits: 5,
            precision: 2,
            ..default()
        },
        PerfUiEntryFrameTime {
            label: "Frame Duration (current)".into(),
            color_gradient: ColorGradient::new().with_stops([
                (1.0, Color::srgb(0.15, 0.8, 0.9)),
                (8.0, Color::srgb(0.7, 0.15, 0.9))
            ]),
            threshold_highlight: Some(10.0),
            digits: 2,
            precision: 4,
            ..default()
        },
        PerfUiEntryFrameTimeWorst {
            label: "Frame Duration (worst)".into(),
            color_gradient: ColorGradient::new().with_stops([
                (1.0, Color::srgb(0.15, 0.8, 0.9)),
                (8.0, Color::srgb(0.7, 0.15, 0.9))
            ]),
            threshold_highlight: Some(10.0),
            digits: 2,
            precision: 4,
            ..default()
        },
    ));

    // Perf UI #2: ECS stats + System CPU/RAM usage
    // (displayed using fancy Bar widgets)
    commands.spawn((
        PerfUiRoot {
            position: PerfUiPosition::BottomLeft,
            // always display this Perf UI below the other one
            z_index: GlobalZIndex(i32::MAX - 1),
            ..root_config.clone()
        },
        PerfUiWidgetBar {
            fill_direction: BarFillDirection::Center,
            bar_background: Color::srgba(0.0, 0.0, 0.0, 0.5),
            // The color gradient also affects the range of values for the bar
            bar_color: ColorGradient::new().with_stops([
                (0.0, Color::BLACK),
                (200.0, Color::WHITE),
            ]),
            bar_border_color: Color::WHITE,
            bar_border_px: 2.0,
            ..PerfUiWidgetBar::new(PerfUiEntryEntityCount {
                label: "Number of ECS Entities".into(),
                threshold_highlight: None,
                color_gradient: ColorGradient::single(Color::BLACK),
                digits: 4,
                ..default()
            })
        },
        PerfUiWidgetBar {
            text_position: BarTextPosition::OutsideEnd,
            bar_background: Color::srgba(0.0, 0.0, 0.0, 0.5),
            bar_color: ColorGradient::new().with_stops([
                (0.0, Color::srgb(0.0, 0.0, 1.0)),
                (100.0, Color::srgb(1.0, 0.0, 0.0)),
            ]),
            bar_border_color: Color::WHITE,
            bar_border_px: 2.0,
            ..PerfUiWidgetBar::new(PerfUiEntryCpuUsage {
                label: "System CPU Utilization".into(),
                color_gradient: ColorGradient::new().with_stops([
                    (0.0, Color::srgb(0.0, 0.0, 1.0)),
                    (100.0, Color::srgb(1.0, 0.0, 0.0)),
                ]),
                threshold_highlight: None,
                precision: 1,
                ..default()
            })
        },
        PerfUiWidgetBar {
            text_position: BarTextPosition::OutsideEnd,
            bar_background: Color::srgba(0.0, 0.0, 0.0, 0.5),
            bar_color: ColorGradient::new().with_stops([
                (0.0, Color::srgb(0.0, 0.0, 1.0)),
                (100.0, Color::srgb(1.0, 0.0, 0.0)),
            ]),
            bar_border_color: Color::WHITE,
            bar_border_px: 2.0,
            ..PerfUiWidgetBar::new(PerfUiEntryMemUsage {
                label: "System RAM Utilization".into(),
                color_gradient: ColorGradient::new().with_stops([
                    (0.0, Color::srgb(0.0, 0.0, 1.0)),
                    (100.0, Color::srgb(1.0, 0.0, 0.0)),
                ]),
                threshold_highlight: None,
                precision: 1,
                ..default()
            })
        },
    ));

    // Perf UI #3: Clock + running time
    commands.spawn((
        PerfUiRoot {
            // let's not have labels for this one
            display_labels: false,
            position: PerfUiPosition::BottomRight,
            // always display this Perf UI below the other two
            z_index: GlobalZIndex(i32::MAX - 2),
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

    // Perf UI #4: Cursor Position, Window Properties
    commands.spawn((
        PerfUiRoot {
            position: PerfUiPosition::TopRight,
            // always display this Perf UI below the other three
            z_index: GlobalZIndex(i32::MAX - 3),
            ..root_config.clone()
        },
        PerfUiEntryCursorPosition {
            label: "Mouse".into(),
            separator: "\n",
            width: 0, // no padding with spaces
            display_units: true,
            display_axis: false,
            physical_pixels: true,
            ..default()
        },
        PerfUiEntryWindowResolution {
            label: "Window Size".into(),
            separator: "\n",
            width: 0, // no padding with spaces
            display_units: true,
            display_axis: false,
            physical_pixels: true,
            ..default()
        },
        PerfUiEntryWindowScaleFactor {
            label: "Window Scaling Factor".into(),
            ..default()
        },
        PerfUiEntryWindowMode {
            label: "Mode".into(),
            ..default()
        },
        PerfUiEntryWindowPresentMode {
            label: "VSync".into(),
            ..default()
        },
    ));
}
