use dioxus::prelude::*;

//
// ME
//
pub struct SocialLink {
	pub name: &'static str,
	pub url: &'static str,
	pub icon: Asset,
}

pub struct Me {
	pub name: &'static str,
	pub image: Asset,
	pub age: &'static str,
	pub email: &'static str,
	pub location: &'static str,
	pub langs: &'static [&'static str],
	pub scripting: &'static [&'static str],
	pub frameworks: &'static [&'static str],
	pub tools: &'static [&'static str],
	pub socials: &'static [SocialLink],
}

pub static ME: Me = Me {
	name: "James Plummer",
	image: asset!("assets/photo.webp"),
	age: "24",
	email: "jamesp2001@live.co.uk",
	location: "Wrocław, Poland",
	langs: &["Rust", "Typescript", "Svelte", "Go", "Lua"],
	scripting: &["Nushell", "Bash", "Python"],
	frameworks: &[
		"Dioxus (Rust)",
		"Poem (Rust)",
		"Chi (Go)",
		"Astro (Typescript)",
		"SvelteKit (Svelte)",
		"Next.js (Typescript)",
	],
	tools: &["Docker", "GitHub Actions", "K3s"],
	socials: &[
		SocialLink {
			name: "GitHub",
			url: "https://github.com/jamesukiyo",
			icon: asset!("/assets/github.svg"),
		},
		SocialLink {
			name: "X/Twitter",
			url: "https://twitter.com/jamesukiyo",
			icon: asset!("/assets/x.svg"),
		},
	],
};

//
// PROJECTS
//
pub enum ProjectType {
	Website,
	CliTool,
	Script,
	NeovimPlugin,
	Utility,
	Config,
}

impl ProjectType {
	pub const fn as_str(&self) -> &'static str {
		match self {
			Self::Website => "website",
			Self::CliTool => "CLI tool",
			Self::Script => "script",
			Self::NeovimPlugin => "neovim plugin",
			Self::Utility => "utility",
			Self::Config => "config",
		}
	}
}

impl std::fmt::Display for ProjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

pub struct ProjectInfo {
	pub name: &'static str,
	pub desc: &'static str,
	pub type_of: ProjectType,
	pub tech: &'static [&'static str],
	pub gh_url: Option<&'static str>,
	pub site_url: Option<&'static str>,
}

impl ProjectInfo {
	pub fn tech_str(&self) -> String {
		self.tech.join(", ")
	}

	pub fn type_str(&self) -> &'static str {
		self.type_of.as_str()
	}

	pub fn gh_url_str(&self) -> &'static str {
		self.gh_url.unwrap_or("N/A")
	}

	pub fn site_url_str(&self) -> &'static str {
		self.site_url.unwrap_or("N/A")
	}
}

pub static PROJECTS: &[ProjectInfo] = &[
	ProjectInfo {
		name: "dr-radka",
		desc: "",
		type_of: ProjectType::Website,
		tech: &["SvelteKit", "Typescript", "Tailwind", "Sanity CMS"],
		gh_url: None,
		site_url: Some("https://dr-radka.pl"),
	},
	ProjectInfo {
		name: "charfreq-rs",
		desc: "",
		type_of: ProjectType::CliTool,
		tech: &["Rust"],
		gh_url: Some("https://github.com/jamesukiyo/charfreq-rs"),
		site_url: None,
	},
	ProjectInfo {
		name: "shell-rs",
		desc: "",
		type_of: ProjectType::CliTool,
		tech: &["Rust"],
		gh_url: Some("https://github.com/jamesukiyo/shell-rs"),
		site_url: None,
	},
	ProjectInfo {
		name: "windows-setup",
		desc: "",
		type_of: ProjectType::Script,
		tech: &["Bash"],
		gh_url: Some("https://github.com/jamesukiyo/windows-setup"),
		site_url: None,
	},
	ProjectInfo {
		name: "search-this.nvim",
		desc: "",
		type_of: ProjectType::NeovimPlugin,
		tech: &["Lua"],
		gh_url: Some("https://github.com/jamesukiyo/search-this.nvim"),
		site_url: None,
	},
	ProjectInfo {
		name: "server-health-monitor",
		desc: "",
		type_of: ProjectType::Utility,
		tech: &["C++", "Python", "Bash"],
		gh_url: Some("https://github.com/jamesukiyo/server-health-monitor"),
		site_url: None,
	},
	ProjectInfo {
		name: "pausarr",
		desc: "",
		type_of: ProjectType::Script,
		tech: &["Bash"],
		gh_url: Some("https://github.com/jamesukiyo/pausarr"),
		site_url: None,
	},
	ProjectInfo {
		name: "nvim",
		desc: "",
		type_of: ProjectType::Config,
		tech: &["Lua"],
		gh_url: Some("https://github.com/jamesukiyo/nvim"),
		site_url: None,
	},
	ProjectInfo {
		name: "me",
		desc: "",
		type_of: ProjectType::Website,
		tech: &["Rust", "Dioxus", "Tailwind"],
		gh_url: Some("https://github.com/jamesukiyo/me"),
		site_url: Some("https://jamesukiyo.com"),
	},
];
