mod constants;

use crate::constants::{NOT_INCLUDED, RUST_SVG};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;

const BASE_DIR: &str = "/Users/tieppo/Library/Application Support/Zed/extensions/installed/material-icon-theme";
const ICONS_DIR: &str = "/icons";
const ICON_REF_DIR: &str = "./icons/";
const JSON_FILE: &str = "/icon_themes/material-icon-theme.json";
const TEMP_DIR: &str = "/Users/tieppo/icon-gen";
const SVG_EXTENSION: &str = ".clone.svg";
const PREFFIXES: &[&str] = &["", ".", "_", "-"];
const IS_DEBUG: bool = false;

type DynKeyValue = HashMap<String, String>;
type DynPathValue = HashMap<String, PathValueProps>;
type DirNamedIconsValue = HashMap<String, DirIconProps>;

#[derive(Debug, Deserialize, Serialize)]
struct PathValueProps {
	path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DirIconProps {
	collapsed: String,
	expanded: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ThemeProps {
	name: String,
	appearance: String,
	file_icons: DynPathValue,
	directory_icons: DirIconProps,
	named_directory_icons: DirNamedIconsValue,
	file_suffixes: DynKeyValue,
	file_stems: DynKeyValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonProps {
	#[serde(rename = "$schema")]
	_schema: String,
	name: String,
	author: String,
	themes: Vec<ThemeProps>,
}

fn main() -> std::io::Result<()> {
	let destination = if IS_DEBUG { TEMP_DIR } else { BASE_DIR };
	let icon_dir = fs::read_dir(format!("{BASE_DIR}{ICONS_DIR}")).expect("[ERROR]: loading directory");
	let mut json_file = File::open(format!("{BASE_DIR}{JSON_FILE}")).expect("[ERROR]: loading file");
	let mut json_file_content_raw = String::new();
	let mut theme: JsonProps;

	json_file
		.read_to_string(&mut json_file_content_raw)
		.expect("[ERROR]: reading file icon content");

	theme = serde_json::from_str(&json_file_content_raw).expect("[ERROR]: parsing json to struct");

	for entry in icon_dir {
		let f = entry.unwrap();
		let f_name = f.file_name().into_string().unwrap();

		if f_name.contains("nest") && f_name != "nest.svg" {
			let mut file_content = String::new();

			File::open(format!("{BASE_DIR}{ICONS_DIR}/{}", &f_name))
				.expect("[ERROR]: opening svg")
				.read_to_string(&mut file_content)
				.expect("[ERROR]: reading svg");

			let color = file_content.split("fill=\"").last().unwrap().split_once("\"").unwrap().0;
			let extension: &str = f_name
				.split("-")
				.last()
				.expect("[ERROR]: extracting extension, first part")
				.split(".")
				.next()
				.expect("[ERROR]: spliting extension, last part");

			write_svg(&f_name, destination, color, &mut theme, extension);
		}
	}

	for f in NOT_INCLUDED.into_iter() {
		write_svg(
			&format!("rust-{}{SVG_EXTENSION}", f.file_desc_name),
			destination,
			f.color,
			&mut theme,
			f.file_desc_name,
		);
	}

	json_file_content_raw = format!(
		"{}\n",
		serde_json::to_string_pretty(&theme).expect("[ERROR]: parsing json to string")
	);
	fs::write(format!("{destination}{JSON_FILE}"), &json_file_content_raw).expect("[ERROR]: writing json file");

	Ok(())
}

fn write_svg(f_name: &str, destination: &str, color: &str, theme: &mut JsonProps, extension: &str) {
	let new_file_content = RUST_SVG.replace("{{replace_color}}", color);
	let value = format!("rust-{extension}");

	theme.themes[0].file_icons.insert(
		format!("{value}"),
		PathValueProps {
			path: format!("{ICON_REF_DIR}{value}{SVG_EXTENSION}"),
		},
	);

	for p in PREFFIXES {
		theme.themes[0].file_stems.insert(format!("{p}{extension}.rs"), value.clone());
		theme.themes[0].file_suffixes.insert(format!("{p}{extension}"), value.clone());
	}

	fs::write(
		format!("{destination}{ICONS_DIR}/{}", f_name.replace("nest", "rust")),
		new_file_content,
	)
	.expect("[ERROR]: writing new svg file");
}
