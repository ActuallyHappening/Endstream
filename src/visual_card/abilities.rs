use derive_more::{From, Into};

use anyhow::anyhow;

use super::*;

mod ability;
pub use ability::{Ability, AbilityForm};

#[derive(Debug, Clone, PartialEq, Eq, Into)]
pub struct Abilities(Vec<Ability>);

impl Abilities {
	pub fn new(abilities: Vec<Ability>) -> Result<Self, anyhow::Error> {
		if abilities.len() > 3 {
			return Err(anyhow!("Too many abilities! 3 at most"));
		}
		Ok(Self(abilities))
	}
}

impl Abilities {
	pub const height: f32 = 2.7;
}

impl SpawnToParent for Abilities {
	fn spawn_to_child_builder(
		&self,
		parent: &mut ChildBuilder<'_, '_, '_>,
		translation: Vec3,
		(meshs, mat, ass): crate::utils::mutASS,
	) -> Entity {
		let margins = (Abilities::height - self.0.len() as f32 * Ability::height) / (self.0.len() as f32 + 1.);

		let mut parent = parent.spawn(PbrBundle {
			transform: Transform::from_translation(translation),
			..default()
		});

		for (i, ability) in self.0.iter().enumerate() {
			let i = i as f32;
			let y_offset = (i + 1.) * -margins;
			let translation = (y_offset + Abilities::height / 2.) * Vec3::Y;
			ability.spawn_to_parent(&mut parent, translation, (meshs, mat, ass))
		}

		parent.name("Abilities (parent)").id()
	}
}
