use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_mod_picking::prelude::*;
use endstream::MainPlugin;

fn main() {
	App::new()
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						fit_canvas_to_parent: true,
						prevent_default_event_handling: false,
						..default()
					}),
					..default()
				})
				.build()
				.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
		)
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
