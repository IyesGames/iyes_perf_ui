use bevy::prelude::*;
use crate::prelude::*;

/// Prelude of predefined widget types.
pub mod prelude {
    pub use super::bar::PerfUiWidgetBar;
}

pub mod bar;

#[cfg(feature = "entries")]
pub(crate) fn predefined_widgets_plugin(app: &mut App) {
    use crate::entries::prelude::*;
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryFPS>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryFrameTime>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryFPSWorst>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryFrameTimeWorst>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryEntityCount>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryCpuUsage>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryMemUsage>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryFixedOverstep>, _>();
}
