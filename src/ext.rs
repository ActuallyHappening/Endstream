use bevy::prelude::*;
use bevy::{ecs::system::EntityCommands};
use bevy_mod_picking::{backends::rapier::RapierPickTarget, prelude::*};

pub type ASS<'a> = (
	ResMut<'a, Assets<Mesh>>,
	ResMut<'a, Assets<StandardMaterial>>,
	ResMut<'a, AssetServer>,
);
pub type mutASS<'a, 'b> = (
	&'b mut ResMut<'a, Assets<Mesh>>,
	&'b mut ResMut<'a, Assets<StandardMaterial>>,
	&'b mut ResMut<'a, AssetServer>,
);

pub trait EntityCommandsExt {
	fn with_picking(&mut self) -> &mut Self;

	fn as_pick_camera(&mut self) -> &mut Self;

	fn name(&mut self, name: &'static str) -> &mut Self;
}

impl EntityCommandsExt for EntityCommands<'_, '_, '_> {
	/// Adds bevy_mod_picking magic
	fn with_picking(&mut self) -> &mut Self {
		self
			.insert(PickableBundle::default())
			.insert(RapierPickTarget)
			.insert(RaycastPickTarget::default())
	}

	/// Adds bevy_mod_picking magic to camera
	fn as_pick_camera(&mut self) -> &mut Self {
		self
			.insert(RapierPickCamera::default())
			.insert(RaycastPickCamera::default())
	}

	fn name(&mut self, name: &'static str) -> &mut Self {
		self.insert(Name::new(name))
	}
}

pub trait IntoAssetPath {
	fn get_asset_path(&self,) -> String;
}

/// Offset by `Vec3::Z * almost_zero` to avoid z-fighting
pub trait SpawnToParent {
	fn spawn_using_entity_commands(&self, parent: &mut ChildBuilder<'_, '_, '_>, ass: mutASS) -> Entity;
	
	// fn spawn_using_commands(&self, commands: &mut Commands) -> Entity {
	// 	commands.spawn_empty().with_children(|parent| {
	// 		self.spawn_using_entity_commands(parent);
	// 	}).id()

	// 	// let mut parent = commands.spawn_empty();
	// 	// parent.name("trait SpawnToParent virtual entity-commands");

	// 	// let entity = self.spawn_using_entity_commands(&mut parent);
	// 	// parent.id()
	// }
}