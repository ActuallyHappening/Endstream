use bevy::prelude::*;
use derive_more::{From, Into};

use crate::utils::{IntoAssetPath, SpawnToParent, EntityCommandsExt, texture_2d};

use super::CARD_WIDTH;

#[derive(Into, From, Debug, Clone, PartialEq, Eq)]
pub struct Artwork(String);

impl From<&str> for Artwork {
	fn from(s: &str) -> Self {
		Artwork::from(s.to_owned())
	}
}

impl IntoAssetPath for Artwork {
	fn get_asset_path(&self) -> String {
		self.0.clone()
	}
}

impl Artwork {
	pub const height: f32 = 5.1;
}

impl SpawnToParent for Artwork {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::utils::mutASS,
	) -> Entity {
		let mut artwork = parent.spawn(PbrBundle {
			material: mat.add(texture_2d(ass.load(self.get_asset_path()))),
			mesh: meshs.add(shape::Quad::new(Vec2::new(CARD_WIDTH, Artwork::height)).into()),
			transform: Transform::from_translation(translation),
			..default()
		});
		artwork.name("Artwork");
		artwork.id()
	}
}
