#![allow(non_upper_case_globals)]

use crate::agendas::{AgendaCost, SingleAgendaType};
use crate::card::general_info::ControllerGeneralInfo;
use crate::utils::{texture_2d, EntityCommandsExt, IntoAssetPath, SpawnToParent, ASS};
use bevy::prelude::*;
use std::num::NonZeroU8;

mod century;
use century::Century;

use self::back::Back;
use self::background::CardVisualBg;
use self::flavour_text::FlavourText;
use self::gear_slots::{GearSlots, GearType};
use self::general_info::{Class, ClassRace, Gender, Health, Race};
mod agenda_cost;
mod back;
mod background;
mod flavour_text;
mod gear_slots;
mod general_info;

/// Follows style of card and contains all the information necessary to draw any type of card to the screen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardVisual {
	bg: CardVisualBg,
	back: Back,

	start_century: Option<Century>,
	activation_cost: Option<AgendaCost>,
	gear_slots: GearSlots,

	/// Path to artwork image texture
	artwork: String,

	info: ControllerGeneralInfo,

	flavour_text: FlavourText,
}

impl CardVisual {
	pub const artwork_height: f32 = 5.1;

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

				/* #region gear slots */
				const gear_slots_x: f32 = CARD_WIDTH / 2. - right_margin - GearSlots::width / 2.;
				visual.gear_slots.spawn_using_entity_commands(
					parent,
					Vec3::X * gear_slots_x,
					(meshs, mat, ass),
				);
				/* #endregion */
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

	/* #region general info */
	const general_y: f32 =
		artwork_y - CardVisual::artwork_height / 2. - ControllerGeneralInfo::height / 2.;
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
	let debug_card = CardVisual {
		bg: CardVisualBg::Blackish,
		back: Back::Dotty,
		start_century: Some(Century::S2100),
		activation_cost: Some(AgendaCost::new_double(
			(
				2,
				(SingleAgendaType::Military, SingleAgendaType::Wild).into(),
			),
			(3, SingleAgendaType::Science.into()),
		)),
		gear_slots: GearSlots::new(GearType::Weapon, GearType::Relic),

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
