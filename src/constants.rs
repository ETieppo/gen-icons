
pub struct NotIncProps {
	pub color: &'static str,
	pub file_desc_name: &'static str,
	pub associations: Option<&'static[&'static str]>,
}

pub const RUST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><path fill="{{replace_color}}" d="m30 12-4-2V6h-4l-2-4-4 2-4-2-2 4H6v4l-4 2 2 4-2 4 4 2v4h4l2 4 4-2 4 2 2-4h4v-4l4-2-2-4ZM6 16a9.9 9.9 0 0 1 .842-4H10v8H6.842A9.9 9.9 0 0 1 6 16m10 10a9.98 9.98 0 0 1-7.978-4H16v-2h-2v-2h4c.819.819.297 2.308 1.179 3.37a1.89 1.89 0 0 0 1.46.63h3.34A9.98 9.98 0 0 1 16 26m-2-12v-2h4a1 1 0 0 1 0 2Zm11.158 6H24a2.006 2.006 0 0 1-2-2 2 2 0 0 0-2-2 3 3 0 0 0 3-3q0-.08-.004-.161A3.115 3.115 0 0 0 19.83 10H8.022a9.986 9.986 0 0 1 17.136 10"/></svg>"##;

pub const NOT_INCLUDED: &[NotIncProps] = &[
	NotIncProps {
		file_desc_name: "router",
		color: "#88f975",
		associations: None,
	},
	NotIncProps {
		file_desc_name: "test",
		color: "#00fdee",
		associations: Some(&["tests"]),
	},
	NotIncProps {
		file_desc_name: "tests",
		color: "#00fdee",
		associations: None,
	},
	NotIncProps {
		file_desc_name: "mod",
		color: "#b60800",
		associations: None,
	},
	NotIncProps {
		file_desc_name: "lib",
		color: "#ff2222",
		associations: Some(&["main"]),
	},
];
