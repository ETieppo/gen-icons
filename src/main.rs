mod constants;

use crate::constants::{NOT_INCLUDED, RUST_SVG};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env::home_dir;
use std::fs;
use std::fs::File;
use std::io::Read;

#[cfg(target_os = "macos")]
const BASE_DIR: &str = "/Library/Application Support/Zed/extensions/installed/material-icon-theme";
#[cfg(target_os = "linux")]
const BASE_DIR: &str = "/.local/share/zed/extensions/installed/material-icon-theme";

const ICONS_DIR: &str = "/icons";
const ICON_REF_DIR: &str = "./icons/";
const JSON_FILE: &str = "/icon_themes/material-icon-theme.json";
const TEMP_DIR: &str = "/gen-icons";
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
	let base_dir_to_write = if IS_DEBUG { TEMP_DIR } else { BASE_DIR };
	let home = home_dir().and_then(|p| p.into_os_string().into_string().ok()).unwrap();
	let destination = home.clone() + base_dir_to_write;
	let icon_dir = fs::read_dir(format!("{home}{BASE_DIR}{ICONS_DIR}",)).expect("[ERROR]: loading directory");
	let mut json_file = File::open(format!("{home}{BASE_DIR}{JSON_FILE}")).expect("[ERROR]: loading file");
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

			File::open(format!("{home}{BASE_DIR}{ICONS_DIR}/{}", &f_name))
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

			write_svg(&f_name, &destination, color, &mut theme, extension, None);
		}
	}

	for f in NOT_INCLUDED.iter() {
		write_svg(
			&format!("rust-{}{SVG_EXTENSION}", f.file_desc_name),
			&destination,
			f.color,
			&mut theme,
			f.file_desc_name,
			f.associations,
		);
	}

	json_file_content_raw = format!(
		"{}\n",
		serde_json::to_string_pretty(&theme).expect("[ERROR]: parsing json to string")
	);

	fs::write(format!("{destination}{JSON_FILE}"), &json_file_content_raw).expect("[ERROR]: writing json file");
	Ok(())
}

fn write_svg(
	f_name: &str,
	destination: &String,
	color: &str,
	theme: &mut JsonProps,
	extension: &str,
	associations: Option<&'static [&'static str]>,
) {
	let new_file_content = RUST_SVG.replace("{{replace_color}}", color);
	let value = format!("rust-{extension}");
	let mut file_associations_to_register: HashSet<&str> = associations.unwrap_or(&[]).iter().copied().collect();
	file_associations_to_register.insert(extension);

	theme.themes[0].file_icons.insert(
		value.clone(),
		PathValueProps {
			path: format!("{ICON_REF_DIR}{value}{SVG_EXTENSION}"),
		},
	);

	for e in file_associations_to_register {
		for p in PREFFIXES {
			theme.themes[0].file_stems.insert(format!("{p}{e}.rs"), value.clone());
			theme.themes[0].file_suffixes.insert(format!("{p}{e}"), value.clone());
		}
	}

	fs::write(
		format!("{destination}{ICONS_DIR}/{}", f_name.replace("nest", "rust")),
		new_file_content,
	)
	.expect("[ERROR]: writing new svg file");
}
