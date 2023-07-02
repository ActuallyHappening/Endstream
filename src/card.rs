use bevy::prelude::*;

use crate::ASS;

pub enum Card {
	Controller(ControllerCard)
}

pub enum ControllerCard {
	HQ,
}

impl ControllerCard {
	pub fn get_asset_path(&self) -> &'static str {
		match self {
			ControllerCard::HQ => "assets/controllers/1-V4 - R - HQ 1 Ural 18 FINAL.png",
		}
	}
}

pub const CARD_WIDTH: f32 = 5.;
pub const CARD_HEIGHT: f32 = 7.;

pub fn spawn_card(commands: &mut Commands, (meshs, mat, ass): &mut ASS) {
	let mesh = meshs.add(shape::Box::new(CARD_WIDTH, 0.1, CARD_HEIGHT).into());
	let mut material: StandardMaterial = Color::GREEN.into();
	// material.base_color_texture = Some(ass.load(ControllerCard::HQ.get_asset_path()));
	let material = mat.add(material);
	commands.spawn(PbrBundle {
		mesh,
		material,
		transform: Transform::from_translation(Vec3::new(0., 5., 0.)),
		..default()
	});
}

pub fn spawn_all_cards_debug(mut commands: Commands, mut ass: ASS) {
	spawn_card(&mut commands, &mut ass);
}