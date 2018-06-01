use nzsc_single_player::io::Question;

use std::fmt::Write;

pub fn to_json_string(question: &Question) -> String {
    let mut s = "{".to_string();

    match question {
        &Question::ChooseCharacter {
            ref available_characters,
        } => {
            write!(s, r#""type":"CHOOSE_CHARACTER","availableCharacters":["#).unwrap();
            for character in available_characters {
                write!(s, "\"{}\",", character).unwrap();
            }
        },
        &Question::ChooseBooster {
            ref available_boosters,
        } => {
            write!(s, r#""type":"CHOOSE_BOOSTER","availableBoosters":["#).unwrap();
            for booster in available_boosters {
                write!(s, "\"{}\",", booster).unwrap();
            }
        },
        &Question::ChooseMove {
            ref available_moves,
        } => {
            write!(s, r#""type":"CHOOSE_MOVE","availableMoves":["#).unwrap();
            for available_move in available_moves {
                write!(s, "\"{}\",", available_move).unwrap();
            }
        },
    };

    s.pop(); // Remove the trailing ","
    write!(s, "]}}").unwrap();

    s
}
