#![allow(non_upper_case_globals)]

use std::num::NonZeroU8;

use crate::ext::{EntityCommandsExt, IntoAssetPath, ASS, SpawnToParent};
use crate::{
	agendas::{AgendaCost, AgendaType, SingleAgendaType},
	textmesh::{get_text_mesh, get_text_mesh_with_bbox},
	texture_2d,
};
use bevy::prelude::*;
use strum::IntoStaticStr;

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

	info: GeneralInfo,
}

impl CardVisual {
	pub const artwork_height: f32 = 5.1;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneralInfo {
	name: String,
	aka_name: Option<String>,
	gender: Gender,
	race: Race,

	health: NonZeroU8,
}

/// Gender, including `Gender::Neither`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gender {
	Male,
	Female,

	Neither,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Race {
	Human,
	PostHuman,
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

trait TransformExt {
	fn translate(self, delta: Vec3) -> Self;
}
impl TransformExt for Transform {
	fn translate(mut self, delta: Vec3) -> Self {
		self.translation += delta;
		self
	}
}

mod century;
use century::Century;

pub const CARD_WIDTH: f32 = 6.2;
pub const CARD_HEIGHT: f32 = 10.3;

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

	const left_margin: f32 = 0.3;

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
					start_century.spawn_using_entity_commands(parent, (meshs, mat, ass));
				}
				// end spawn century

				// spawn agenda cost
				if let Some(agenda_cost) = visual.activation_cost {
					let x = -(CARD_WIDTH / 2.0) + agenda_cost.width() / 2.0 + left_margin;
					let transform = Transform::from_translation(x * Vec3::X);
					parent
						.spawn(PbrBundle {
							transform,
							..default()
						})
						.name("Activation cost parent")
						.with_children(|activation_cost| {
							// cost frame
							let cost_frame_shape =
								shape::Quad::new(Vec2::new(agenda_cost.width(), AgendaCost::height));
							let material = texture_2d(ass.load(agenda_cost.get_frame_asset_path()));
							let mut cost_frame = activation_cost.spawn(PbrBundle {
								mesh: meshs.add(cost_frame_shape.into()),
								material: mat.add(material),
								..default()
							});
							cost_frame.name("Cost frame");
							// end cost frame

							// agenda icons & numbers
							cost_frame.with_children(|cost_frame| {
								let mesh = shape::Quad::new(Vec2::new(AgendaType::width, AgendaType::height));
								let mesh = meshs.add(mesh.into());
								const number_size: f32 = 0.2;

								match &agenda_cost {
									AgendaCost::One { only } => {
										// icon
										let material = texture_2d(ass.load(only.agenda.get_icon_asset_path()));
										// -0.05 is a slight offset, magic number
										let transform = Transform::from_xyz(0.0 - 0.05, -0.05, almost_zero);

										cost_frame
											.spawn(PbrBundle {
												transform,
												material: mat.add(material),
												mesh,
												..default()
											})
											.name("Only agenda");

										// number
										let (mesh, offset) = get_text_mesh(&only.number.to_string(), number_size);
										let top_right_transform = Transform::from_xyz(
											cost_frame_shape.size.x / 2. - number_size / 2.,
											cost_frame_shape.size.y / 2. - number_size / 2.,
											almost_zero,
										);
										cost_frame
											.spawn(PbrBundle {
												transform: top_right_transform.translate(offset),
												mesh: meshs.add(mesh),
												material: mat.add(Color::WHITE.into()),
												..default()
											})
											.name("Number for only agenda");
									}
									AgendaCost::Two { first, second } => {
										// first
										let material = texture_2d(ass.load(first.agenda.get_icon_asset_path()));
										// -0.05 is a slight offset, magic number
										let transform =
											Transform::from_xyz(-agenda_cost.width() / 4. - 0.05, -0.05, almost_zero);

										cost_frame
											.spawn(PbrBundle {
												transform,
												material: mat.add(material),
												mesh: mesh.clone(),
												..default()
											})
											.name("First agenda");

										// first number
										let (t_mesh, offset) = get_text_mesh(&second.number.to_string(), number_size);
										let top_right_transform = Transform::from_xyz(
											-number_size / 2.,
											cost_frame_shape.size.y / 2. - number_size / 2.,
											almost_zero,
										);
										cost_frame
											.spawn(PbrBundle {
												transform: top_right_transform.translate(offset),
												mesh: meshs.add(t_mesh),
												material: mat.add(Color::WHITE.into()),
												..default()
											})
											.name("Number for only agenda");

										// second
										let material = texture_2d(ass.load(second.agenda.get_icon_asset_path()));
										// -0.05 is a slight offset, magic number
										let transform =
											Transform::from_xyz(agenda_cost.width() / 4. - 0.05, 0.0 - 0.05, almost_zero);

										cost_frame
											.spawn(PbrBundle {
												transform,
												material: mat.add(material),
												mesh,
												..default()
											})
											.name("Second agenda");

										// second number
										let (mesh, offset) = get_text_mesh(&second.number.to_string(), number_size);
										let top_right_transform = Transform::from_xyz(
											cost_frame_shape.size.x / 2. - number_size / 2.,
											cost_frame_shape.size.y / 2. - number_size / 2.,
											almost_zero,
										);
										cost_frame
											.spawn(PbrBundle {
												transform: top_right_transform.translate(offset),
												mesh: meshs.add(mesh),
												material: mat.add(Color::WHITE.into()),
												..default()
											})
											.name("Number for only agenda");
									}
								}
							});
							// end icons & numbers
						});
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
	const general_height: f32 = 0.98;
	const general_y: f32 = artwork_y - CardVisual::artwork_height / 2. - general_height / 2.;
	parent.with_children(|general_row| {
		let general = visual.info;
		const general_text_size: f32 = 0.3;
		let name_mesh = get_text_mesh_with_bbox(&general.name, general_text_size);
		// #region names
		// general_row.spawn(PbrBundle {
		// 	transform: Transform::from_xyz(-CARD_WIDTH / 2. + left_margin),
		// 	..default()
		// }).name("Names row");
		// #endregion names
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
			info: GeneralInfo {
				name: "Mori".to_string(),
				aka_name: Some("The Piercer".to_string()),
				gender: Gender::Male,
				race: Race::Human,
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
