use bevy::prelude::*;
use derive_more::Display;
use strum::EnumIs;

use crate::{
	ext::{EntityCommandsExt, IntoAssetPath, SpawnToParent, TransformExt},
	texture_2d, card::{CARD_WIDTH, right_margin},
};

use super::almost_zero;

#[derive(Display, Copy, Debug, Clone, PartialEq, Eq, EnumIs)]
pub enum GearType {
	Weapon,
	Device,
	Relic,
}

/// Left to right
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct GearSlots {
	pub first: GearType,
	pub second: GearType,
}

impl GearSlots {
	pub fn new(first: GearType, second: GearType) -> GearSlots {
		GearSlots { first, second }
	}
}

impl IntoAssetPath for GearType {
	fn get_asset_path(&self) -> String {
		format!(
			"card-icons/{}-icon.png",
			match self {
				GearType::Device => "device",
				GearType::Relic => "relic",
				GearType::Weapon => "weapon",
			}
		)
	}
}

impl GearType {
	const width: f32 = 0.1;
	const height: f32 = 0.1;
}

impl SpawnToParent for GearType {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::ext::mutASS,
	) -> Entity {
		let mut parent = parent.spawn(PbrBundle {
			material: mat.add(texture_2d(ass.load(self.get_asset_path()))),
			transform: Transform::from_translation(translation).translate(Vec3::Z * almost_zero),
			mesh: meshs.add(shape::Quad::new(Vec2::new(GearType::width, GearType::height)).into()),
			..default()
		});
		parent.name("GearType");
		parent.id()
	}
}

impl GearSlots {
	const margin: f32 = 0.05;
	pub const width: f32 = GearType::width * 2.0 + GearSlots::margin;
}

impl SpawnToParent for GearSlots {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::ext::mutASS,
	) -> Entity {
		let mut parent = parent.spawn(PbrBundle {
			transform: Transform::from_translation(translation).translate(Vec3::Z * almost_zero),
			..default()
		});
		parent.name("GearSlots parent");

		/* #region second */
		const right_x: f32 = GearSlots::margin / 2. + GearType::width / 2.;
		parent.with_children(|parent| {
			self.second.spawn_using_entity_commands(parent, Vec3::X * right_x, (meshs, mat, ass));
		});
		/* #endregion */

		/* #region first */
		const left_z: f32 = -right_x;
		parent.with_children(|parent| {
			self.first.spawn_using_entity_commands(parent, Vec3::X * left_z, (meshs, mat, ass));
		});
		/* #endregion */

		parent.id()
	}
}
