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

#[derive(Serialize, Deserialize)]
pub struct Time {
  pre_ans_delay: f64,
  post_ans_delay: f64,
  score_delay: f64,
  game_over_delay: f64,
}
