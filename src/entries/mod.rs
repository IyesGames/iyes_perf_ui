//! Predefined entry types offered by this library.

use bevy::prelude::*;
use crate::prelude::*;

/// Prelude of predefined entry types.
pub mod prelude {
    pub use super::PerfUiCompleteBundle;

    pub use super::diagnostics::{
        PerfUiEntryFPS,
        PerfUiEntryFrameTime,
        PerfUiEntryFPSWorst,
        PerfUiEntryFrameTimeWorst,
        PerfUiEntryFrameCount,
        PerfUiEntryEntityCount,
        PerfUiEntryCpuUsage,
        PerfUiEntryMemUsage,
    };
    pub use super::time::{
        PerfUiEntryClock,
        PerfUiEntryRunningTime,
        PerfUiEntryFixedTimeStep,
        PerfUiEntryFixedOverstep,
    };
    pub use super::window::{
        PerfUiEntryWindowResolution,
        PerfUiEntryWindowScaleFactor,
        PerfUiEntryWindowMode,
        PerfUiEntryWindowPresentMode,
        PerfUiEntryCursorPosition,
    };
}

pub mod diagnostics;
pub mod time;
pub mod window;

pub(crate) fn predefined_entries_plugin(app: &mut App) {
    app.add_perf_ui_entry_type::<PerfUiEntryFPS>();
    app.add_perf_ui_entry_type::<PerfUiEntryFrameTime>();
    app.add_perf_ui_entry_type::<PerfUiEntryFPSWorst>();
    app.add_perf_ui_entry_type::<PerfUiEntryFrameTimeWorst>();
    app.add_perf_ui_entry_type::<PerfUiEntryFrameCount>();
    app.add_perf_ui_entry_type::<PerfUiEntryEntityCount>();
    app.add_perf_ui_entry_type::<PerfUiEntryCpuUsage>();
    app.add_perf_ui_entry_type::<PerfUiEntryMemUsage>();
    app.add_perf_ui_entry_type::<PerfUiEntryClock>();
    app.add_perf_ui_entry_type::<PerfUiEntryRunningTime>();
    app.add_perf_ui_entry_type::<PerfUiEntryFixedTimeStep>();
    app.add_perf_ui_entry_type::<PerfUiEntryFixedOverstep>();
    app.add_perf_ui_entry_type::<PerfUiEntryWindowResolution>();
    app.add_perf_ui_entry_type::<PerfUiEntryWindowScaleFactor>();
    app.add_perf_ui_entry_type::<PerfUiEntryWindowMode>();
    app.add_perf_ui_entry_type::<PerfUiEntryWindowPresentMode>();
    app.add_perf_ui_entry_type::<PerfUiEntryCursorPosition>();
}

/// Bundle for a Perf UI with all entry types provided by `iyes_perf_ui`.
///
/// This gives you a simple one-liner to spawn a comprehensive Perf UI!
///
/// ```rust
/// commands.spawn(PerfUiCompleteBundle::default());
/// ```
///
/// If you want to create a Perf UI with specific entries of your choice,
/// just spawn an entity with [`PerfUiRoot`] + your desired entries, instead
/// of using this bundle.
///
/// ```rust
/// commands.spawn((
///     PerfUiRoot::default(),
///     PerfUiEntryFPS::default(),
///     PerfUiEntryClock::default(),
///     // ...
/// ));
/// ```
#[allow(missing_docs)]
#[derive(Bundle, Default)]
pub struct PerfUiCompleteBundle {
    pub root: PerfUiRoot,
    pub fps: PerfUiEntryFPS,
    pub fps_worst: PerfUiEntryFPSWorst,
    pub frametime: PerfUiEntryFrameTime,
    pub frametime_worst: PerfUiEntryFrameTimeWorst,
    pub frame_count: PerfUiEntryFrameCount,
    pub entity_count: PerfUiEntryEntityCount,
    pub cpu_usage: PerfUiEntryCpuUsage,
    pub mem_usage: PerfUiEntryMemUsage,
    pub fixed_timestep: PerfUiEntryFixedTimeStep,
    pub fixed_overstep: PerfUiEntryFixedOverstep,
    pub time_running: PerfUiEntryRunningTime,
    pub time_clock: PerfUiEntryClock,
    pub cursor_position: PerfUiEntryCursorPosition,
    pub window_resolution: PerfUiEntryWindowResolution,
    pub window_scale_factor: PerfUiEntryWindowScaleFactor,
    pub window_mode: PerfUiEntryWindowMode,
    pub window_present_mode: PerfUiEntryWindowPresentMode,
}
