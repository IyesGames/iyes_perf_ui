//! Bar Widget
//!
//! Displays a Perf UI entry as a "bar", instead of a bare value.
//!
//! To use it, simply wrap your entry type in the [`PerfUiWidgetBar`]
//! struct, and insert that as a component to your Perf UI entity,
//! instead of inserting the entry directly as a component.

use std::marker::PhantomData;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::ecs::system::lifetimeless::SQuery;

use crate::entry::{PerfUiEntry, PerfUiEntryDisplayRange};
use crate::ui::widget::{PerfUiWidget, PerfUiWidgetMarker};
use crate::utils::ColorGradient;

/// Where should the text value be displayed inside the bar?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BarTextPosition {
    /// Do not display the value as text. Bar only.
    NoText,
    /// Position the text inside the bar, at the center.
    #[default]
    Center,
    /// Position the text inside the bar, at the start.
    Start,
    /// Position the text inside the bar, at the end.
    End,
    /// Position the text outside the bar, at the start.
    OutsideStart,
    /// Position the text outside the bar, at the end.
    OutsideEnd,
}

/// Which way should the bar fill up?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BarFillDirection {
    /// From left to right.
    #[default]
    Left,
    /// From the center, expanding towards both sides.
    Center,
    /// From right to left.
    Right,
}

/// Display a Perf UI entry as a Bar Widget.
///
/// This struct wraps the entry type, which will be the source
/// of the data value to be displayed by the bar.
///
/// It allows you to customize the properties of the bar.
#[derive(Component)]
pub struct PerfUiWidgetBar<E: PerfUiEntryDisplayRange> {
    /// Should the bar also display the value as text? Where?
    pub text_position: BarTextPosition,
    /// Set the color of the text that displays the value.
    pub text_color_override: Option<Color>,
    /// Which way should the bar fill up?
    pub fill_direction: BarFillDirection,
    /// What should be the color of the filled portion of the bar?
    pub bar_color: ColorGradient,
    /// What should be the color of the unfilled portion of the bar?
    pub bar_background: Color,
    /// The thickness of the bar's border.
    pub bar_border_px: f32,
    /// The color of the bar's border.
    pub bar_border_color: Color,
    /// Force the bar to have a specific height in pixels.
    pub bar_height_px: Option<f32>,
    /// Force the bar to have a specific length in pixels.
    pub bar_length_px: Option<f32>,
    /// The entry (data source for the bar widget).
    pub entry: E,
}

#[doc(hidden)]
#[derive(Component)]
pub struct PerfUiWidgetBarParts {
    e_bar_inner: Entity,
    e_text: Option<Entity>,
}

#[doc(hidden)]
#[derive(Component)]
pub struct BarWidgetInnerBarMarker<E: PerfUiEntry> {
    _pd: PhantomData<E>,
}

#[doc(hidden)]
#[derive(Component)]
pub struct BarWidgetTextMarker<E: PerfUiEntry> {
    _pd: PhantomData<E>,
}

impl<V, E> PerfUiWidgetBar<E>
where
    V: num_traits::Num + num_traits::ToPrimitive + Copy,
    E: PerfUiEntry<Value = V> + PerfUiEntryDisplayRange,
{
    /// Create a new Bar widget with default settings
    pub fn new(entry: E) -> Self {
        Self {
            text_position: default(),
            text_color_override: None,
            fill_direction: default(),
            bar_color: ColorGradient::single(Color::srgb(0.5, 0.5, 0.5)),
            bar_background: Color::srgba(0.0, 0.0, 0.0, 0.5),
            bar_border_color: Color::srgb(0.0, 0.0, 0.0),
            bar_border_px: 1.0,
            bar_height_px: None,
            bar_length_px: None,
            entry,
        }
    }

    fn get_range(&self) -> Option<(f64, f64)> {
        use num_traits::NumCast;
        let g_min = self.bar_color.min_stop()
            .map(|(v, _)| *v as f64);
        let g_max = self.bar_color.max_stop()
            .map(|(v, _)| *v as f64);
        let h_min = self.entry.min_value_hint()
            .and_then(|v| <f64 as NumCast>::from(v));
        let h_max = self.entry.max_value_hint()
            .and_then(|v| <f64 as NumCast>::from(v));
        if g_min == g_max {
            if let (Some(h_min), Some(h_max)) = (h_min, h_max) {
                return Some((h_min, h_max));
            } else {
                return None;
            }
        }
        let v_min = match (g_min, h_min) {
            (Some(g_min), Some(h_min)) => g_min.min(h_min),
            (Some(g_min), None) => g_min,
            (None, Some(h_min)) => h_min,
            (None, None) => return None,
        };
        let v_max = match (g_max, h_max) {
            (Some(g_max), Some(h_max)) => g_max.max(h_max),
            (Some(g_max), None) => g_max,
            (None, Some(h_max)) => h_max,
            (None, None) => return None,
        };
        Some((v_min, v_max))
    }
}

