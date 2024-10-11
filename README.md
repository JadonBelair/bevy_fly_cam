# bevy_fly_cam

A simple plugin to add a 3D free cam with fps controls to your Bevy project

## Example Usage

```rust
use bevy::prelude::*;
use bevy_fly_cam::FlyCamPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FlyCamPlugin)
    .run();
}
```
