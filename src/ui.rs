//! Framework for the UI hierarchy

use bevy::prelude::*;
use self::root::PerfUiRoot;

pub mod root;
pub mod widget;

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct PerfUiSortKey(i32);

pub(crate) fn rc_sort_perf_ui_widgets(
    q: Query<(), (With<PerfUiRoot>, Changed<Children>)>,
) -> bool {
    !q.is_empty()
}

pub(crate) fn sort_perf_ui_widgets(
    mut q_root: Query<&mut Children, (With<PerfUiRoot>, Changed<Children>)>,
    q_sortkey: Query<&PerfUiSortKey>,
) {
    for mut children in &mut q_root {
        children.sort_by_key(|e| q_sortkey.get(*e).map(|k| k.0).unwrap_or(0));
    }
}
