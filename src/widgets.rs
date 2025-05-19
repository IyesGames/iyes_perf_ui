//! Predefined widget types offered by this library.

#[allow(unused_imports)]
use bevy::prelude::*;
#[allow(unused_imports)]
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
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryFPSPctLow>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryFPSAverage>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryEntityCount>, _>();
    #[cfg(feature = "sysinfo")]
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryCpuUsage>, _>();
    #[cfg(feature = "sysinfo")]
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryMemUsage>, _>();
    #[cfg(feature = "sysinfo")]
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntrySystemCpuUsage>, _>();
    #[cfg(feature = "sysinfo")]
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntrySystemMemUsage>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryFixedOverstep>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryRenderCpuTime>, _>();
    app.add_perf_ui_widget::<bar::PerfUiWidgetBar<PerfUiEntryRenderGpuTime>, _>();
}
