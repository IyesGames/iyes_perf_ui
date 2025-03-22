//! Perf UI Entries for Bevy Render

use bevy::prelude::*;
use bevy::diagnostic::DiagnosticsStore;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;

use crate::prelude::*;
use crate::entry::*;
use crate::utils::*;

/// Perf UI Entry to display the CPU time spent on rendering.
///
/// This is the sum of all the `render/*/elapsed_cpu` diagnostics reported by `bevy_render`.
///
/// This value is an indication of how efficiently you are utilizing the
/// GPU APIs (`wgpu` and its backends, like Vulkan or DirectX 12).
/// Better API usage will reduce the value.
///
/// Displays the CPU time in *milliseconds*.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryRenderCpuTime {
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
    /// Highlight the value if above this threshold.
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

impl Default for PerfUiEntryRenderCpuTime {
    fn default() -> Self {
        PerfUiEntryRenderCpuTime {
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

/// Perf UI Entry to display the GPU time spent on rendering.
///
/// This is the sum of all the `render/*/elapsed_gpu` diagnostics reported by `bevy_render`.
///
/// This value is an indication of how much work the GPU is doing. For example,
/// optimizing your shaders and drawing less stuff will make this value go down.
///
/// Displays the GPU time in *milliseconds*.
#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryRenderGpuTime {
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
    /// Highlight the value if above this threshold.
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

impl Default for PerfUiEntryRenderGpuTime {
    fn default() -> Self {
        PerfUiEntryRenderGpuTime {
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

impl PerfUiEntry for PerfUiEntryRenderCpuTime {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Render CPU Time"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let mut total = None;
        for diag in diagnostics.iter() {
            let path = diag.path().as_str();
            if !path.starts_with("render") || !path.ends_with("elapsed_cpu") {
                continue;
            }
            let value = if self.smoothed {
                diag.smoothed()
            } else {
                diag.value()
            };
            if let Some(v) = value {
                if let Some(t) = total.as_mut() {
                    *t += v;
                } else {
                    total = Some(v);
                }
            }
        }
        total
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

impl PerfUiEntryDisplayRange for PerfUiEntryRenderCpuTime {
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

impl PerfUiEntry for PerfUiEntryRenderGpuTime {
    type SystemParam = SRes<DiagnosticsStore>;
    type Value = f64;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Render GPU Time"
        } else {
            &self.label
        }
    }
    fn update_value(
        &self,
        diagnostics: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let mut total = None;
        for diag in diagnostics.iter() {
            let path = diag.path().as_str();
            if !path.starts_with("render") || !path.ends_with("elapsed_gpu") {
                continue;
            }
            let value = if self.smoothed {
                diag.smoothed()
            } else {
                diag.value()
            };
            if let Some(v) = value {
                if let Some(t) = total.as_mut() {
                    *t += v;
                } else {
                    total = Some(v);
                }
            }
        }
        total
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

impl PerfUiEntryDisplayRange for PerfUiEntryRenderGpuTime {
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
