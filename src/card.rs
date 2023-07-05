#![allow(non_upper_case_globals)]

use crate::card::general_info::ControllerGeneralInfo;
use crate::ext::{EntityCommandsExt, IntoAssetPath, SpawnToParent, ASS};
use crate::{
	agendas::{AgendaCost, AgendaType, SingleAgendaType},
	textmesh::{get_text_mesh, get_text_mesh_with_bbox},
	texture_2d,
};
use bevy::prelude::*;
use std::num::NonZeroU8;

mod century;
use century::Century;

use self::general_info::{Gender, ClassRace, Class, Race};
mod agenda;
mod general_info;

pub enum ControllerCard {
	Ssv93Ural,
}

impl IntoAssetPath for ControllerCard {
	fn get_asset_path(&self) -> String {
		match self {
			// ControllerCard::Ssv93Ural => "cards/Controller - SSV-93 URAL.png",
			ControllerCard::Ssv93Ural => "cards/Operator - Mori.png",
		}
		.into()
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CardVisual {
	bg: CardVisualBg,
	start_century: Option<Century>,
	activation_cost: Option<AgendaCost>,
	/// Path to artwork image texture
	artwork: String,

	info: ControllerGeneralInfo,
}

impl CardVisual {
	pub const artwork_height: f32 = 5.1;
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum CardVisualBg {
	Yellowish,
	Blackish,
}

impl CardVisualBg {
	fn colour_factor(&self) -> Color {
		match self {
			CardVisualBg::Blackish => Color::WHITE * 0.58,
			CardVisualBg::Yellowish => Color::WHITE * 0.58,
			// CardVisualBg::Yellowish => Color::BLACK,
		}
	}
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

pub const CARD_WIDTH: f32 = 6.2;
pub const CARD_HEIGHT: f32 = 10.3;

/// Margin of space between card content and left side of card
const left_margin: f32 = 0.3;
const almost_zero: f32 = 0.01;

/// Constructs a card from component [CardVisual] parts and
/// returns entity id.
///
/// Implicitly constructs card on XZ plane, facing -Z direction (as in top of card is -Z direction)
fn construct_card_from_visual(
	visual: CardVisual,
	mut position: Transform,
	commands: &mut Commands,
	(meshs, mat, ass): &mut ASS,
) -> Entity {
	let shape_dimensions = Vec2::new(CARD_WIDTH, CARD_HEIGHT);
	let shape = shape::Quad::new(shape_dimensions);

	// spawn background / parent mesh
	let mesh = meshs.add(shape.into());
	let material = mat.add(StandardMaterial {
		base_color_texture: Some(ass.load(visual.bg.get_asset_path())),
		// base_color: visual.bg.colour_factor(),
		// alpha_mode: AlphaMode::Blend,
		unlit: true,
		..default()
	});

	position.rotate_x(-90f32.to_radians());
	let mut parent = commands.spawn(PbrBundle {
		mesh,
		material,
		transform: position,
		..default()
	});
	parent.name("Card (parent)");

	/* #region top row */
	/// Margin of top row from the top of the card
	/// (and margin between bottom of top row and top of artwork)
	const top_margin: f32 = 0.4;
	// const top_row_y: f32 = 9.5 / 2.;
	const top_row_y: f32 = CARD_HEIGHT / 2. - top_margin;

	parent.with_children(|parent| {
		parent
			.spawn(PbrBundle {
				transform: Transform::from_xyz(0., top_row_y, almost_zero),
				..default()
			})
			.name("Top row")
			.with_children(|parent| {
				// spawn century icon
				if let Some(start_century) = visual.start_century {
					start_century.spawn_using_entity_commands(parent, Vec3::ZERO, (meshs, mat, ass));
				}
				// end spawn century

				// spawn agenda cost
				if let Some(agenda_cost) = visual.activation_cost {
					agenda_cost.spawn_using_entity_commands(parent, Vec3::ZERO, (meshs, mat, ass));
				}
				// end agenda cost
			});
	});
	/* #endregion top row */

	/* #region artwork */
	const artwork_y: f32 = top_row_y - CardVisual::artwork_height / 2. - top_margin;
	parent.with_children(|parent| {
		parent
			.spawn(PbrBundle {
				material: mat.add(texture_2d(ass.load(visual.artwork))),
				mesh: meshs.add(shape::Quad::new(Vec2::new(CARD_WIDTH, CardVisual::artwork_height)).into()),
				transform: Transform::from_xyz(0., artwork_y, almost_zero),
				..default()
			})
			.name("Artwork");
	});
	/* #endregion artwork */

	// #region general info row

	const general_y: f32 = artwork_y - CardVisual::artwork_height / 2. - ControllerGeneralInfo::height / 2.;
	parent.with_children(|general_row| {
		let general = visual.info;
		general.spawn_using_entity_commands(general_row, Vec3::Y * general_y, (meshs, mat, ass));
	});
	// #endregion end general info

	parent.id()
}

pub fn spawn_all_cards_debug(mut commands: Commands, mut ass: ASS) {
	spawn_card_cheating(&mut commands, &mut ass);
	construct_card_from_visual(
		CardVisual {
			bg: CardVisualBg::Blackish,
			start_century: Some(Century::S2100),
			activation_cost: Some(AgendaCost::new_double(
				(
					2,
					(SingleAgendaType::Military, SingleAgendaType::Wild).into(),
				),
				(3, SingleAgendaType::Science.into()),
			)),
			// activation_cost: Some(AgendaCost::new_single(3, SingleAgendaType::Politics.into())),
			artwork: String::from("operators/2-V4 - R - 02 Mori 28 FINAL.png"),
			info: ControllerGeneralInfo {
				name: "Mori".to_string(),
				aka_name: Some("The Piercer".to_string()),
				gender: Gender::Male,
				race: ClassRace {
					class: Class::None,
					race: Race::Human,
				},
				health: NonZeroU8::new(1).unwrap(),
			},
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
		unlit: true,
		..default()
	});
	commands
		.spawn(PbrBundle {
			mesh,
			material,
			transform: Transform::from_translation(Vec3::new(0., 5., 0.)),
			..default()
		})
		.name("Cheating card");
}
