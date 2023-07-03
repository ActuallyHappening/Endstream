use crate::IntoAssetPath;
use std::num::NonZeroU8;
use strum::{EnumIs, EnumIter, EnumString, IntoStaticStr};

#[derive(IntoStaticStr, EnumIter, EnumString, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SingleAgendaType {
	Military,
	Science,
	Politics,
	Wild,
}

/// Represents agenda symbol/s for a single cost rectangle
/// e.g.
/// ```rust
/// AgendaType::Single(SingleAgendaType::Wild)
/// ```
#[derive(Debug, Clone, EnumIs)]
pub enum AgendaType {
	Single(SingleAgendaType),
	Double(SingleAgendaType, SingleAgendaType),
}

/// Represents the semi-rectangular agenda cost icons, including possibility of two distinct rectangles.
/// e.g.
/// ```rust
/// AgendaCost::One {
///   only: AgendaCostItem {
///     number: NonZeroU8::new(2).unwrap(),
///     agenda_type: AgendaType::Single(SingleAgendaType::Wild),
///   },
/// }
/// ```
///
/// The second can't be a double agenda if the first is a double agenda
#[derive(Debug, Clone)]
pub enum AgendaCost {
	One {
		only: AgendaCostItem,
	},
	Two {
		first: AgendaCostItem,
		second: AgendaCostItem,
	},
}

/// Represents single rectangle (cost) icon,
/// potentially with a double agenda requirement
/// e.g.
/// ```rust
/// AgendaCostItem {
/// 	number: NonZeroU8::new(2).unwrap(),
/// 	agenda_type: AgendaType::Single(SingleAgendaType::Wild),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AgendaCostItem {
	number: NonZeroU8,
	agenda_type: AgendaType,
}

// ** implementation blocks **

// * agendas *

// * single agenda *
impl SingleAgendaType {
	fn get_abbreviation(&self) -> &'static str {
		match self {
			SingleAgendaType::Military => "m",
			SingleAgendaType::Science => "s",
			SingleAgendaType::Politics => "p",
			SingleAgendaType::Wild => "w",
		}
	}
}

// * agenda type *
impl IntoIterator for AgendaType {
	type Item = SingleAgendaType;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			AgendaType::Single(single) => vec![single].into_iter(),
			AgendaType::Double(first, second) => vec![first, second].into_iter(),
		}
	}
}

impl AgendaType {
	/// ```rust
	/// use endstream::agendas::{AgendaType, SingleAgendaType};
	/// let agenda_type = AgendaType::Double(SingleAgendaType::Military, SingleAgendaType::Science);
	/// assert_eq!(agenda_type.get_abbreviations(), "ms");
	/// ```
	pub fn get_abbreviations(&self) -> String {
		self
			.clone()
			.into_iter()
			.map(|single| single.get_abbreviation())
			.collect()
	}
}

impl AgendaType {
	pub fn get_icon_asset_path(&self) -> String {
		format!("agenda-icons/raw-icons/{}.png", self.get_abbreviations())
	}
}

// * costs *

impl IntoIterator for AgendaCost {
	type Item = AgendaCostItem;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			AgendaCost::One { only } => vec![only].into_iter(),
			AgendaCost::Two { first, second } => vec![first, second].into_iter(),
		}
	}
}

impl AgendaCost {
	/// Gets the (path to) frame, on-top of which a number and agenda icon/s are placed
	pub fn get_frame_asset_path(&self) -> String {
		match self {
			AgendaCost::One { only: AgendaCostItem { agenda_type, .. } } => {
				match agenda_type {
					AgendaType::Single(..) => {
						"agenda-icons/cost-frames/1uno.png"
					},
					AgendaType::Double(..) => {
						"agenda-icons/cost-frames/1duo.png"
					},
				}
			},
			AgendaCost::Two { first: AgendaCostItem { agenda_type: type1, .. }, second: AgendaCostItem { agenda_type: type2, .. } } => {
				match (type1, type2) {
					(AgendaType::Single(..), AgendaType::Single(..)) => {
						"agenda-icons/cost-frames/1uno2uno.png"
					},
					(AgendaType::Double(..), AgendaType::Single(..)) => {
						"agenda-icons/cost-frames/1duo2uno.png"
					},
					(AgendaType::Single(..), AgendaType::Double(..)) => {
						"agenda-icons/cost-frames/1uno2duo.png"
					},
					(AgendaType::Double(..), AgendaType::Double(..)) => {
						"agenda-icons/cost-frames/1duo2duo.png"
					},
				}
			}
		}.to_string()
	}
}