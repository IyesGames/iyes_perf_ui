//! This example shows how to create a new custom entry for your Perf UI.
//!
//! We will keep track of the time when the mouse was last clicked, by
//! storing it in an ECS resource, and implement a Perf UI entry to display it.
//!
//! This example is the "full" complex version. It shows how to add support
//! for all the fancy features of the crate, so that you can make your custom
//! Perf UI entry look nice, just like the ones provided by the library. :)
//!
//! If you just want to see how to create a custom entry with minimal boilerplate,
//! see the `custom_minimal` example instead.

use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use iyes_perf_ui::prelude::*;
use iyes_perf_ui::entry::PerfUiEntry;

use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // we want Bevy to measure these values for us:
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())

        .add_plugins(PerfUiPlugin)

        // we must register our custom entry type
        .add_perf_ui_simple_entry::<PerfUiTimeSinceLastClick>()

        .init_resource::<TimeSinceLastClick>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_click)

        .run();
}

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    // spawn a camera to be able to see anything
    commands.spawn(Camera2d);

    commands.spawn((
        PerfUiRoot {
            // Let's provide some custom fonts, so we can also
            // see the font changing when an entry is highlighted
            font_label: ass.load("Ubuntu-B.ttf"),
            font_value: ass.load("Ubuntu-R.ttf"),
            font_highlight: ass.load("Ubuntu-RI.ttf"),
            ..default()
        },
        PerfUiEntryFPS::default(),
        PerfUiTimeSinceLastClick::default(),
    ));
}

/// Global resource to store the time when the mouse was last clicked
#[derive(Resource, Default)]
pub struct TimeSinceLastClick {
    last_click: Duration,
}

/// Custom Perf UI entry to show the time since the last mouse click
#[derive(Component)]
#[require(PerfUiRoot)]
pub struct PerfUiTimeSinceLastClick {
    /// The label text to display, to allow customization
    pub label: String,
    /// Should we display units?
    pub display_units: bool,
    /// Highlight the value if it goes above this threshold
    pub threshold_highlight: Option<f32>,
    /// Support color gradients!
    pub color_gradient: ColorGradient,
    /// Width for formatting the string
    pub digits: u8,
    /// Precision for formatting the string
    pub precision: u8,

    /// Required to ensure the entry appears in the correct place in the Perf UI
    pub sort_key: i32,
}

impl Default for PerfUiTimeSinceLastClick {
    fn default() -> Self {
        PerfUiTimeSinceLastClick {
            label: String::new(),
            display_units: true,
            threshold_highlight: Some(10.0),
            color_gradient: ColorGradient::new_preset_gyr(1.0, 4.0, 8.0).unwrap(),
            digits: 2,
            precision: 3,
            // get the correct value from the library
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

// Implement the trait for integration into the Perf UI
impl PerfUiEntry for PerfUiTimeSinceLastClick {
    type Value = f64;
    // Any system parameters we need in order to compute our value
    type SystemParam = (SRes<Time>, SRes<TimeSinceLastClick>);

    // The text that will be shown as the Perf UI label
    fn label(&self) -> &str {
        // return our stored value, if customized, or the default
        if self.label.is_empty() {
            "Time since last click"
        } else {
            &self.label
        }
    }

    // We must return the sort key we stored when constructing the struct
    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    // Called every frame to compute a new value to show
    fn update_value(
        &self,
        (time, lastclick): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let d = time.elapsed() - lastclick.last_click;
        Some(d.as_secs_f64())
    }

    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        // we can use a premade helper function for nice-looking formatting
        let mut s = iyes_perf_ui::utils::format_pretty_float(self.digits, self.precision, *value);
        // (and append units to it)
        if self.display_units {
            s.push_str(" s");
        }
        s
    }

    // (optional) Called every frame to determine if a custom color should be used for the value
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }

    // (optional) Called every frame to determine if the value should be highlighted
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
}

fn handle_click(
    time: Res<Time>,
    mut lastclick: ResMut<TimeSinceLastClick>,
    mut evr_mouse: EventReader<MouseButtonInput>,
) {
    for ev in evr_mouse.read() {
        if ev.state == ButtonState::Pressed {
            lastclick.last_click = time.elapsed();
        }
    }
}
