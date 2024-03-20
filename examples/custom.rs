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
use bevy::utils::Duration;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // we want Bevy to measure these values for us:
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)

        .add_plugins(PerfUiPlugin)

        // we must register our custom entry type
        .add_perf_ui_entry_type::<PerfUiTimeSinceLastClick>()

        .init_resource::<TimeSinceLastClick>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_click)

        .run();
}

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    // spawn a camera to be able to see anything
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        PerfUiRoot {
            // Let's provide some custom fonts, so we can also
            // see the font changing when an entry is highlighted
            font_label: ass.load("Ubuntu-B.ttf"),
            font_value: ass.load("Ubuntu-R.ttf"),
            font_highlight: ass.load("Ubuntu-RI.ttf"),
            // just so things don't move around (Ubuntu font is not fixed width)
            values_col_width: Some(64.0),
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
pub struct PerfUiTimeSinceLastClick {
    /// Highlight the value if it goes above this threshold
    pub threshold_highlight: f32,
    /// Display with custom color below this value (default color otherwise)
    pub threshold_low: f32,
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
            threshold_highlight: 10.0,
            threshold_low: 1.0,
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
        "Time since last click"
    }

    // We must return the sort key we stored when constructing the struct
    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    // Called every frame to compute a new value to show
    fn update_value<'w>(
        &mut self,
        (time, lastclick): &mut <Self::SystemParam as SystemParam>::Item<'w, '_>,
    ) -> Option<Self::Value> {
        let d = time.elapsed() - lastclick.last_click;
        Some(d.as_secs_f64())
    }

    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        // we can use a premade helper function for nice-looking formatting
        iyes_perf_ui::utils::format_pretty_float(self.digits, self.precision, *value)
    }

    // (optional) Called every frame to determine if a custom color should be used for the value
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        if *value < self.threshold_low as f64 {
            Some(Color::RED)
        } else {
            // will use the default color
            None
        }
    }

    // (optional) Called every frame to determine if the value should be highlighted
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        *value > self.threshold_highlight as f64
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
