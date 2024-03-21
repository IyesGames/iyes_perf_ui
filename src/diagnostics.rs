//! Perf UI Entries based on Bevy Diagnostics

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::utils::FloatOrd;

use crate::prelude::*;
use crate::utils::*;

/// Perf UI Entry to display Bevy's built-in FPS measurement diagnostic.
#[derive(Component, Debug, Clone)]
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
            digits: 4,
            precision: 0,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in frame time measurement diagnostic.
///
/// Displays the frame time in *milliseconds*.
#[derive(Component, Debug, Clone)]
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
    /// Should we display the smoothed value or the raw value?
    ///
    /// Default: true (smoothed)
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
            smoothed: true,
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
            digits: 2,
            precision: 3,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in frame counter.
#[derive(Component, Debug, Clone)]
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
            digits: 6,
            sort_key: next_sort_key(),
        }
    }
}

/// Perf UI Entry to display Bevy's built-in CPU Usage measurement diagnostic.
///
/// Displays the usage as a percentage.
#[derive(Component, Debug, Clone)]
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

/// Perf UI Entry to display Bevy's built-in Memory (RAM) Usage measurement diagnostic.
///
/// Displays the usage as a percentage.
#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryMemUsage {
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

impl Default for PerfUiEntryMemUsage {
    fn default() -> Self {
        PerfUiEntryMemUsage {
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
    fn width_hint(&self) -> usize {
        width_hint_pretty_float(self.digits, self.precision)
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

impl PerfUiEntry for PerfUiEntryFPSWorst {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f32;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "FPS (min)"
        } else {
            &self.label
        }
    }
    fn width_hint(&self) -> usize {
        width_hint_pretty_float(self.digits, self.precision)
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
    fn width_hint(&self) -> usize {
        let w = width_hint_pretty_float(self.digits, self.precision);
        if self.display_units {
            w + 3
        } else {
            w
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
    fn width_hint(&self) -> usize {
        let w = width_hint_pretty_float(self.digits, self.precision);
        if self.display_units {
            w + 3
        } else {
            w
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
    fn width_hint(&self) -> usize {
        width_hint_pretty_int(self.digits)
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
    fn width_hint(&self) -> usize {
        width_hint_pretty_int(self.digits)
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

impl PerfUiEntry for PerfUiEntryCpuUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Total CPU Usage"
        } else {
            &self.label
        }
    }
    fn width_hint(&self) -> usize {
        width_hint_pretty_float(2, self.precision) + 1
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)?.value()?
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

impl PerfUiEntry for PerfUiEntryMemUsage {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Total RAM Usage"
        } else {
            &self.label
        }
    }
    fn width_hint(&self) -> usize {
        width_hint_pretty_float(2, self.precision) + 1
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(if self.smoothed {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)?.smoothed()?
        } else {
            diagnostics.get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)?.value()?
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
