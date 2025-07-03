mod framedata;
mod input;
mod player;
mod simul;
mod timer;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
