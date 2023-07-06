use bevy::prelude::*;
use derive_more::Display;
use strum::EnumIs;

use crate::{
	card::{right_margin, CARD_WIDTH},
	ext::{EntityCommandsExt, IntoAssetPath, SpawnToParent, TransformExt},
	texture_2d,
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

struct GearSlotFrame;

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
	const width: f32 = 0.25;
	const height: f32 = 0.3;
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

impl IntoAssetPath for GearSlotFrame {
	fn get_asset_path(&self) -> String {
		"card-icons/gear-frame.png".to_string()
	}
}

impl GearSlotFrame {
	const icon_padding: f32 = 0.05;
	const width: f32 = GearType::width + GearSlotFrame::icon_padding * 2.;
	const height: f32 = GearType::height + GearSlotFrame::icon_padding * 2.;
}

impl SpawnToParent for GearSlotFrame {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::ext::mutASS,
	) -> Entity {
		let mut frame = parent.spawn(PbrBundle {
			transform: Transform::from_translation(translation),
			mesh: meshs
				.add(shape::Quad::new(Vec2::new(GearSlotFrame::width, GearSlotFrame::height)).into()),
			material: mat.add(texture_2d(ass.load(self.get_asset_path()))),
			..default()
		});
		frame.name("GearSlotFrame");
		frame.id()
	}
}

impl GearSlots {
	const margin: f32 = 0.15;
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
			let translation = Vec3::X * right_x;
			self
				.second
				.spawn_using_entity_commands(parent, translation, (meshs, mat, ass));
			GearSlotFrame.spawn_using_entity_commands(parent, translation, (meshs, mat, ass));
		});
		/* #endregion */

		/* #region first */
		const left_z: f32 = -right_x;
		parent.with_children(|parent| {
			let translation = Vec3::X * left_z;
			self
				.first
				.spawn_using_entity_commands(parent, translation, (meshs, mat, ass));
			GearSlotFrame.spawn_using_entity_commands(parent, translation, (meshs, mat, ass));
		});
		/* #endregion */

		parent.id()
	}
}