type BarWidgetMarker<E> = PerfUiWidgetMarker<PerfUiWidgetBar<E>>;

impl<V, E> PerfUiWidget<E> for PerfUiWidgetBar<E>
where
    V: num_traits::Num + num_traits::ToPrimitive + Copy,
    E: PerfUiEntry<Value = V> + PerfUiEntryDisplayRange,
{
    type SystemParamSpawn = ();
    type SystemParamUpdate = (
        E::SystemParam,
        SQuery<(
            &'static mut BackgroundColor,
            &'static PerfUiWidgetBarParts,
        ), (
            With<BarWidgetMarker<E>>,
            Without<BarWidgetInnerBarMarker<E>>,
        )>,
        SQuery<(
            &'static mut BackgroundColor,
            &'static mut Node,
        ), (
            With<BarWidgetInnerBarMarker<E>>,
            Without<BarWidgetMarker<E>>,
        )>,
        SQuery<(&'static mut Text, &'static mut TextColor, &'static mut TextFont), With<BarWidgetTextMarker<E>>>,
    );

    fn spawn(
        &self,
        root: &crate::prelude::PerfUiRoot,
        _e_root: Entity,
        commands: &mut Commands,
        _: &mut <Self::SystemParamSpawn as SystemParam>::Item<'_, '_>,
    ) -> Entity {
        let e_bar_outer = commands.spawn((
            BackgroundColor(self.bar_background),
            BorderColor(self.bar_border_color),
            Node {
                border: UiRect::all(Val::Px(self.bar_border_px)),
                height: if let Some(h) = self.bar_height_px {
                    Val::Px(h)
                } else {
                    Val::Auto
                },
                width: if let Some(w) = self.bar_length_px {
                    Val::Px(w)
                } else {
                    Val::Auto
                },
                flex_grow: if self.bar_length_px.is_some() {
                    0.0
                } else {
                    1.0
                },
                justify_content: match self.text_position {
                    BarTextPosition::Start => JustifyContent::FlexStart,
                    BarTextPosition::End => JustifyContent::FlexEnd,
                    _ => JustifyContent::Center,
                },
                align_items: AlignItems::Center,
                ..default()
            },
        )).id();
        let e_bar_inner_wrapper = commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                bottom: Val::Px(self.bar_border_px * 2.0),
                left: Val::Px(0.0),
                right: Val::Px(self.bar_border_px * 2.0),
                ..default()
            },
        )).id();
        let e_bar_inner = commands.spawn((
            BarWidgetInnerBarMarker::<E> {
                _pd: PhantomData,
            },
            BackgroundColor(Color::NONE),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                left: Val::Percent(0.0),
                right: Val::Percent(100.0),
                ..default()
            },
        )).id();
        commands.entity(e_bar_inner_wrapper).add_child(e_bar_inner);
        commands.entity(e_bar_outer).add_child(e_bar_inner_wrapper);
        let mut parts = PerfUiWidgetBarParts {
            e_bar_inner,
            e_text: None,
        };
        let e_bar_wrapper = commands.spawn((
            Node {
                padding: UiRect::all(Val::Px(4.0)),
                width: if let Some(w) = root.values_col_width {
                    Val::Px(w)
                } else {
                    Val::Auto
                },
                flex_grow: if root.values_col_width.is_some() {
                    0.0
                } else {
                    1.0
                },
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: match self.text_position {
                    BarTextPosition::OutsideStart => FlexDirection::RowReverse,
                    BarTextPosition::OutsideEnd => FlexDirection::Row,
                    _ => default(),
                },
                align_items: if self.bar_height_px.is_some() {
                    AlignItems::Center
                } else {
                    AlignItems::Stretch
                },
                ..default()
            },
        )).id();
        commands.entity(e_bar_wrapper).add_child(e_bar_outer);
        if self.text_position != BarTextPosition::NoText {
            let e_text = commands.spawn((
                BarWidgetTextMarker::<E> {
                    _pd: PhantomData,
                },
                Node {
                    margin: match self.text_position {
                        BarTextPosition::OutsideEnd => UiRect {
                            left: Val::Px(4.0),
                            ..UiRect::all(Val::Auto)
                        },
                        BarTextPosition::OutsideStart => UiRect {
                            right: Val::Px(4.0),
                            ..UiRect::all(Val::Auto)
                        },
                        _ => UiRect::all(Val::Auto),
                    },
                    ..default()
                },
                Text(root.text_err.clone()),
                TextFont {
                    font: root.font_value.clone(),
                    font_size: root.fontsize_value,
                    ..default()
                },
                TextColor(self.text_color_override .unwrap_or(root.err_color))
            )).id();
            match self.text_position {
                BarTextPosition::OutsideStart | BarTextPosition::OutsideEnd => {
                    commands.entity(e_bar_wrapper).add_child(e_text);
                }
                _ => {
                    commands.entity(e_bar_outer).add_child(e_text);
                }
            }
            parts.e_text = Some(e_text);
        }
        let e_widget = commands.spawn((
            parts,
            BackgroundColor(root.inner_background_color),
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(root.inner_margin)),
                padding: UiRect::all(Val::Px(root.inner_padding)),
                ..default()
            },
        )).id();
        if root.display_labels {
            let e_label_wrapper = commands.spawn((
                Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
            )).id();
            let e_label = commands.spawn((
                Text(format!("{}: ", self.entry.label())),
                TextFont {
                    font: root.font_label.clone(),
                    font_size: root.fontsize_label,
                    ..default()
                },
                TextColor(root.label_color)
            )).id();
            commands.entity(e_label_wrapper).add_child(e_label);
            commands.entity(e_widget).add_child(e_label_wrapper);
        }
        commands.entity(e_widget).add_child(e_bar_wrapper);
        e_widget
    }

    fn update(
        &self,
        root: &crate::prelude::PerfUiRoot,
        _e_root: Entity,
        e_widget: Entity,
        (
            entry_param,
            q_widget,
            q_bar_inner,
            q_text,
        ): &mut <Self::SystemParamUpdate as SystemParam>::Item<'_, '_>,
    ) {
        if let Ok((mut bgcolor, parts)) = q_widget.get_mut(e_widget) {
            let value = self.entry.update_value(entry_param);
            let entry_highlight = value
                .map(|v| self.entry.value_highlight(&v))
                .unwrap_or(false);

            if entry_highlight {
                bgcolor.0 = root.inner_background_color_highlight;
            } else {
                bgcolor.0 = root.inner_background_color;
            }

            if let Ok((mut bar_color, mut bar_style)) = q_bar_inner.get_mut(parts.e_bar_inner) {
                use num_traits::NumCast;
                let value = value.and_then(|v| <f64 as NumCast>::from(v));

                if let Some(value) = value {
                    bar_color.0 = self.bar_color.get_color_for_value(value as f32)
                        .unwrap_or(Color::NONE);
                }

                if let (Some(value), Some((v_min, v_max))) = (value, self.get_range()) {
                    let pct = ((value - v_min) / (v_max - v_min))
                        .clamp(0.0, 1.0) * 100.0;
                    match self.fill_direction {
                        BarFillDirection::Left => {
                            bar_style.left = Val::Percent(0.0);
                            bar_style.right = Val::Percent((100.0 - pct) as f32);
                        }
                        BarFillDirection::Right => {
                            bar_style.left = Val::Percent((100.0 - pct) as f32);
                            bar_style.right = Val::Percent(0.0);
                        }
                        BarFillDirection::Center => {
                            bar_style.left = Val::Percent(((100.0 - pct) / 2.0) as f32);
                            bar_style.right = Val::Percent(((100.0 - pct) / 2.0) as f32);
                        }
                    }
                } else {
                    // we can't really fill a bar if we don't know the range,
                    // so just force it to be empty
                    bar_style.left = Val::Percent(0.0);
                    bar_style.right = Val::Percent(100.0);
                }
            }

            if let Some((mut text, mut color, mut font)) = parts.e_text.and_then(|e| q_text.get_mut(e).ok()) {
                if let Some(value) = value {
                    let s = self.entry.format_value(&value);
                    *text = Text(s.trim().to_owned());
                    if entry_highlight {
                        font.font = root.font_highlight.clone();
                    } else {
                        font.font = root.font_value.clone();
                    }
                    if self.text_color_override.is_none() {
                        let new_color = self.entry.value_color(&value)
                            .unwrap_or(root.default_value_color);
                        *color = TextColor(new_color);
                    }
                } else {
                    *text = Text(root.text_err.trim().to_owned());
                    font.font = root.font_value.clone();
                    if self.text_color_override.is_none() {
                        *color = TextColor(root.err_color);
                    }
                }
            }
        }
    }

    fn sort_key(&self) -> i32 {
        self.entry.sort_key()
    }
}

