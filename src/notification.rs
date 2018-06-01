use nzsc_single_player::io::{
    Notification,
    WhoGetsTheHeadstart,
    WhoGetsThePoint,
};

pub fn to_json_string(notification: &Notification) -> String {
    match notification {
        &Notification::CharacterSelectionAndHeadstart {
            ref human_character,
            ref computer_character,
            ref who_gets_the_headstart,
        } => {
            let who_gets_the_headstart = match who_gets_the_headstart {
                WhoGetsTheHeadstart::Neither => "NEITHER",
                WhoGetsTheHeadstart::JustComputer => "COMPUTER",
                WhoGetsTheHeadstart::JustHuman => "HUMAN",
            };
            format!(r#"{{"type":"CHARACTER_SELECTION_AND_HEADSTART","humanCharacter":"{}","computerCharacter":"{}","whoGetsTheHeadstart":"{}"}}"#, human_character, computer_character, who_gets_the_headstart)
        },

        &Notification::SameCharacterSelection {
            ref both_character,
        } => {
            format!(r#"{{"type":"SAME_CHARACTER_SELECTION","bothCharacter":"{}"}}"#, both_character)
        },

        &Notification::BoosterSelection {
            ref human_booster,
            ref computer_booster,
        } => {
            format!(r#"{{"type":"BOOSTER_SELECTION","humanBooster":"{}","computerBooster":"{}"}}"#, human_booster, computer_booster)
        },

        &Notification::MoveSelectionAndOutcome {
            ref human_move,
            ref computer_move,
            ref who_gets_the_point,
        } => {
            let who_gets_the_point = match who_gets_the_point {
                WhoGetsThePoint::Neither => "NEITHER",
                WhoGetsThePoint::JustComputer => "COMPUTER",
                WhoGetsThePoint::JustHuman => "HUMAN",
                WhoGetsThePoint::Both => "BOTH",
            };

            format!(r#"{{"type":"MOVE_SELECTION_AND_OUTCOME","humanMove":"{}","computerMove":"{}","whoGetsThePoint":"{}"}}"#, human_move, computer_move, who_gets_the_point)
        },

        &Notification::ScoreUpdate {
            ref human_points,
            ref computer_points,
        } => {
            format!(r#"{{"type":"SCORE_UPDATE","humanPoints":{},"computerPoints":{}}}"#, human_points, computer_points)
        },

        &Notification::TiebreakingScoreSetback {
            ref both_points,
        } => {
            format!(r#"{{"type":"TIEBREAKING_SCORE_SETBACK","bothPoints":{}}}"#, both_points)
        },

        &Notification::GameOver {
            human_points,
            computer_points,
        } => {
            fn nickname_for_margin(margin: u8) -> String {
                match margin {
                    1 => "Clinch".to_string(),
                    2 => "Hypnotization".to_string(),
                    3 => "Obliteration".to_string(),
                    4 => "Annihilation".to_string(),
                    5 => "Wipeout".to_string(),
                    _ => {
                        panic!("Impossible victory margin: {}", margin);
                    }
                }
            }

            let margin = (human_points as i16 - computer_points as i16).abs() as u8;
            let margin_nickname = nickname_for_margin(margin);
            format!(r#"{{"type":"GAME_OVER","humanPoints":{},"computerPoints":{},"marginNickName":"{}"}}"#, human_points, computer_points, margin_nickname)
        },

        &Notification::CharacterNonexistentPenalty {
            ref attempted_character_name,
        } => {
            format!(r#"{{"type":"CHARACTER_NONEXISTENT_PENALTY","attemptedCharacterName":"{}"}}"#, attempted_character_name)
        },

        &Notification::CharacterThreeTimesInARowPenalty {
            ref attempted_character,
        } => {
            format!(r#"{{"type":"CHARACTER_THREE_TIMES_IN_A_ROW_PENALTY","attemptedCharacter":"{}"}}"#, attempted_character)
        },

        &Notification::BoosterNonexistentPenalty {
            ref attempted_booster_name,
        } => {
            format!(r#"{{"type":"BOOSTER_NONEXISTENT_PENALTY","attemptedBoosterName":"{}"}}"#, attempted_booster_name)
        },

        &Notification::BoosterFromWrongCharacterPenalty {
            ref attempted_booster,
        } => {
            format!(r#"{{"type":"BOOSTER_FROM_WRONG_CHARACTER_PENALTY","attemptedBooster":"{}"}}"#, attempted_booster)
        },

        &Notification::MoveNonexistentPenalty {
            ref attempted_move_name,
        } => {
            format!(r#"{{"type":"MOVE_NONEXISTENT_PENALTY","attemptedMoveName":"{}"}}"#, attempted_move_name)
        },

        &Notification::MoveThreeTimesInARowPenalty {
            ref attempted_move,
        } => {
            format!(r#"{{"type":"MOVE_THREE_TIMES_IN_A_ROW_PENALTY","attemptedMove":"{}"}}"#, attempted_move)
        },

        &Notification::MoveSingleUsePenalty {
            ref attempted_move,
        } => {
            format!(r#"{{"type":"MOVE_SINGLE_USE_PENALTY","attemptedMove":"{}"}}"#, attempted_move)
        },

        &Notification::MoveDestroyedPenalty {
            ref attempted_move,
        } => {
            format!(r#"{{"type":"MOVE_DESTROYED_PENALTY","attemptedMove":"{}"}}"#, attempted_move)
        },

        &Notification::MoveFromWrongCharacterPenalty {
            ref attempted_move,
        } => {
            format!(r#"{{"type":"MOVE_FROM_WRONG_CHARACTER_PENALTY","attemptedMove":"{}"}}"#, attempted_move)
        },

        &Notification::MoveFromWrongBoosterPenalty {
            ref attempted_move,
        } => {
            format!(r#"{{"type":"MOVE_FROM_WRONG_BOOSTER_PENALTY","attemptedMove":"{}"}}"#, attempted_move)
        },
    }
}
