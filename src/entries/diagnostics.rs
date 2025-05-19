//! Perf UI Entries based on Bevy Diagnostics

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::math::FloatOrd;

#[cfg(feature = "sysinfo")]
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;

use crate::prelude::*;
use crate::entry::*;
use crate::utils::*;

/// Perf UI Entry to display Bevy's built-in FPS measurement diagnostic.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryFPS {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Red-Yellow-Green gradient between 30-60-120 FPS.
    pub color_gradient: ColorGradient,
    /// Highlight the value if FPS is below this threshold.
    ///
    /// Default: `20.0`
    pub threshold_highlight: Option<f32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `None`
    pub max_value_hint: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `4`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `0`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFPS {
    fn default() -> Self {
        PerfUiEntryFPS {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_ryg(30.0, 60.0, 120.0).unwrap(),
            threshold_highlight: Some(20.0),
            max_value_hint: None,
            smoothed: true,
            digits: 4,
            precision: 0,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in FPS measurement diagnostic.
///
/// Displays the worst (lowest) value in recent history.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryFPSWorst {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Red-Yellow-Green gradient between 30-60-120 FPS.
    pub color_gradient: ColorGradient,
    /// Highlight the value if FPS is below this threshold.
    ///
    /// Default: `20.0`
    pub threshold_highlight: Option<f32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `None`
    pub max_value_hint: Option<f32>,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `4`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `0`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFPSWorst {
    fn default() -> Self {
        PerfUiEntryFPSWorst {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_ryg(30.0, 60.0, 120.0).unwrap(),
            threshold_highlight: Some(20.0),
            max_value_hint: None,
            digits: 4,
            precision: 0,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in FPS measurement diagnostic.
///
/// Displays the average of the values Bevy keeps in its history buffer.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryFPSAverage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Red-Yellow-Green gradient between 30-60-120 FPS.
    pub color_gradient: ColorGradient,
    /// Highlight the value if FPS is below this threshold.
    ///
    /// Default: `20.0`
    pub threshold_highlight: Option<f32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `None`
    pub max_value_hint: Option<f32>,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `4`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `0`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFPSAverage {
    fn default() -> Self {
        PerfUiEntryFPSAverage {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_ryg(30.0, 60.0, 120.0).unwrap(),
            threshold_highlight: Some(20.0),
            max_value_hint: None,
            digits: 4,
            precision: 0,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in FPS measurement diagnostic.
///
/// Computes the average of the lowest N percent of values in recent history.
///
/// This is akin to the "1% low" metric that is commonly used to measure
/// frame rate stability. By comparing it to the average or current FPS,
/// you get an idea of how much the frame rate fluctuates. If the values are
/// close, that indicates a smooth experience. If the difference is large,
/// that indicates lag spikes / inconsistent framerate.
///
/// The percentage of values to select is customizable, defaulting to 10%.
/// See the `filter_fraction` field.
///
/// The reason for using 10% instead of 1% as the default is that Bevy only
/// keeps a history buffer of 120 values. Using 1% would only leave 2 values
/// (rounded up). 10% is 12 values, which arguably gives a better indication
/// of framerate stability.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryFPSPctLow {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Red-Yellow-Green gradient between 30-60-120 FPS.
    pub color_gradient: ColorGradient,
    /// Highlight the value if FPS is below this threshold.
    ///
    /// Default: `20.0`
    pub threshold_highlight: Option<f32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `None`
    pub max_value_hint: Option<f32>,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `4`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `0`
    pub precision: u8,
    /// What fraction of values to select?
    ///
    /// Default: `0.1` (i.e "10% low")
    pub filter_fraction: f32,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFPSPctLow {
    fn default() -> Self {
        PerfUiEntryFPSPctLow {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_ryg(30.0, 60.0, 120.0).unwrap(),
            threshold_highlight: Some(20.0),
            max_value_hint: None,
            digits: 4,
            precision: 0,
            filter_fraction: 0.1,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in frame time measurement diagnostic.
///
/// Displays the frame time in *milliseconds*.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryFrameTime {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Display the unit ("ms") alongside the number.
    ///
    /// Default: `true`
    pub display_units: bool,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between the frametimes equivalent to 120-60-30 FPS.
    pub color_gradient: ColorGradient,
    /// Highlight the value if frame time is above this threshold.
    ///
    /// Default: frametime equivalent to 20 FPS
    pub threshold_highlight: Option<f32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `None`
    pub max_value_hint: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: false (raw)
    pub smoothed: bool,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `2`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `3`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFrameTime {
    fn default() -> Self {
        PerfUiEntryFrameTime {
            label: String::new(),
            display_units: true,
            color_gradient: ColorGradient::new_preset_gyr(
                1000.0 / 120.0,
                1000.0 / 60.0,
                1000.0 / 30.0,
            ).unwrap(),
            threshold_highlight: Some(1000.0 / 20.0),
            max_value_hint: None,
            smoothed: false,
            digits: 2,
            precision: 3,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in frame time measurement diagnostic.
///
/// Displays the worst (highest) value in recent history.
///
/// Displays the frame time in *milliseconds*.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryFrameTimeWorst {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Display the unit ("ms") alongside the number.
    ///
    /// Default: `true`
    pub display_units: bool,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between the frametimes equivalent to 120-60-30 FPS.
    pub color_gradient: ColorGradient,
    /// Highlight the value if frame time is above this threshold.
    ///
    /// Default: frametime equivalent to 20 FPS
    pub threshold_highlight: Option<f32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `None`
    pub max_value_hint: Option<f32>,
    /// Number of digits to display for the integer (whole number) part.
    ///
    /// Default: `2`
    pub digits: u8,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `3`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFrameTimeWorst {
    fn default() -> Self {
        PerfUiEntryFrameTimeWorst {
            label: String::new(),
            display_units: true,
            color_gradient: ColorGradient::new_preset_gyr(
                1000.0 / 120.0,
                1000.0 / 60.0,
                1000.0 / 30.0,
            ).unwrap(),
            threshold_highlight: Some(1000.0 / 20.0),
            max_value_hint: None,
            digits: 2,
            precision: 3,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in frame counter.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryFrameCount {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Number of digits to display.
    ///
    /// Default: `6`
    pub digits: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryFrameCount {
    fn default() -> Self {
        PerfUiEntryFrameCount {
            label: String::new(),
            digits: 6,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in ECS entity counter.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryEntityCount {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 100-1000-10000.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: `20000`
    pub threshold_highlight: Option<u32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `None`
    pub max_value_hint: Option<u32>,
    /// Number of digits to display.
    ///
    /// Default: `6`
    pub digits: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl Default for PerfUiEntryEntityCount {
    fn default() -> Self {
        PerfUiEntryEntityCount {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_gyr(100.0, 1000.0, 10000.0).unwrap(),
            threshold_highlight: Some(20000),
            max_value_hint: None,
            digits: 6,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in Process CPU Usage measurement diagnostic.
///
/// Displays the CPU usage of the current process (your game) as a percentage.
#[cfg(feature = "sysinfo")]
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryCpuUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 25%-50%-75%.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: 90%
    pub threshold_highlight: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

#[cfg(feature = "sysinfo")]
impl Default for PerfUiEntryCpuUsage {
    fn default() -> Self {
        PerfUiEntryCpuUsage {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_gyr(25.0, 50.0, 75.0).unwrap(),
            threshold_highlight: Some(90.0),
            smoothed: true,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in System CPU Usage measurement diagnostic.
///
/// Displays the Total System CPU usage as a percentage.
#[cfg(feature = "sysinfo")]
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntrySystemCpuUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 25%-50%-75%.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: 90%
    pub threshold_highlight: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

#[cfg(feature = "sysinfo")]
impl Default for PerfUiEntrySystemCpuUsage {
    fn default() -> Self {
        PerfUiEntrySystemCpuUsage {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_gyr(25.0, 50.0, 75.0).unwrap(),
            threshold_highlight: Some(90.0),
            smoothed: true,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in Process Memory (RAM) Usage measurement diagnostic.
///
/// Displays the amount of RAM used by the current process (your game) in GiB.
#[cfg(feature = "sysinfo")]
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryMemUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Display the unit ("GiB") alongside the number.
    ///
    /// Default: `true`
    pub display_units: bool,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 0.5-1.0-2.0 GiB.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: 3.0 GiB.
    pub threshold_highlight: Option<f32>,
    /// If displayed using a Bar (or other similar) widget that can
    /// show the value within a range, what should its max value be?
    ///
    /// If `None`, the value will be computed from the maximum of the
    /// color gradient and the highlight threshold.
    ///
    /// Default: `Some(4.0)`.
    pub max_value_hint: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `3`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

#[cfg(feature = "sysinfo")]
impl Default for PerfUiEntryMemUsage {
    fn default() -> Self {
        PerfUiEntryMemUsage {
            label: String::new(),
            display_units: true,
            color_gradient: ColorGradient::new_preset_gyr(0.5, 1.0, 2.0).unwrap(),
            threshold_highlight: Some(3.0),
            max_value_hint: Some(4.0),
            smoothed: true,
            precision: 3,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in System Memory (RAM) Usage measurement diagnostic.
///
/// Displays the Total System RAM usage as a percentage.
#[cfg(feature = "sysinfo")]
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntrySystemMemUsage {
    /// Custom label. If empty (default), the default label will be used.
    pub label: String,
    /// Enable color based on value.
    ///
    /// To disable (always use default color), set to empty `ColorGradient::default()`.
    ///
    /// Default: Green-Yellow-Red gradient between 25%-50%-75%.
    pub color_gradient: ColorGradient,
    /// Highlight the value if above this threshold.
    ///
    /// Default: 90%
    pub threshold_highlight: Option<f32>,
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
    pub smoothed: bool,
    /// Number of digits to display for the fractional (after the decimal point) part.
    ///
    /// Default: `2`
    pub precision: u8,
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

#[cfg(feature = "sysinfo")]
impl Default for PerfUiEntrySystemMemUsage {
    fn default() -> Self {
        PerfUiEntrySystemMemUsage {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_gyr(25.0, 50.0, 75.0).unwrap(),
            threshold_highlight: Some(90.0),
            smoothed: true,
            precision: 2,
            sort_key: next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiEntryFPS {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "FPS"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)?.smoothed()?
        } else {
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_float(self.digits, self.precision, *value)
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) < t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntryDisplayRange for PerfUiEntryFPS {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x),
                (Some(a), Some((b, _))) => Some(a.max(*b)),
                (None, None) => None,
            }
        ).map(|v| v as f64)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

impl PerfUiEntry for PerfUiEntryFPSWorst {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "FPS (worst)"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)?
            .values()
            .filter_map(|f| if !f.is_nan() {
                Some(FloatOrd(*f as f32))
            } else {
                None
            })
            .min()?.0
        )
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_float(self.digits, self.precision, *value as f64)
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| *value < t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntryDisplayRange for PerfUiEntryFPSWorst {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x),
                (Some(a), Some((b, _))) => Some(a.max(*b)),
                (None, None) => None,
            }
        )
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

impl PerfUiEntry for PerfUiEntryFPSAverage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "FPS (avg)"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)?
            .average()? as f32
        )
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_float(self.digits, self.precision, *value as f64)
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| *value < t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntryDisplayRange for PerfUiEntryFPSAverage {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x),
                (Some(a), Some((b, _))) => Some(a.max(*b)),
                (None, None) => None,
            }
        )
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

impl PerfUiEntry for PerfUiEntryFPSPctLow {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "FPS (low)"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let mut values: Vec<_> = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)?
            .values()
            .filter_map(|f| if !f.is_nan() {
                Some(FloatOrd(*f as f32))
            } else {
                None
            })
            .collect();

        if values.is_empty() {
            return None;
        }
        let bottom_len = (values.len() as f32 * self.filter_fraction).ceil() as usize;
        if bottom_len == 0 {
            return None;
        }

        values.sort_unstable();

        let sum: f32 = values.into_iter().take(bottom_len).map(|fo| fo.0).sum();
        Some(sum / bottom_len as f32)
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_float(self.digits, self.precision, *value as f64)
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| *value < t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntryDisplayRange for PerfUiEntryFPSPctLow {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x),
                (Some(a), Some((b, _))) => Some(a.max(*b)),
                (None, None) => None,
            }
        )
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

impl PerfUiEntry for PerfUiEntryFrameTime {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Frame Time"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)?.smoothed()?
        } else {
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(self.digits, self.precision, *value);
        if self.display_units {
            s.push_str(" ms");
        }
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntryDisplayRange for PerfUiEntryFrameTime {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x),
                (Some(a), Some((b, _))) => Some(a.max(*b)),
                (None, None) => None,
            }
        ).map(|v| v as f64)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

impl PerfUiEntry for PerfUiEntryFrameTimeWorst {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Frame Time (max)"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)?
            .values()
            .filter_map(|f| if !f.is_nan() {
                Some(FloatOrd(*f as f32))
            } else {
                None
            })
            .max()?.0
        )
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(self.digits, self.precision, *value as f64);
        if self.display_units {
            s.push_str(" ms");
        }
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| *value > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntryDisplayRange for PerfUiEntryFrameTimeWorst {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x),
                (Some(a), Some((b, _))) => Some(a.max(*b)),
                (None, None) => None,
            }
        )
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

impl PerfUiEntry for PerfUiEntryFrameCount {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = u32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Frame Count"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)?.value()? as u32)
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_int(self.digits, *value as i64)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntry for PerfUiEntryEntityCount {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = u32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Entity Count"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)?.value()? as u32)
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        format_pretty_int(self.digits, *value as i64)
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| *value > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl PerfUiEntryDisplayRange for PerfUiEntryEntityCount {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x as u32),
                (Some(a), Some((b, _))) => Some(a.max(*b as u32)),
                (None, None) => None,
            }
        )
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0)
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntry for PerfUiEntryCpuUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "CPU Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        s.push('%');
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntryDisplayRange for PerfUiEntryCpuUsage {
    fn max_value_hint(&self) -> Option<Self::Value> {
        Some(100.0)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntry for PerfUiEntrySystemCpuUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "System CPU Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        s.push('%');
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntryDisplayRange for PerfUiEntrySystemCpuUsage {
    fn max_value_hint(&self) -> Option<Self::Value> {
        Some(100.0)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntry for PerfUiEntryMemUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "RAM Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        if self.display_units {
            s.push_str(" GiB");
        }
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntryDisplayRange for PerfUiEntryMemUsage {
    fn max_value_hint(&self) -> Option<Self::Value> {
        self.max_value_hint.or(
            match (self.threshold_highlight, self.color_gradient.max_stop()) {
                (Some(x), None) => Some(x),
                (None, Some((x, _))) => Some(*x),
                (Some(a), Some((b, _))) => Some(a.max(*b)),
                (None, None) => None,
            }
        ).map(|v| v as f64)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntry for PerfUiEntrySystemMemUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "System RAM Usage"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE)?.value()?
        })
    }
    fn format_value(
        &self,
        value: &Self::Value,
    ) -> String {
        let mut s = format_pretty_float(2, self.precision, *value);
        s.push('%');
        s
    }
    fn value_color(
        &self,
        value: &Self::Value,
    ) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }
    fn value_highlight(
        &self,
        value: &Self::Value,
    ) -> bool {
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

#[cfg(feature = "sysinfo")]
impl PerfUiEntryDisplayRange for PerfUiEntrySystemMemUsage {
    fn max_value_hint(&self) -> Option<Self::Value> {
        Some(100.0)
    }
    fn min_value_hint(&self) -> Option<Self::Value> {
        Some(0.0)
    }
}
