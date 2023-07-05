use std::borrow::Cow;

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

	fn name<T: Into<Cow<'static, str>>>(&mut self, name: T) -> &mut Self;
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

	fn name<T: Into<Cow<'static, str>>>(&mut self, name: T) -> &mut Self {
		self.insert(Name::new(name))
	}
}

/// Clearly demonstrates that this tyipcally enum directly corrolates uniquely to an asset
/// as can be directly loaded using the [AssetServer].
pub trait IntoAssetPath {
	fn get_asset_path(&self,) -> String;
}

pub trait TransformExt {
	fn translate(self, delta: Vec3) -> Self;
}
impl TransformExt for Transform {
	fn translate(mut self, delta: Vec3) -> Self {
		self.translation += delta;
		self
	}
}

/// Offset by `Vec3::Z * almost_zero` to avoid z-fighting
pub trait SpawnToParent {
	fn spawn_using_entity_commands(&self, parent: &mut ChildBuilder<'_, '_, '_>, translation: Vec3, ass: mutASS) -> Entity;
	
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