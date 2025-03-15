mod prelude;
mod utils;

use clap::Parser;
use color_eyre::owo_colors::OwoColorize;
use prelude::*;
use utils::cli::{Args, Commands};
use crabby_ai::{execute, CRAB_ART};

fn main() -> Result {
    let args = Args::parse();

    match args.command {
        Commands::Start => {
            let crab = if args.mega_pass {
                let password = rpassword::prompt_password("Enter megapass password: ")?;
                if password != "xoxoxoxo1029" {
                    bail!("You shall not pass")
                }

                format!("THE MEGA CRAB HAS BEEN AWAKENED! FEEL THE FEAR!!!!\n{}", CRAB_ART[2].red())
            } else {
                format!("Hey there! I'm Rusty! Nice to meet you. How may I help you today?\n{}",
                        CRAB_ART[1].bright_blue())
            };

            println!("(Use :q to quit)\n{crab}");

            loop {
                let reply = rprompt::prompt_reply("> ")?;

                if reply.to_lowercase().trim() == ":q" {
                    break;
                }

                execute(reply, args.mega_pass)?;
            }
        }
    }

    Ok(())
}
