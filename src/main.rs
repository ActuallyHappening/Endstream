use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use endstream::MainPlugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(MainPlugin)
		.add_plugin(bevy_editor_pls::prelude::EditorPlugin::default())
		.add_plugins(
			DefaultPickingPlugins
				.build()
				.disable::<DefaultHighlightingPlugin>()
				.disable::<DebugPickingPlugin>()
				.enable::<RapierBackend>(),
		)
		.run();
}
