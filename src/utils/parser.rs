//! The parser module for parsing the strange human speech...

#[derive(Debug)]
pub(super) enum MachineReadable {
    HumanIs,
    MachineStatus,
    MachineIdentity,
    HumanGreets,
    Word(String),
}

pub(super) fn parse<T: AsRef<str>>(text: T) -> Vec<MachineReadable> {
    let text = text.as_ref();
    let mut converted = vec![];

    let split = text.split_whitespace();

    let mut human_is = false;
    let mut machine_stat = false;
    let mut machine_id = false;
    
    for raw_word in split {
        let word = raw_word.to_lowercase();

        match word.as_ref() {
            "i" => human_is = true,
            "how" => machine_stat = true,
            "who" => machine_id = true,

            "am" if human_is => {
                converted.push(MachineReadable::HumanIs);
                human_is = false;
            },
            "you" if machine_stat => {
                converted.push(MachineReadable::MachineStatus);
                machine_stat = false;
            },
            "you" if machine_id => {
                converted.push(MachineReadable::MachineIdentity);
                machine_id = false;
            },

            "hi" | "hey" | "hello" => converted.push(MachineReadable::HumanGreets),

            _ => {
                if !human_is && !machine_stat && !machine_id {
                    converted.push(MachineReadable::Word(raw_word.to_string()))
                }
            }
        }
    }

    converted
}

pub(super) fn parsed_to_text(parsed: &[MachineReadable]) -> String {
    let mut string = String::new();

    for i in parsed {
        if let MachineReadable::Word(word) = i {
            string.push_str(word);
            string.push(' ');
        }
    }

    string
}