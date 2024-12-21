use std::{borrow::Cow, collections::HashMap, os::unix::process::CommandExt, path::PathBuf, process::Command, sync::LazyLock};

use clap::Parser as _;
use colored::Colorize;

#[derive(clap::Parser)]
struct Arguments {
    program: String,

    #[arg(long)]
    with: Option<String>,
}

fn main() {
    let arguments = Arguments::parse();

    let editor = arguments
        .with
        .as_ref()
        .map(|editor: &String| Cow::<str>::Borrowed(editor))
        .unwrap_or(shellexpand::full(&CONFIG.options.editor).unwrap());

    let file = CONFIG
        .programs
        .iter()
        .find_map(|(name, program)| {
            (name == &arguments.program
                || program
                    .aliases
                    .as_ref()
                    .map(|aliases| aliases.contains(&arguments.program))
                    .unwrap_or(false))
            .then(|| {
                program
                    .config
                    .iter()
                    .find_map(|config_file| {
                        let path = shellexpand::full(config_file);
                        path.map(|path| PathBuf::from(path.as_ref()).is_file().then_some(path)).ok().flatten()
                    })
                    .unwrap_or_else(|| {
                        eprintln!(
                            "\n{} The program \"{}\" is a known program, but no configuration file exists for it yet. Create either: \n",
                            "Error:".bold().red(),
                            arguments.program.bold().green()
                        );
                        for possible_file in &program.config {
                            eprintln!("    * {}", possible_file.bold().green());
                        }
                        eprintln!();
                        std::process::exit(1);
                    })
            })
        })
        .unwrap_or_else(|| {
            eprintln!(
                "\n{} The program \"{}\" is not known. If this isn't a typo, consider adding an entry in {}.\n",
                "Error:".bold().red(),
                arguments.program.bold().red().underline(),
                "~/.config/cfg/cfg.toml".bold().green()
            );
            std::process::exit(1);
        });

    Command::new(editor.as_ref()).arg(file.as_ref()).exec();
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config = std::fs::read_to_string(shellexpand::tilde("~/.config/cfg/cfg.toml").as_ref());

    let toml = config.map_or_else(
        |_error| {
            let config = include_str!("./default_config.toml");
            std::fs::create_dir_all(shellexpand::tilde("~/.config/cfg").as_ref()).unwrap();
            std::fs::write(shellexpand::tilde("~/.config/cfg/cfg.toml").as_ref(), &config).unwrap();
            Cow::Borrowed(config)
        },
        |config| Cow::Owned(config),
    );

    toml::de::from_str(&toml).unwrap_or_else(|error| {
        eprintln!(
            "{} Malformatted config file: {} exists but doesn't contain valid TOML data: {error}",
            "Error: ".bold().red(),
            "~/.config/cfg/cfg.toml".bold().green()
        );
        std::process::exit(1);
    })
});

#[derive(serde::Deserialize)]
struct Program {
    aliases: Option<Vec<String>>,
    config: Vec<String>,
}

#[derive(serde::Deserialize)]
struct Options {
    editor: String,
}

#[derive(serde::Deserialize)]
struct Config {
    options: Options,
    programs: HashMap<String, Program>,
}
