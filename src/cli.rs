use std::fmt::Display;

use clap::{Args, Parser, Subcommand};
use derive_more::{From, FromStr, Into};
use figment::{
  map,
  value::{Dict, Map, Value},
  Error, Metadata, Profile, Provider,
};

use crate::{config::LinkType, FILE_EXTENSION, PROJECT_DIRS};

#[derive(From, Debug, FromStr, Into)]
pub struct PathBuf(pub std::path::PathBuf);

impl Display for PathBuf {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.display())
  }
}

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Cli {
  #[clap(long, short)]
  /// Overwrites the dotfiles path set in the config file
  ///
  /// If no dotfiles path is provided in the config file the default "~/.dotfiles" is used
  pub dotfiles: Option<PathBuf>,
  #[clap(long, short, default_value_t = PROJECT_DIRS.config_dir().join(format!("config.{FILE_EXTENSION}")).into())]
  /// Path to the config file
  pub config: PathBuf,

  #[clap(subcommand)]
  pub command: Command,
}

#[derive(Debug, Args)]
pub struct Dots {
  #[clap(default_value = "\"*\"")]
  /// All dots to link
  pub dots: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// Clones a dotfiles git repository
  Clone {
    /// The url of the repository passed to the git clone command
    repo: Option<String>,
  },

  /// Links dotfiles to the filesystem
  Link {
    #[clap(flatten)]
    dots: Dots,

    #[clap(long, short, arg_enum)]
    /// Which link type to use for linking dotfiles
    link_type: Option<LinkType>,

    #[clap(long, short)]
    /// Force link creation if file already exists
    force: bool,
  },

  /// Syncs dotfiles with the git repository
  Sync {
    #[clap(flatten)]
    dots: Dots,
  },
}

impl Provider for Cli {
  fn metadata(&self) -> Metadata {
    Metadata::named("Cli")
  }

  fn data(&self) -> Result<Map<Profile, Dict>, Error> {
    let mut dict = Dict::new();

    if let Some(dotfiles) = &self.dotfiles {
      dict.insert("dotfiles".to_string(), Value::serialize(dotfiles.to_string()).unwrap());
    }

    if let Command::Clone { repo: Some(repo) } = &self.command {
      dict.insert("repo".to_string(), Value::serialize(repo).unwrap());
    }

    if let Command::Link {
      link_type: Some(link_type),
      dots: _,
      force: _,
    } = &self.command
    {
      dict.insert("link_type".to_string(), Value::serialize(link_type).unwrap());
    }

    Ok(map! {
      Profile::Global => dict
    })
  }
}

// dotfiles: cli.dotfiles.map(|d| d.into()),
// repo: if let Command::Clone { repo } = &cli.command { repo.clone() } else { None },
// link_type: if let Command::Link { link_type, dots: _ } = &cli.command { Some(link_type.clone()) } else { None },