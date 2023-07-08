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
#[derive(Debug, Clone, EnumIs, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgendaCostItem {
	pub agenda: AgendaType,
	pub number: NonZeroU8,
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
			SingleAgendaType::Wild => "x",
		}
	}
}

impl From<SingleAgendaType> for AgendaType {
	fn from(single: SingleAgendaType) -> Self {
		AgendaType::Single(single)
	}
}

impl From <(SingleAgendaType, SingleAgendaType)> for AgendaType {
	fn from((first, second): (SingleAgendaType, SingleAgendaType)) -> Self {
		AgendaType::Double(first, second)
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
	/// Width of single cost rectangle
	pub const width: f32 = 0.8;
	pub const height: f32 = 0.35;

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

	pub fn new_single(single: SingleAgendaType) -> Self {
		AgendaType::Single(single)
	}

	pub fn new_double(first: SingleAgendaType, second: SingleAgendaType) -> Self {
		AgendaType::Double(first, second)
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
	pub const height: f32 = 0.5;

	pub const fn width(&self) -> f32 {
		match self {
			AgendaCost::One { .. } => 0.95,
			AgendaCost::Two { .. } => 1.85,
		}
	}

	/// Gets the (path to) frame, on-top of which a number and agenda icon/s are placed
	pub fn get_frame_asset_path(&self) -> String {
		match self {
			AgendaCost::One { only: AgendaCostItem { agenda: agenda_type, .. } } => {
				match agenda_type {
					AgendaType::Single(..) => {
						"agenda-icons/cost-frames/1uno2none.png"
					},
					AgendaType::Double(..) => {
						"agenda-icons/cost-frames/1duo2none.png"
					},
				}
			},
			AgendaCost::Two { first: AgendaCostItem { agenda: type1, .. }, second: AgendaCostItem { agenda: type2, .. } } => {
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

	pub fn new_single(number: u8, agenda_type: AgendaType) -> Self {
		AgendaCost::One {
			only: AgendaCostItem {
				number: NonZeroU8::new(number).unwrap(),
				agenda: agenda_type,
			},
		}
	}

	pub fn new_double((first_number, first_agenda_type): (u8, AgendaType), (second_number, second_agenda_type): (u8, AgendaType)) -> Self {
		AgendaCost::Two {
			first: AgendaCostItem {
				number: NonZeroU8::new(first_number).unwrap(),
				agenda: first_agenda_type,
			},
			second: AgendaCostItem {
				number: NonZeroU8::new(second_number).unwrap(),
				agenda: second_agenda_type,
			},
		}
	}
}