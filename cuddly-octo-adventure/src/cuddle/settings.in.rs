use std::path::PathBuf;
use super::Team;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub timer_color: String,
    pub bg_color: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub bg_loc: Option<PathBuf>,
    pub message: Message,
    pub sound: Sound,
    pub time: Time,
    pub teams: Vec<Team>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            timer_color: "".to_owned(),
            bg_color: "".to_owned(),
            bg_loc: None,
            message: Message::default(),
            sound: Sound::default(),
            time: Time::default(),
            teams: vec![Team::default()],
        }
    }
}

// TODO add more custom text!
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub score_presentation: String,
    pub game_over: String,
    pub team_prefix: String,
    pub score_prefix: String,
    pub singular_point_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub plural_point_name: Option<String>,
}

impl Default for Message {
    fn default() -> Message {
        Message {
            score_presentation: "Current Scores".to_owned(),
            game_over: "Tha tha tha tha That's All Folks!".to_owned(),
            team_prefix: "Team".to_owned(),
            score_prefix: "Score".to_owned(),
            singular_point_name: "Fake Internet Point".to_owned(),
            plural_point_name: Some("FIPs".to_owned()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sound {
    #[serde(skip_serializing_if="Option::is_none")]
    pub bg_sound_loc: Option<PathBuf>,
    pub bg_sound_vol: f64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub yay_sound_loc: Option<PathBuf>,
    pub yay_sound_vol: f64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub nay_sound_loc: Option<PathBuf>,
    pub nay_sound_vol: f64,
}

impl Default for Sound {
    fn default() -> Sound {
        Sound {
            bg_sound_loc: None,
            bg_sound_vol: 100.0,
            yay_sound_loc: None,
            yay_sound_vol: 100.0,
            nay_sound_loc: None,
            nay_sound_vol: 100.0,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    pub pre_ans_delay: f64,
    pub post_ans_delay: f64,
    pub score_delay: f64,
    pub game_over_delay: f64,
}

impl Default for Time {
    fn default() -> Time {
        Time {
            pre_ans_delay: 60.0,
            post_ans_delay: 5.0,
            score_delay: 5.0,
            game_over_delay: 8.0,
        }
    }
}
