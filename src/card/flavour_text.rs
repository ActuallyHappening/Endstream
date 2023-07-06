use std::borrow::Cow;

use bevy::prelude::*;

use crate::{
	ext::{EntityCommandsExt, SpawnToParent, TransformExt},
	textmesh::{get_text_mesh, Fonts},
};

use super::almost_zero;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlavourText {
	first: String,
	second: Option<String>,
}

impl FlavourText {
	pub fn new(str: impl Into<Cow<'static, str>>) -> Result<FlavourText, anyhow::Error> {
		let str = str.into();
		let str = str.trim().to_uppercase();

		const char_limit: usize = 60;
		// split string after X characters, and put second half in second
		let lines = textwrap::wrap(&str, char_limit);
		if lines.len() == 1 {
			Ok(FlavourText {
				first: lines[0].to_string(),
				second: None,
			})
		} else if lines.len() == 2 {
			Ok(FlavourText {
				first: lines[0].to_string(),
				second: Some(lines[1].to_string()),
			})
		} else {
			anyhow::bail!("Flavour text is too long. Max length is 2 lines, got {}: {lines:?}", lines.len());
		}
	}
}

impl FlavourText {
	const text_size: f32 = 0.25;
	const colour: Color = Color::rgb(0.8, 0.8, 0.8);

	pub const margin_from_bottom: f32 = 0.3;
	const first_y: f32 = 0.15 + 0.1;
	const second_y: f32 = -0.15 + 0.1;
}

impl SpawnToParent for FlavourText {
	fn spawn_using_entity_commands(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, _): crate::ext::mutASS,
	) -> Entity {
		let mut parent = parent.spawn(PbrBundle {
			transform: Transform::from_translation(translation).translate(Vec3::Z * almost_zero),
			..default()
		});
		parent.name("FlavourText parent");

		/* #region first text */
		let mat = mat.add(FlavourText::colour.into());
		parent.with_children(|parent| {
			let (mesh, offset) = get_text_mesh(self.first.clone(), FlavourText::text_size, Fonts::Light);
			parent
				.spawn(PbrBundle {
					mesh: meshs.add(mesh),
					transform: Transform::from_translation(offset)
						.translate(
							Vec3::Y * {
								if self.second.is_some() {
									FlavourText::first_y
								} else {
									0.0
								}
							},
						),
					material: mat.clone(),
					..default()
				})
				.name("FlavourText 1");
		});
		/* #endregion */

		/* #region second text */
		if let Some(second) = &self.second {
			parent.with_children(move |parent| {
				let (mesh, offset) = get_text_mesh(second.clone(), FlavourText::text_size, Fonts::Light);
				parent
					.spawn(PbrBundle {
						mesh: meshs.add(mesh),
						transform: Transform::from_translation(offset)
							.translate(Vec3::Y * FlavourText::second_y),
						material: mat,
						..default()
					})
					.name("FlavourText 2");
			});
		}

		parent.id()
	}
}
