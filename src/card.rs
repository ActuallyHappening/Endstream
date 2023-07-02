use bevy::prelude::*;

use crate::{EntityCommandsExt, IntoAssetPath, ASS};

pub enum ControllerCard {
	Ssv93Ural,
}

impl IntoAssetPath for ControllerCard {
	fn get_asset_path(&self) -> String {
		match self {
			ControllerCard::Ssv93Ural => "cards/Controller - SSV-93 URAL.png",
		}
		.into()
	}
}

pub const CARD_WIDTH: f32 = 5.;
pub const CARD_HEIGHT: f32 = 7.;

struct CardVisual {
	bg: CardVisualBg,
}

enum CardVisualBg {
	Yellowish,
	Blackish,
}

pub enum TimeStream {
	S1800,
	S1900,
	S2000,
	S2100,
	S2200,
	S2300,
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

/// Constructs a card from component [CardVisual] parts and
/// returns entity id
fn construct_card_from_visual(
	visual: CardVisual,
	pos: Transform,
	commands: &mut Commands,
	(meshs, mat, ass): &mut ASS,
) -> Entity {
	// spawn background / parent mesh
	let mesh = meshs.add(shape::Box::new(CARD_WIDTH, 0.1, CARD_HEIGHT).into());
	let material = mat.add(StandardMaterial {
		base_color_texture: Some(ass.load(visual.bg.get_asset_path())),
		..default()
	});
	let parent = commands.spawn(PbrBundle {
		mesh,
		material,
		transform: pos,
		..default()
	});

	parent.id()
}

pub fn spawn_all_cards_debug(mut commands: Commands, mut ass: ASS) {
	spawn_card_cheating(&mut commands, &mut ass);
	construct_card_from_visual(
		CardVisual {
			bg: CardVisualBg::Blackish,
		},
		Transform::from_xyz(CARD_WIDTH + 2., 5.0, 0.),
		&mut commands,
		&mut ass,
	);
}

fn spawn_card_cheating(commands: &mut Commands, (meshs, mat, ass): &mut ASS) {
	let mesh = meshs.add(shape::Box::new(CARD_WIDTH, 0.1, CARD_HEIGHT).into());
	let material = mat.add(StandardMaterial {
		base_color_texture: Some(ass.load(ControllerCard::Ssv93Ural.get_asset_path())),
		..default()
	});
	commands.spawn(PbrBundle {
		mesh,
		material,
		transform: Transform::from_translation(Vec3::new(0., 5., 0.)),
		..default()
	});
}
