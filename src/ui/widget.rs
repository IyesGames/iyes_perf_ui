//! Framework for creating different widgets for displaying Perf UI entries.

use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::ecs::system::StaticSystemParam;
use bevy::ecs::system::lifetimeless::SQuery;
use crate::ui::root::PerfUiRoot;
use crate::entry::PerfUiEntry;

use super::PerfUiSortKey;

/// Trait for Perf UI Widgets.
pub trait PerfUiWidget<T: PerfUiEntry>: Component {
    /// Any extra system parameters you need to setup the UI.
    type SystemParamSpawn: SystemParam + 'static;
    /// Any system parameters you need to update the UI.
    type SystemParamUpdate: SystemParam + 'static;

    /// Spawn the UI hierarchy for this entry.
    ///
    /// You may spawn either:
    ///  - A single UI Node entity
    ///  - A UI Node entity with children under it
    ///
    /// In either case, this method must return the newly spawned `Entity`.
    ///
    /// You should *not* spawn additional entities that are not children
    /// of the entity you return from this method.
    ///
    /// After this method is called, a `PerfUiWidgetMarker<T>` will
    /// automatically be inserted on your entity. This allows it to be
    /// tracked in the future (for despawning, etc.).
    ///
    /// You are given access to your widget component (`self`), the
    /// root ui configuration component (`root`), the entity ID of the
    /// root entity (`e_root`), in case you need them.
    ///
    /// If you need any additional data, you can put Bevy system parameters
    /// into `type SystemParamSpawn` and access them via `param`.
    ///
    /// Use the provided `commands` for spawning your entities.
    ///
    /// Do not add yourself as a child of `e_root`! `iyes_perf_ui` will take
    /// care of that for you!
    fn spawn(
        &self,
        root: &PerfUiRoot,
        e_root: Entity,
        commands: &mut Commands,
        param: &mut <Self::SystemParamSpawn as SystemParam>::Item<'_, '_>,
    ) -> Entity;

    /// Update the UI for the widget.
    ///
    /// You can use arbitrary Bevy system parameters to access the data
    /// you need to update the UI. Put them in `type SystemParamUpdate`
    /// and access them via `param`.
    ///
    /// You are given access to your widget component (`self`), the
    /// root ui configuration component (`root`), the entity ID of the
    /// root entity (`e_root`), and the entity ID of the widget entity
    /// (`e_widget`, the one you returned from `spawn`), in case you need them.
    fn update(
        &self,
        root: &PerfUiRoot,
        e_root: Entity,
        e_widget: Entity,
        param: &mut <Self::SystemParamUpdate as SystemParam>::Item<'_, '_>,
    );

    /// The sort key of the entry that the widget is displaying.
    fn sort_key(&self) -> i32;
}

/// Marker component to keep track of a widget's toplevel entity
#[derive(Component)]
pub struct PerfUiWidgetMarker<W> {
    e_root: Entity,
    _pd: PhantomData<W>,
}

pub(crate) fn rc_setup_perf_ui_widget<E: PerfUiEntry, W: PerfUiWidget<E>>(
    q: Query<(), Or<(Changed<W>, Changed<PerfUiRoot>)>>,
    removed: RemovedComponents<W>,
) -> bool {
    !q.is_empty() || !removed.is_empty()
}

pub(crate) fn setup_perf_ui_widget<E: PerfUiEntry, W: PerfUiWidget<E>>(
    mut commands: Commands,
    q_root: Query<(Entity, &PerfUiRoot, &W), Or<(Changed<W>, Changed<PerfUiRoot>)>>,
    q_widget: Query<(Entity, &PerfUiWidgetMarker<W>)>,
    mut removed: RemovedComponents<W>,
    widget_param: StaticSystemParam<W::SystemParamSpawn>,
) {
    let mut widget_param = widget_param.into_inner();

    // handle any removals:
    // if the entry component was removed from a perf ui root entity,
    // we need to find the entity of the entry's UI and despawn it.
    for e_removed in removed.read() {
        if let Some(e_entry) = q_widget.iter()
            .find(|(_, marker)| marker.e_root == e_removed)
            .map(|(e, _)| e)
        {
            commands.entity(e_entry).despawn();
        }
    }
    // handle any additions or reconfigurations:
    // if an entry component was added/changed to a perf ui root entity,
    // or if the ui root component itself was changed,
    // find and despawn any existing entries and
    // spawn a new UI hierarchy for the entry.
    for (e_root, root, widget) in &q_root {
        // despawn any old/existing UI hierarchy for relevant entries
        if let Some(e_widget) = q_widget.iter()
            .find(|(_, marker)| marker.e_root == e_root)
            .map(|(e, _)| e)
        {
            commands.entity(e_widget).despawn();
        }

        let e_widget = widget.spawn(
            root, e_root, &mut commands, &mut widget_param
        );
        commands.entity(e_widget).insert((
            PerfUiWidgetMarker::<W> {
                e_root,
                _pd: PhantomData,
            },
            PerfUiSortKey(widget.sort_key()),
        ));
        commands.entity(e_root).add_child(e_widget);
    }
}

