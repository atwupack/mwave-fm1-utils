use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use wmidi::{Note, Velocity};
use mwave_fm1_utils::FM1Player;
use mwave_fm1_utils::fx::{Effect, ReverbType};

fn main() -> Result<(), Box<dyn Error>> {
  let mut fm1_player = FM1Player::new()?;
  sleep(Duration::from_millis(4 * 150));

  fm1_player.change_voice(2)?;
  fm1_player.enable_effect(Effect::REVERB)?;
  fm1_player.set_effect_parameter(ReverbType::Hall)?;
  fm1_player.play_note(Note::Gb4, Velocity::MAX, 4)?;
  fm1_player.play_note(Note::F4, Velocity::MAX, 3)?;
  fm1_player.play_note(Note::Eb4, Velocity::MAX, 1)?;
  fm1_player.play_note(Note::Db4, Velocity::MAX, 6)?;
  fm1_player.play_note(Note::B3, Velocity::MAX, 2)?;
  fm1_player.play_note(Note::Bb3, Velocity::MAX, 4)?;
  fm1_player.play_note(Note::Ab3, Velocity::MAX, 4)?;
  fm1_player.play_note(Note::Gb3, Velocity::MAX, 4)?;
  fm1_player.disable_effect(Effect::REVERB)?;
  fm1_player.set_effect_parameter(ReverbType::Room)?;

  Ok(())
}