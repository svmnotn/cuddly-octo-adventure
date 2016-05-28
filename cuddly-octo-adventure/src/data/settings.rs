use std::path::PathBuf;
use super::Team;

#[derive(Serialize, Deserialize)]
pub struct Settings {
  // timer_font,
  // timer_color,
  // bg_color,
  #[serde(skip_serializing_if="Option::is_none")]
  bg_loc: Option<PathBuf>,
  message: Message,
  sound: Sound,
  time: Time,
  teams: Vec<Team>,
}

impl Default for Settings {
  fn default() -> Settings {
    Settings {
      bg_loc: None,
      message: Message::default(),
      sound: Sound::default(),
      time: Time::default(),
      teams: Vec::new(),
    }
  }
}

// TODO add more custom text!
#[derive(Serialize, Deserialize)]
pub struct Message {
  score_presentation: String,
  game_over: String,
  team_prefix: String,
  score_prefix: String,
  singular_point_name: String,
  #[serde(skip_serializing_if="Option::is_none")]
  plural_point_name: Option<String>,
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

#[derive(Serialize, Deserialize)]
pub struct Sound {
  #[serde(skip_serializing_if="Option::is_none")]
  bg_sound_loc: Option<PathBuf>,
  bg_sound_vol: f32,
  #[serde(skip_serializing_if="Option::is_none")]
  yay_sound_loc: Option<PathBuf>,
  yay_sound_vol: f32,
  #[serde(skip_serializing_if="Option::is_none")]
  nay_sound_loc: Option<PathBuf>,
  nay_sound_vol: f32,
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


#[derive(Serialize, Deserialize)]
pub struct Time {
  pre_ans_delay: f64,
  post_ans_delay: f64,
  score_delay: f64,
  game_over_delay: f64,
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
