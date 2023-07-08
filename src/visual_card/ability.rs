use std::str::FromStr;

use anyhow::anyhow;
use colored::Colorize;
use derive_more::{From, Into};
use strum::EnumIs;
use textwrap::{wrap, Options, WordSeparator};

use super::*;

pub enum AbilityForm {
	Cost(AgendaCost),
	Passive,
}

impl AbilityForm {
	const passive_asset_path: &str = "assets/card-icons/passive-ability.png";
}

/// Word that should be rendered in different colour
#[derive(Default, EnumIs, PartialEq, Eq, Debug, Clone, Copy)]
enum HighlightedSpan {
	#[default]
	Normal,
	YellowCaps,
}

/// Un line-wrapped highlighted text to render
#[derive(From, Debug)]
struct UnwrappedAbilityText(Vec<(String, HighlightedSpan)>);

/// *Wrapped* ability text
#[derive(From, Debug, Clone, PartialEq, Eq)]
struct AbilityText(Vec<Vec<(String, HighlightedSpan)>>);

pub struct Ability {
	form: AbilityForm,
	text: AbilityText,
}

impl FromStr for HighlightedSpan {
	type Err = anyhow::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// split into words
		let mut words = UnwrappedAbilityText::split_text(s).into_iter();
		let first_word = words.next().ok_or_else(|| anyhow!("Empty string"))?;
		if words.next().is_some() {
			return Err(anyhow!("More than one word in HighlightedSpan"));
		}

		Ok(match first_word.to_uppercase().as_str() {
			"STRIKE" | "ARMOUR" | "IMMOBILIZED" | "IMMOBILIZE" | "IMPAIRED" | "UNTOUCHABLE"
			| "CONTROL" | "FRAIL" | "STREAM" | "RESTRICT" | "LOCKDOWN" | "CONTAMINATE" | "EQUIP"
			| "DEPLOY" | "DISABLE" | "MOVE" => HighlightedSpan::YellowCaps,
			_ => HighlightedSpan::Normal,
		})
	}
}

impl FromStr for UnwrappedAbilityText {
	type Err = anyhow::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		UnwrappedAbilityText::split_text(s)
			.into_iter()
			.map(|word| {
				let span = word.parse::<HighlightedSpan>()?;
				Ok((word.to_string(), span))
			})
			.collect::<Result<Vec<_>, _>>()
			.map(UnwrappedAbilityText)
	}
}

#[test]
fn ability_text_parses() {
	let t = "IMMOBILIZE an operator anywhere on the board.";
	let text = t.parse::<UnwrappedAbilityText>().unwrap();
	assert_eq!(text.0.len(), t.split_whitespace().count());
	assert_eq!(
		text.0[0],
		("IMMOBILIZE".to_string(), HighlightedSpan::YellowCaps)
	);
	assert_eq!(
		text.0.get(7 - 1).unwrap().clone(),
		("board.".to_string(), HighlightedSpan::Normal)
	);
}

impl std::fmt::Display for AbilityText {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for line in &self.0 {
			for (word, span) in line {
				let word = match span {
					HighlightedSpan::Normal => word.clone(),
					HighlightedSpan::YellowCaps => {
						let word: &str = word;
						word.to_uppercase().yellow().to_string()
					}
				};
				write!(f, "{} ", word)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl UnwrappedAbilityText {
	const word_seperator: WordSeparator = WordSeparator::new();
	const avg_char_width_pixels: f32 = 0.11;

	fn wrap_options(&self, width_in_pixels: f32) -> Options {
		let res = (width_in_pixels / UnwrappedAbilityText::avg_char_width_pixels);
		eprintln!("Calculated wrap width: {}", res);
		Options::new(res as usize)
			.break_words(false)
			.word_splitter(textwrap::WordSplitter::NoHyphenation)
			.word_separator(UnwrappedAbilityText::word_seperator)
	}

	fn split_text(text: &str) -> Vec<String> {
		UnwrappedAbilityText::word_seperator
			.find_words(text)
			.map(|s| s.to_string())
			.collect()
	}

	fn with_known_width(self, width_in_pixels: f32) -> AbilityText {
		let (mut full, types) = self.0.iter().fold(
			(String::with_capacity(0), Vec::new()),
			|(mut full, mut types), (str, span)| {
				full.push_str(&format!("{} ", str));
				types.push(*span);
				(full, types)
			},
		);
		full = full.trim_end().to_string();

		let lines = wrap(&full, self.wrap_options(width_in_pixels));

		let mut res: Vec<Vec<(String, HighlightedSpan)>> = Vec::new();
		let mut type_counter = 0;
		for line in lines {
			eprintln!("Parsing line: {}", line);

			let mut line_res = Vec::new();
			for word in UnwrappedAbilityText::split_text(&line) {
				let span = types[type_counter];
				type_counter += 1;
				line_res.push((word.to_string(), span));
			}
			res.push(line_res);
		}

		res.into()
	}
}

#[test]
fn test_wrapping() {
	let t = "When moving out of Tamerlan's turnpoint operators cannot MOVE cross-stream and can only MOVE one century up/down-stream";
	let text = t.parse::<UnwrappedAbilityText>().unwrap();

	eprintln!("{:#?}", text);

	let text = text.with_known_width(AbilityText::large_width);

	eprintln!("{}", text);
}

impl AbilityText {
	const text_size: f32 = 0.2;

	const large_width: f32 = 4.5;
	const small_width: f32 = 3.5;
}