//! This example shows how to create a new custom entry for your Perf UI.
//!
//! We will keep track of the time when the mouse was last clicked, by
//! storing it in an ECS resource, and implement a Perf UI entry to display it.
//!
//! This example is the "minimal" version. It shows how to create a simple
//! integration with minimal boilerplate.
//!
//! If you want to see how to add support for all the fancy formatting features
//! of the library, to make things look pretty, see the `custom` example instead.

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
        .add_plugins(PerfUiPlugin)

        // we must register our custom entry type
        .add_perf_ui_simple_entry::<PerfUiTimeSinceLastClick>()

        .init_resource::<TimeSinceLastClick>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_click)

        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiTimeSinceLastClick,
    ));
}

/// Global resource to store the time when the mouse was last clicked
#[derive(Resource, Default)]
pub struct TimeSinceLastClick {
    last_click: Duration,
}

/// Custom Perf UI entry to show the time since the last mouse click
#[derive(Component, Default)]
pub struct PerfUiTimeSinceLastClick;

// Implement the trait for integration into the Perf UI
impl PerfUiEntry for PerfUiTimeSinceLastClick {
    type Value = u64;
    // Any system parameters we need in order to compute our value
    type SystemParam = (SRes<Time>, SRes<TimeSinceLastClick>);

    // The text that will be shown as the Perf UI label
    fn label(&self) -> &str {
        "Time since last click"
    }

    // We must return a sort key, to determine where to place the entry
    fn sort_key(&self) -> i32 {
        // We can hardcode a value here. A negative value will
        // make our entry appear always on top, before any default
        // entries with automatic sort keys.
        -1
    }

    fn update_value(
        &self,
        (time, lastclick): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let d = time.elapsed() - lastclick.last_click;
        Some(d.as_secs())
    }

    // since we don't provide an implementation of `fn format_value`,
    // the value will just be printed with its `Debug` formatting.
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
