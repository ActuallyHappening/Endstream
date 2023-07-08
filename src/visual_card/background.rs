use bevy::prelude::*;

use crate::{utils::{IntoAssetPath, SpawnToParent, EntityCommandsExt}, utils::texture_2d};

use super::CardVisual;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardVisualBg {
	Yellowish,
	Blackish,
}

impl IntoAssetPath for CardVisualBg {
	fn get_asset_path(&self) -> String {
		match self {
			CardVisualBg::Blackish => "cards/Black Card Background.png",
			CardVisualBg::Yellowish => "cards/Yellow Card Background.png",
		}
		.into()
	}
}

impl CardVisualBg {
	const dimensions: Vec2 = CardVisual::dimensions;
}

impl SpawnToParent for CardVisualBg {
	fn spawn_to_child_builder(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::utils::mutASS,
	) -> Entity {
		let card_shape = shape::Quad::new(CardVisualBg::dimensions);

		// spawn parent mesh
		let mesh = meshs.add(card_shape.into());
		let material = mat.add(StandardMaterial {
			alpha_mode: AlphaMode::Opaque,
			..texture_2d(ass.load(self.get_asset_path()))
		});

		let mut bg = parent.spawn(PbrBundle {
			mesh,
			material,
			transform: Transform::from_translation(translation),
			..default()
		});
		bg.name("Background");
		bg.id()
	}
}
