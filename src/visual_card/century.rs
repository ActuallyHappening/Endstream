use strum::IntoStaticStr;
use bevy::prelude::*;

use crate::{utils::texture_2d, textmesh::{get_text_mesh, Fonts}, utils::{SpawnToParent, EntityCommandsExt, mutASS, TransformExt}, visual_card::{almost_zero}};

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
	pub fn get_asset_path() -> &'static str {
		"card-icons/century-icon.png"
	}
}

impl Century {
	const height: f32 = 0.4;
	const width: f32 = 0.8;
}

impl SpawnToParent for Century {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut bevy::prelude::ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): mutASS,
	) -> Entity {
		let century_shape = shape::Quad::new(Vec2::new(Century::width, Century::height));
		let mesh = meshs.add(century_shape.into());

		let material = mat.add(texture_2d(ass.load(Century::get_asset_path())));
		let mut century = parent.spawn(PbrBundle {
			mesh,
			material,
			transform: Transform::from_translation(translation),
			..default()
		});
		century.name("Century marker");

		// century text
		const century_text_size: f32 = 0.4;
		century.with_children(|century| {
			let (mesh, offset) = get_text_mesh(self.into_num().to_string(), century_text_size, Fonts::Light);
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

		century.id()
	}
}
