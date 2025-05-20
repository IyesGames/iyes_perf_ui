# Changelog

Notable user-facing changes with each release version will be described in this file.

## [0.5.0]: 2025-05-20

Added:
 - `PerfUiEntryFPSAverage` entry (Average of recent frames).
 - `PerfUiEntryFPSPctLow` entry (Average of the slowest N% of recent frames).
 - `PerfUiEntrySystemCpuUsage`/`PerfUiEntrySystemMemUsage` entries (equivalent to the old `PerfUiEntryCpuUsage`/`PerfUiEntryMemUsage`)

Changed:
 - Bevy 0.16 support.
 - `PerfUiEntryCpuUsage`/`PerfUiEntryMemUsage` now report the CPU/RAM usage of the current process (your game/app), rather than total system usage. `PerfUiEntryMemUsage` now reports GiB instead of percentage.
 - `PerfUiEntryFrameTime`/`PerfUiEntryRenderCpuTime`/`PerfUiEntryRenderGpuTime` now default to un-smoothed (raw) values, to help identify slow frames from screenshots.

Fixed:
 - UI Root entity now has a `Name` component to help with debugging tools.
 - Entries with a `max_value_hint` now fallback to the max of either the color gradient or the highlight threshold, if set to None. Previously, only the color gradient was used.

## [0.4.0]: 2025-02-26

Added:
 - `PerfUiEntryRenderCpuTime`, `PerfUiEntryRenderGpuTime` entries.

Changed:
 - Bevy 0.15 support.
 - Default text parameters and font size, due to Bevy's new text backend.
 - `PerfUiRoot` is now added automatically as a "required component".
 - Bundles renamed and no longer contain `PerfUiRoot`, just entry types:
   - `PerfUiCompleteBundle` -> `PerfUiAllEntries`
   - `PerfUiBundle` -> `PerfUiDefaultEntries`
 - Built-in entries based on `bevy_window` are now behind a `window` cargo feature (enabled by default)

Removed:
 - Width hints and automatic sizing of the values column. The values column now has a constant size.

## [0.3.0]: 2024-07-06

Added:
 - Widget framework: `PerfUiWidget` trait, allowing implementations of alternative widgets.
 - Bar Widget
 - More helper bundles for spawning Perf UIs: `PerfUiBundle`, `PerfUi*Entries` (for various groups of entries).
 - Example showing how to toggle (spawn/despawn) Perf UI.
 - All built-in widget implementations gated behind `"widgets"` cargo feature. Enabled by default. Disable for a smaller build, if you do not intend to use any of the provided widgets.

Changed:
 - Bevy 0.14 support.
 - `ColorGradient` now stores and interpolates colors in the OKLAB color space.
 - All built-in entry implementations gated behind `"entries"` cargo feature. Enabled by default. Disable for a smaller build, if you only intend to use your own custom entry implementations.
 - CPU and RAM entries gated behind `sysinfo` cargo feature (which needs to enable `sysinfo_plugin` on `bevy`). Enabled by default.

Fixed:
 - Run Conditions now take into account visibility, eliminating perf overhead when Perf UI is spawned but not visible.

## [0.2.3]: 2024-03-21

Added:
 - `PerfUiCompleteBundle` to allow spawning a Perf UI with all our entries in one line of code!

Changed:
 - `simple` example renamed to `specific_entries`. New `simple` example uses `PerfUiCompleteBundle`.

## [0.2.2]: 2024-03-21

Added:
 - `PerfUiEntryWindowScaleFactor` built-in entry.
 - `ColorGradient` is now in `prelude`.

## [0.2.1]: 2024-03-21

Added:
 - `ColorGradient::single()` constructor for when you want a fixed color.

## [0.2.0]: 2024-03-21

Added:
 - `PerfUiRoot.inner_background_color_highlight`: different background color for highlighted entries.
 - `PerfUiRoot.display_labels`: can be used to disable the labels column and show bare values.
 - `PerfUiRoot.layout_horizontal`: display entries horizontally instead of vertically.
 - [`fps_minimalist`](examples/fps_minimalist.rs) example, shows how to make a minimal FPS counter.
 - `label` field on all built-in entry types, to allow customizable label strings.
 - Built-in entries can now display units (opt-out) with their values, where relevant.
 - `PerfUiEntryCursorPosition` built-in entry.
 - `PerfUiEntryWindowResolution` built-in entry.
 - `PerfUiEntryWindowMode` built-in entry.
 - `PerfUiEntryWindowPresentMode` built-in entry.
 - `PerfUiEntryFixedTimeStep` built-in entry.
 - `PerfUiEntryFixedOverstep` built-in entry.
 - `PerfUiEntry::width_hint` optional method. If implemented, allows ensuring the UI is correctly
   sized and the values are correctly aligned at all times.
 - `utils::width_hint_pretty_*` helper functions for values formatted using `utils::format_pretty_*`.
 - `utils::ColorGradient` helper to allow for custom gradients / color interpolation.

Changed:
 - `enable_{color,highlight}` and `threshold_*` fields on built-in entry types that support
   coloring/highlighting based on value, have been replaced with `color_gradient` to allow full customization.
 - `PerfUiEntry::update_value` now takes `&self`. It is no longer allowed to mutate the component.
 - `PerfUiEntry::label` now returns `&str` instead of `String`.

Fixed:
 - The UI will now correctly update itself if you change any parameters after it has already been spawned.
 - UI entries will now correctly disappear if you remove their PerfUIEntry component.

Removed:
 - `utils::ryg_gradient_{up,down}`. Use the new `utils::ColorGradient` instead.

## [0.1.0]: 2024-03-19

Initial Release

[0.5.0]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.5.0
[0.4.0]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.4.0
[0.3.0]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.3.0
[0.2.3]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.2.3
[0.2.2]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.2.2
[0.2.1]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.2.1
[0.2.0]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.2.0
[0.1.0]: https://github.com/IyesGames/iyes_perf_ui/tree/v0.1.0
