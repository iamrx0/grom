use std::process;
use clap::{Parser, Subcommand};
use grom::commands::{diary, project, quick_note, sync};
use grom::core::config;

#[derive(Parser)]
#[command(name = "grom")]
#[command(author = "alwaysamer")]
#[command(version = "1.0")]
#[command(about = "Note-Taking")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    project: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    Quick {
        #[arg(value_name = "NOTE_NAME")]
        note_name: String,
    },
    New {
        #[arg(value_name = "PROJECT_NAME")]
        project_name: String,
    },
    Today {},
    Week {},
    Month {},
    Sync {
        #[command(subcommand)]
        command: SyncCommand,
    },
}

#[derive(Subcommand)]
enum SyncCommand {
    Init {
        #[arg(value_name = "REMOTE_URL")]
        remote_url: String,
    },
    Push {
        #[arg(value_name = "MESSAGE")]
        message: String,
    },
    Pull {},
}

fn main() {
    ctrlc::set_handler(move || {}).expect("settings ctrl-c handler");
    let cli = Cli::parse();
    let config = match config::load_config() {
        Ok(cfg) => cfg,
        Err(_) => {
            cliclack::note("T_T", "Unable to load config.").unwrap();
            process::exit(1);
        }
    };

    if let Some(command) = &cli.command {
        match &command {
            Command::Quick { note_name } => {
                if quick_note::quick_note(note_name, config).is_err() {
                    cliclack::note("T_T", "Unable to create quick note").unwrap();
                }
            },
            Command::Today {} => {
                if diary::daily_diary(config).is_err() {
                    cliclack::note("T_T", "Unable to create daily diary").unwrap();
                }
            },
            Command::Week {} => {
                if diary::weekly_diary(config).is_err() {
                    cliclack::note("T_T", "Unable to create weekly diary").unwrap();
                }
            },
            Command::Month {} => {
                if diary::monthly_diary(config).is_err() {
                    cliclack::note("T_T", "Unable to create monthly diary").unwrap();
                }
            },
            Command::New { project_name } => {
                if project::create(project_name.clone(), config).is_err() {
                    cliclack::note("T_T", "Unable to create project").unwrap();
                }
            },
            Command::Sync { command } => {
                match command {
                    SyncCommand::Init { remote_url } => {
                        if sync::init(remote_url.clone(), config).is_err() {
                            cliclack::note("T_T", "Unable to initialize sync.").unwrap();
                        }
                    }
                    SyncCommand::Push { message } => {
                        if sync::push(message.clone(), config).is_err() {
                            cliclack::note("T_T", "Unable to push changes.").unwrap();
                        }
                    }
                    SyncCommand::Pull {} => {
                        if sync::pull(config).is_err() {
                            cliclack::note("T_T", "Unable to pull changes.").unwrap();
                        }
                    }
                }
            }
        }
    } else if let Some(project) = &cli.project {
        if project::open(project.clone(), config).is_err() {
            cliclack::note("T_T", "Unable to open project").unwrap();
        }
    } else if project::interactive_selecion(config).is_err() {
        cliclack::note("T_T", "Unable to open project").unwrap();
    }
}
