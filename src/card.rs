#![allow(non_upper_case_globals)]

use crate::{
	agendas::{AgendaCost, AgendaType, SingleAgendaType},
	texture_2d, EntityCommandsExt, IntoAssetPath, ASS,
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

#[derive(IntoStaticStr, Debug, Clone, PartialEq, Eq)]
pub enum Century {
	S1800,
	S1900,
	S2000,
	S2100,
	S2200,
	S2300,
}

impl Century {
	pub fn into_num(&self) -> u16 {
		match self {
			Century::S1800 => 1800,
			Century::S1900 => 1900,
			Century::S2000 => 2000,
			Century::S2100 => 2100,
			Century::S2200 => 2200,
			Century::S2300 => 2300,
		}
	}
}

impl Century {
	fn get_asset_path() -> &'static str {
		"card-icons/century-icon.png"
	}
}

pub const CARD_WIDTH: f32 = 6.2;
pub const CARD_HEIGHT: f32 = 10.3;

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
	const almost_zero: f32 = 0.01;

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
	parent.name("Card good (parent)");

	parent.with_children(|parent| {
		const top_row_y: f32 = 9.5 / 2.;

		// spawn century icon
		if let Some(start_century) = visual.start_century {
			const century_height: f32 = 0.4;
			const century_width: f32 = 0.8;

			let century_shape = shape::Quad::new(Vec2::new(century_width, century_height));
			let mesh = meshs.add(century_shape.into());

			let century_transform = Transform::from_xyz(0., top_row_y, almost_zero);
			let material = mat.add(StandardMaterial {
				base_color_texture: Some(ass.load(Century::get_asset_path())),
				alpha_mode: AlphaMode::Blend,
				unlit: true,
				..default()
			});
			let mut century = parent.spawn(PbrBundle {
				transform: century_transform,
				mesh,
				material,
				..default()
			});
			century.name("Century marker");

			// century text
			const century_text_size: f32 = 0.4;
			century.with_children(|century| {
				let (mesh, offset) =
					get_text_mesh(&start_century.into_num().to_string(), century_text_size);
				century
					.spawn(PbrBundle {
						transform: Transform::from_translation(offset).translate(Vec3::Z * almost_zero),
						mesh: meshs.add(mesh),
						material: mat.add(Color::WHITE.into()),
						..default()
					})
					.name("Century text");
			});
			// end century text
		}
		// end spawn century

		// spawn agenda cost
		if let Some(agenda_cost) = visual.activation_cost {
			let x = -(CARD_WIDTH / 2.0) + agenda_cost.width() / 2.0 + AgendaCost::left_margin;
			let transform = Transform::from_xyz(x, top_row_y, almost_zero);
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
					let material = StandardMaterial {
						base_color_texture: Some(ass.load(agenda_cost.get_frame_asset_path())),
						alpha_mode: AlphaMode::Blend,
						unlit: true,
						..default()
					};
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
									- number_size / 2.,
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
		},
		Transform::from_xyz(CARD_WIDTH + 2., 5.0, 0.),
		&mut commands,
		&mut ass,
	);
}

/// Returns mesh + offset (to ensure coordinates start in center of text)
fn get_text_mesh(text: &str, pixel_size: f32) -> (Mesh, Vec3) {
	use meshtext::{MeshGenerator, MeshText, TextSection};
	let font_data = include_bytes!(concat!(
		env!("CARGO_MANIFEST_DIR"),
		"/assets/fonts/Oswald-Regular.ttf"
	));
	let mut generator = MeshGenerator::new(font_data);
	let transform = Mat4::from_scale(Vec3::new(pixel_size, pixel_size, 0.)).to_cols_array();
	let text_mesh: MeshText = generator
		.generate_section(text, true, Some(&transform))
		.unwrap();

	let vertices = text_mesh.vertices;
	let positions: Vec<[f32; 3]> = vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
	let uvs = vec![[0f32, 0f32]; positions.len()];

	let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
	mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
	mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
	mesh.compute_flat_normals();

	(
		mesh,
		Vec3::X * (text_mesh.bbox.size().x / -2.) + Vec3::Y * (text_mesh.bbox.size().y / -2.),
	)
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
