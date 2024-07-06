use std::marker::PhantomData;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::ecs::system::lifetimeless::SQuery;

use crate::entry::{PerfUiEntry, PerfUiEntryDisplayRange};
use crate::ui::widget::{PerfUiWidget, PerfUiWidgetMarker};
use crate::utils::ColorGradient;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BarTextPosition {
    NoText,
    #[default]
    Center,
    Start,
    End,
    OutsideStart,
    OutsideEnd,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BarFillDirection {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Component)]
pub struct PerfUiWidgetBar<E: PerfUiEntryDisplayRange> {
    pub text_position: BarTextPosition,
    pub text_color_override: Option<Color>,
    pub fill_direction: BarFillDirection,
    pub bar_color: ColorGradient,
    pub bar_background: Color,
    pub bar_border_px: f32,
    pub bar_border_color: Color,
    pub bar_height_px: Option<f32>,
    pub bar_length_px: Option<f32>,
    pub entry: E,
}

#[derive(Component)]
pub struct PerfUiWidgetBarParts {
    e_bar_inner: Entity,
    e_text: Option<Entity>,
}

#[derive(Component)]
pub struct BarWidgetInnerBarMarker<E: PerfUiEntry> {
    _pd: PhantomData<E>,
}

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
            &'static mut Style,
        ), (
            With<BarWidgetInnerBarMarker<E>>,
            Without<BarWidgetMarker<E>>,
        )>,
        SQuery<&'static mut Text, With<BarWidgetTextMarker<E>>>,
    );

    fn spawn(
        &self,
        root: &crate::prelude::PerfUiRoot,
        _e_root: Entity,
        commands: &mut Commands,
        _: &mut <Self::SystemParamSpawn as SystemParam>::Item<'_, '_>,
    ) -> Entity {
        let e_bar_outer = commands.spawn((
            NodeBundle {
                background_color: BackgroundColor(self.bar_background),
                border_color: BorderColor(self.bar_border_color),
                style: Style {
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
                ..default()
            },
        )).id();
        let e_bar_inner_wrapper = commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    bottom: Val::Px(self.bar_border_px * 2.0),
                    left: Val::Px(0.0),
                    right: Val::Px(self.bar_border_px * 2.0),
                    ..default()
                },
                ..default()
            },
        )).id();
        let e_bar_inner = commands.spawn((
            BarWidgetInnerBarMarker::<E> {
                _pd: PhantomData,
            },
            NodeBundle {
                background_color: BackgroundColor(Color::NONE),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    left: Val::Percent(0.0),
                    right: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        )).id();
        commands.entity(e_bar_inner_wrapper).push_children(&[e_bar_inner]);
        commands.entity(e_bar_outer).push_children(&[e_bar_inner_wrapper]);
        let mut parts = PerfUiWidgetBarParts {
            e_bar_inner,
            e_text: None,
        };
        let e_bar_wrapper = commands.spawn((
            NodeBundle {
                style: Style {
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
                ..default()
            },
        )).id();
        commands.entity(e_bar_wrapper).push_children(&[e_bar_outer]);
        if self.text_position != BarTextPosition::NoText {
            let e_text = commands.spawn((
                BarWidgetTextMarker::<E> {
                    _pd: PhantomData,
                },
                TextBundle {
                    style: Style {
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
                    text: Text::from_section(
                        root.text_err.clone(),
                        TextStyle {
                            font: root.font_value.clone(),
                            font_size: root.fontsize_value,
                            color: self.text_color_override
                                .unwrap_or(root.err_color),
                        }
                    ),
                    ..default()
                },
            )).id();
            match self.text_position {
                BarTextPosition::OutsideStart | BarTextPosition::OutsideEnd => {
                    commands.entity(e_bar_wrapper).push_children(&[e_text]);
                }
                _ => {
                    commands.entity(e_bar_outer).push_children(&[e_text]);
                }
            }
            parts.e_text = Some(e_text);
        }
        let e_widget = commands.spawn((
            parts,
            NodeBundle {
                background_color: BackgroundColor(root.inner_background_color),
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(root.inner_margin)),
                    padding: UiRect::all(Val::Px(root.inner_padding)),
                    ..default()
                },
                ..default()
            },
        )).id();
        if root.display_labels {
            let e_label_wrapper = commands.spawn((
                NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                },
            )).id();
            let e_label = commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("{}: ", self.entry.label()),
                        TextStyle {
                            font: root.font_label.clone(),
                            font_size: root.fontsize_label,
                            color: root.label_color,
                        }
                    ),
                    ..default()
                },
            )).id();
            commands.entity(e_label_wrapper).push_children(&[e_label]);
            commands.entity(e_widget).push_children(&[e_label_wrapper]);
        }
        commands.entity(e_widget).push_children(&[e_bar_wrapper]);
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

            if let Some(mut text) = parts.e_text.and_then(|e| q_text.get_mut(e).ok()) {
                if let Some(value) = value {
                    let s = self.entry.format_value(&value);
                    text.sections[0].value = s.trim().to_owned();
                    if entry_highlight {
                        text.sections[0].style.font = root.font_highlight.clone();
                    } else {
                        text.sections[0].style.font = root.font_value.clone();
                    }
                    if self.text_color_override.is_none() {
                        let color = self.entry.value_color(&value)
                            .unwrap_or(root.default_value_color);
                        text.sections[0].style.color = color;
                    }
                } else {
                    text.sections[0].value = root.text_err.trim().to_owned();
                    text.sections[0].style.font = root.font_value.clone();
                    if self.text_color_override.is_none() {
                        text.sections[0].style.color = root.err_color;
                    }
                }
            }
        }
    }

    fn sort_key(&self) -> i32 {
        self.entry.sort_key()
    }
}

