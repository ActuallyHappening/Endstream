#![allow(non_upper_case_globals)]

use crate::{EntityCommandsExt, IntoAssetPath, ASS};
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

struct CardVisual {
	bg: CardVisualBg,
	start_century: Option<Century>,
}

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

#[derive(IntoStaticStr)]
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

	trait TransformExt {
		fn translate(self, delta: Vec3) -> Self;
	}
	impl TransformExt for Transform {
		fn translate(mut self, delta: Vec3) -> Self {
			self.translation += delta;
			self
		}
	}

	// spawn background / parent mesh
	let mesh = meshs.add(shape.into());
	let material = mat.add(StandardMaterial {
		base_color_texture: Some(ass.load(visual.bg.get_asset_path())),
		base_color: visual.bg.colour_factor(),
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
		// spawn century icon

		if let Some(start_century) = visual.start_century {
			const century_height: f32 = 0.4;
			const century_width: f32 = 0.8;
			const century_y: f32 = 9.5;

			let century_shape = shape::Quad::new(Vec2::new(century_width, century_height));
			let mesh = meshs.add(century_shape.into());

			let transform = Transform::from_xyz(0., century_y / 2., almost_zero);
			let material = mat.add(StandardMaterial {
				base_color_texture: Some(ass.load(Century::get_asset_path())),
				alpha_mode: AlphaMode::Blend,
				unlit: true,
				..default()
			});
			let mut century = parent.spawn(PbrBundle {
				transform,
				mesh,
				material,
				..default()
			});
			century.name("Century marker");

			// century text
			century.with_children(|century| todo!());
			// end century text
		}
		// end spawn century
	});

	parent.id()
}

pub fn spawn_all_cards_debug(mut commands: Commands, mut ass: ASS) {
	spawn_card_cheating(&mut commands, &mut ass);
	construct_card_from_visual(
		CardVisual {
			bg: CardVisualBg::Blackish,
			start_century: Some(Century::S2100),
		},
		Transform::from_xyz(CARD_WIDTH + 2., 5.0, 0.),
		&mut commands,
		&mut ass,
	);

	// let font = ass.2.load("fonts/NotoSans-Regular.ttf");
	let font = ass.2.load("fonts/FireMono-Medium.ttf");
	let style = TextStyle {
		font,
		color: Color::WHITE,
		..default()
	};
	let text = Text::from_section("please work", style).with_alignment(TextAlignment::Center);
	commands.spawn(TextBundle { text, ..default() });
}

pub fn render_text(
	at: Transform,
	content: &str,
	font_file: &str,
	commands: &mut Commands,
	asset_server: ResMut<AssetServer>,
	materials: ResMut<Assets<Mesh>>,
	meshs: ResMut<Assets<Mesh>>,
) {
	// load font
	let font = asset_server.load(font_file);

	// create text
	let text_something_idk = todo!();

	commands.spawn(text_something_idk);
}


pub fn render_text(
	at: Transform,
	content: &str,
	font_file: &str,
	commands: &mut Commands,
	asset_server: ResMut<AssetServer>,
	materials: ResMut<Assets<Mesh>>,
	meshs: ResMut<Assets<Mesh>>,
) {
	// load font
	let font = asset_server.load(font_file);

	// create text
	let text_something_idk = todo!();

	commands.spawn(text_something_idk);
}

fn spawn_card_cheating(commands: &mut Commands, (meshs, mat, ass): &mut ASS) {
	let mesh = meshs.add(shape::Box::new(CARD_WIDTH, 0.1, CARD_HEIGHT).into());
	let material = mat.add(StandardMaterial {
		base_color_texture: Some(ass.load(ControllerCard::Ssv93Ural.get_asset_path())),
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
