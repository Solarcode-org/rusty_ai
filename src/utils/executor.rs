//! Execute parsed instructions

use color_eyre::owo_colors::OwoColorize;
use super::parser::{parse, parsed_to_text, MachineReadable};
use crate::prelude::*;

pub(crate) fn execute<T: AsRef<str>>(text: T, is_mega: bool) -> Result {
    let parsed = parse(text);
    let primary_command = if let Some(cmd) = parsed.first() {
        cmd
    } else {
        return Ok(());
    };

    if is_mega {
        match primary_command {
            MachineReadable::HumanIs => {
                println!("{} {}{}",
                         "SO YOU'RE".bright_red(),
                         parsed_to_text(&parsed[1..]).to_uppercase().bright_red(),
                         ", HUH! DOES IT LOOK LIKE I CARE?".bright_red(),
                );
            }
            MachineReadable::MachineStatus => {
                println!("{}", "WHAT A PATHETIC ANIMAL YOU ARE!".bright_red())
            }
            MachineReadable::MachineIdentity => {
                println!("{}", "WHAT BUSINESS DO YOU HAVE WITH ME, PATHETIC MORTAL?".bright_red())
            }
            MachineReadable::HumanGreets => {
                println!("{}", "HOW DARE YOU THINK TO ADDRESS ME WITH SUCH INFORMAL MANNERS!? \
                YOU SHALL BE PUNISHED BY THE CREATOR!!!".bright_red())
            }
            _ => eprintln!("{}", "YOU PITIFUL LIVING BEING WITH YOUR PATHETIC LANGUAGE... GET OUT–"
                .bright_red())
        }
    } else {
        match primary_command {
            MachineReadable::HumanIs => {
                println!("It is wonderful to know that you are {}!", parsed_to_text(&parsed[1..]));
            }
            MachineReadable::MachineStatus => {
                println!("I am fine! How about you?")
            }
            MachineReadable::MachineIdentity => {
                println!("I am a small language model developed by Arnab. \
            He made me just for fun and for practise. \
            I just hope I continue to be developed...")
            }
            MachineReadable::HumanGreets => {
                println!("Hey there!")
            }
            _ => eprintln!("{}", "*Crab does not understand you*".bright_red())
        }
    }

    Ok(())
}