use super::CardVisual;
use crate::{
	ext::{IntoAssetPath, SpawnToParent, EntityCommandsExt},
	texture_2d,
};
use bevy::prelude::*;

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Back {
	Dotty,
	Liney,
}

impl IntoAssetPath for Back {
	fn get_asset_path(&self) -> String {
		match self {
			Back::Dotty => "cards/back-dotty.png",
			Back::Liney => "cards/back-liney.png",
		}
		.to_string()
	}
}

impl Back {
	const dimensions: Vec2 = CardVisual::dimensions;
}

impl SpawnToParent for Back {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::ext::mutASS,
	) -> Entity {
		let mut back = parent.spawn(PbrBundle {
			transform: Transform::from_translation(translation),
			material: mat.add(texture_2d(ass.load(self.get_asset_path()))),
			mesh: meshs.add(shape::Quad::new(Back::dimensions).into()),
			..default()
		});
		back.name("Back");
		back.id()
	}
}
