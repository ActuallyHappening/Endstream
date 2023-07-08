#![allow(non_upper_case_globals)]

use crate::agendas::{AgendaCost, SingleAgendaType};
use crate::utils::{texture_2d, EntityCommandsExt, IntoAssetPath, SpawnToParent, ASS};
use crate::visual_card::general_info::ControllerGeneralInfo;
use bevy::prelude::*;
use std::num::NonZeroU8;

mod century;
use century::Century;

use self::artwork::Artwork;
use self::back::Back;
use self::background::CardVisualBg;
use self::flavour_text::FlavourText;
use self::gear_slots::{GearSlots, GearType};
use self::general_info::{Class, ClassRace, Gender, Health, Race};
use self::top_row::TopRow;
mod agenda_cost;
mod artwork;
mod back;
mod background;
mod flavour_text;
mod gear_slots;
mod general_info;
mod top_row;

/// Follows style of card and contains all the information necessary to draw any type of card to the screen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardVisual {
	bg: CardVisualBg,
	back: Back,

	top_row: TopRow,

	artwork: Artwork,

	info: ControllerGeneralInfo,

	flavour_text: FlavourText,
}

impl CardVisual {
	const dimensions: Vec2 = Vec2::new(CARD_WIDTH, CARD_HEIGHT);
}

pub const CARD_WIDTH: f32 = 6.2;
pub const CARD_HEIGHT: f32 = 10.3;

/// Margin of space between card content and left side of card
const left_margin: f32 = 0.3;
/// Margin of space between card content and right side of card
const right_margin: f32 = left_margin;
/// To avoid Z-fighting (some depth_bias is still necessary)
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
	position.rotate_x(-90f32.to_radians());
	let mut parent = commands.spawn(PbrBundle {
		transform: position,
		..default()
	});
	parent.name("Card (parent)");

	/* #region front background */
	parent.with_children(|parent| {
		visual
			.bg
			.spawn_using_entity_commands(parent, Vec3::ZERO, (meshs, mat, ass));
	});
	/* #endregion */

	/* #region back */
	parent.with_children(|parent| {
		visual
			.back
			.spawn_using_entity_commands(parent, Vec3::Z * -almost_zero, (meshs, mat, ass));
	});
	/* #endregion */

	/* #region top row */

	const top_margin: f32 = 0.4;
	const top_row_y: f32 = CARD_HEIGHT / 2. - top_margin;
	const top_row_pos: Vec3 = Vec3::new(0., top_row_y, almost_zero);

	parent.with_children(|parent| {
		visual
			.top_row
			.spawn_using_entity_commands(parent, top_row_pos, (meshs, mat, ass));
	});
	/* #endregion top row */

	/* #region artwork */
	const artwork_y: f32 = top_row_y - Artwork::height / 2. - top_margin;
	parent.with_children(|parent| {
		visual.artwork.spawn_using_entity_commands(
			parent,
			artwork_y * Vec3::Y + almost_zero * Vec3::Z,
			(meshs, mat, ass),
		);
	});
	/* #endregion artwork */

	/* #region general info */
	const general_y: f32 =
		artwork_y - Artwork::height / 2. - ControllerGeneralInfo::height / 2.;
	parent.with_children(|general_row| {
		let general = visual.info;
		general.spawn_using_entity_commands(general_row, Vec3::Y * general_y, (meshs, mat, ass));
	});
	/* #endregion general info */

	/* #region abilities */

	/* #endregion abilities */

	/* #region flavour text */
	const flavour_y: f32 = CARD_HEIGHT / -2. + FlavourText::margin_from_bottom;
	parent.with_children(|flavour_text| {
		visual.flavour_text.spawn_using_entity_commands(
			flavour_text,
			Vec3::Y * flavour_y,
			(meshs, mat, ass),
		);
	});
	/* #endregion flavour text */

	parent.id()
}

pub fn spawn_all_cards_debug(mut commands: Commands, mut ass: ASS) {
	// spawn_card_cheating(&mut commands, &mut ass);
	let debug_card: CardVisual = CardVisual {
		bg: CardVisualBg::Blackish,
		back: Back::Dotty,
		top_row: TopRow {
			start_century: Some(Century::S2100),
			activation_cost: Some(AgendaCost::new_double(
				(
					2,
					(SingleAgendaType::Military, SingleAgendaType::Wild).into(),
				),
				(3, SingleAgendaType::Science.into()),
			)),
			gear_slots: GearSlots::new(GearType::Weapon, GearType::Relic),
		},

		// activation_cost: Some(AgendaCost::new_single(3, SingleAgendaType::Politics.into())),
		artwork: Artwork::from("operators/2-V4 - R - 02 Mori 28 FINAL.png"),
		info: ControllerGeneralInfo {
			name: "Mori".to_string(),
			aka_name: Some("The Piercer".to_string()),
			gender: Gender::Male,
			race: ClassRace {
				class: Class::None,
				race: Race::Human,
			},
			health: Health::new(NonZeroU8::new(1)),
		},
		flavour_text: FlavourText::new("I live by a code. The code says nothing about time travel.")
			.unwrap(),
	};

	construct_card_from_visual(
		debug_card.clone(),
		Transform::from_xyz(CARD_WIDTH + 2., 5.0, 0.),
		&mut commands,
		&mut ass,
	);
	construct_card_from_visual(
		debug_card,
		Transform::from_xyz(0., 5., 0.).with_rotation(Quat::from_rotation_y(160f32.to_radians())),
		&mut commands,
		&mut ass,
	);
}

#[allow(dead_code)]
fn spawn_card_cheating(commands: &mut Commands, (meshs, mat, ass): &mut ASS) {
	enum ControllerCard {
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