/// System that updates the values of Perf UI entries of a given type
///
/// Exposed as `pub` so you can refer to it for ordering.
#[allow(private_interfaces)]
pub fn update_perf_ui_widget<E: PerfUiEntry, W: PerfUiWidget<E>>(
    q_root: Query<(Entity, &PerfUiRoot, &W)>,
    q_widget: Query<(Entity, &PerfUiWidgetMarker<W>)>,
    widget_param: StaticSystemParam<W::SystemParamUpdate>,
) {
    let mut widget_param = widget_param.into_inner();
    for (e_widget, marker) in &q_widget {
        let Ok((e_root, root, widget)) = q_root.get(marker.e_root) else {
            continue; // TODO: should we panic here?
        };
        widget.update(root, e_root, e_widget, &mut widget_param);
    }
}

#[doc(hidden)]
#[derive(Component)]
pub struct SimpleWidgetTextMarker<E: PerfUiEntry> {
    _pd: PhantomData<E>,
}

impl<E: PerfUiEntry> PerfUiWidget<E> for E {
    type SystemParamSpawn = ();
    type SystemParamUpdate = (
        <E as PerfUiEntry>::SystemParam,
        SQuery<&'static mut BackgroundColor, With<PerfUiWidgetMarker<E>>>,
        SQuery<(&'static mut Text, &'static mut TextColor, &'static mut TextFont), With<SimpleWidgetTextMarker<E>>>,
    );

    fn spawn(
        &self,
        root: &crate::prelude::PerfUiRoot,
        _e_root: Entity,
        commands: &mut Commands,
        _: &mut <Self::SystemParamSpawn as SystemParam>::Item<'_, '_>,
    ) -> Entity {
        let e_widget = commands.spawn((
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
                Text(format!("{}: ", self.label())),
                TextColor(root.label_color),
                TextFont {
                    font: root.font_label.clone(),
                    font_size: root.fontsize_label,
                    ..default()
                },
                TextLayout {
                    linebreak: LineBreak::NoWrap,
                    justify: JustifyText::Left,
                },
            )).id();
            commands.entity(e_label_wrapper).add_child(e_label);
            commands.entity(e_widget).add_child(e_label_wrapper);
        }
        let e_text_wrapper = commands.spawn((
            Node {
                padding: UiRect::all(Val::Px(4.0)),
                width: Val::Px(root.values_col_width),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
        )).id();
        let e_text = commands.spawn((
            SimpleWidgetTextMarker::<E> {
                _pd: PhantomData,
            },
            Text(root.text_err.clone()),
            TextFont {
                font: root.font_value.clone(),
                font_size: root.fontsize_value,
                ..default()
            },
            TextColor(root.err_color),
            TextLayout {
                linebreak: LineBreak::NoWrap,
                justify: JustifyText::Right,
            },
        )).id();
        commands.entity(e_text_wrapper).add_child(e_text);
        commands.entity(e_widget).add_child(e_text_wrapper);
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
            q_text,
        ): &mut <Self::SystemParamUpdate as SystemParam>::Item<'_, '_>,
    ) {
        for (mut text, mut color, mut font) in q_text.iter_mut() {
            let mut entry_highlight = false;
            if let Some(value) = self.update_value(entry_param) {
                let new_color = self.value_color(&value)
                    .unwrap_or(root.default_value_color);
                let s = self.format_value(&value);
                *text = Text(s);
                *color = TextColor(new_color);
                if self.value_highlight(&value) {
                    font.font = root.font_highlight.clone();
                    entry_highlight = true;
                } else {
                    font.font = root.font_value.clone();
                }
            } else {
                let s = root.text_err.clone();
                *text = Text(s);
                *color = TextColor(root.err_color);
                font.font = root.font_value.clone();
            }
            if let Ok(mut entry_bgcolor) = q_widget.get_mut(e_widget) {
                if entry_highlight {
                    entry_bgcolor.0 = root.inner_background_color_highlight;
                } else {
                    entry_bgcolor.0 = root.inner_background_color;
                }
            }
        }
    }

    fn sort_key(&self) -> i32 {
        PerfUiEntry::sort_key(self)
    }
}
