# Customizable Performance/Debug Overlay for Bevy UI

[![Crates.io](https://img.shields.io/crates/v/iyes_perf_ui)](https://crates.io/crates/iyes_perf_ui)
[![docs](https://docs.rs/iyes_perf_ui/badge.svg)](https://docs.rs/iyes_perf_ui/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)

Sponsor me:

<a href="https://github.com/sponsors/inodentry"><button class="ghsponsors-button">GitHub Sponsors</button></a>

Bevy Compatibility:

| Bevy Version | Plugin Version |
|--------------|----------------|
| `0.13`       | `0.2`,`0.1`    |

---

This crate provides an implementation of an in-game performance/debug UI overlay
for the [Bevy game engine](https://bevyengine.org).

The goal of this crate is to make it as useful as possible for any Bevy project:
 - Made with Bevy UI (not egui or any other 3rd-party UI solution)
 - Easy to set up (see [`simple`](examples/simple.rs) example)
 - Customizable appearance/styling (see [`settings`](examples/settings.rs), [`fps_minimalist`](examples/fps_minimalist.rs) examples)
 - Modular! You decide what info you want to display!
   - Choose any combination of predefined entries for:
     - Framerate (FPS), Frame Time, Frame Count, ECS Entity Count, CPU Usage, RAM Usage,
       Wall Clock, Running Time, Fixed Time Step, Fixed Overstep,
       Cursor Position, Window Resolution, Window Mode, Present Mode
   - Implement your own custom entries to display anything you like!
     - (see [`custom_minimal`](examples/custom_minimal.rs) and [`custom`](examples/custom.rs) examples)
 - Support for highlighting values using a custom font or color!
   - Allows you to quickly notice if something demands your attention.

Spawning a Perf UI is as simple as:

```rust
commands.spawn((
    PerfUiRoot::default(),
    PerfUiEntryFPS::default(),
    PerfUiEntryFrameTime::default(),
    PerfUiEntryCpuUsage::default(),
    PerfUiEntryClock::default(),
    // ...
));
```

(or instead of `default`, you can configure the parameters in any of these structs)

![Screenshot of the simple example showing default configuration](screenshots/simple.png)

![Screenshot of the settings example showing multiple UIs with custom configuration](screenshots/settings.png)
