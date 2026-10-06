pub mod fx;

use crate::fx::{Effect, EffectParameter};
use midir::{MidiOutput, MidiOutputConnection};
use std::error::Error;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;
use wmidi::Channel::{Ch1, Ch2};
use wmidi::MidiMessage::{ControlChange, NoteOff, NoteOn, ProgramChange};
use wmidi::{MidiMessage, Note, ProgramNumber, U7, Velocity};
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FM1Error(String);

impl Display for FM1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for FM1Error {}

pub struct FM1Player {
    fm1_out_conn: MidiOutputConnection,
}

impl FM1Player {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let midi_out = MidiOutput::new("My Test Output")?;
        let out_ports = midi_out.ports();
        let fm1_out_conn = out_ports
            .iter()
            .find(|port| midi_out.port_name(port).unwrap().starts_with("FM-1"))
            .map(|port| {
                return midi_out.connect(port, "FM-1").unwrap();
            })
            .ok_or("")?;

        Ok(FM1Player { fm1_out_conn })
    }

    pub fn change_voice(&mut self, voice: u8) -> Result<(), FM1Error> {
        self.send_message(ProgramChange(Ch1, ProgramNumber::from_u8_lossy(voice)))
    }

    pub fn start_note(&mut self, note: Note, velocity: Velocity) -> Result<(), FM1Error> {
        self.send_message(NoteOn(Ch1, note, velocity))
    }

    pub fn stop_note(&mut self, note: Note, velocity: Velocity) -> Result<(), FM1Error> {
        self.send_message(NoteOff(Ch1, note, velocity))
    }

    pub fn play_note(
        &mut self,
        note: Note,
        velocity: Velocity,
        duration: u64,
    ) -> Result<(), FM1Error> {
        self.start_note(note, velocity)
            .and_then(|_| {
                sleep(Duration::from_millis(duration * 150));
                Ok(())
            })
            .and_then(|_| self.stop_note(note, velocity))
    }

    pub fn enable_effect(&mut self, effect: Effect) -> Result<(), FM1Error> {
        self.send_message(ControlChange(Ch2, effect.0, U7::from_u8_lossy(1)))
    }

    pub fn disable_effect(&mut self, effect: Effect) -> Result<(), FM1Error> {
        self.send_message(ControlChange(Ch2, effect.0, U7::from_u8_lossy(0)))
    }

    pub fn set_effect_parameter(
        &mut self,
        parameter: impl EffectParameter,
    ) -> Result<(), FM1Error> {
        self.send_message(ControlChange(
            Ch2,
            parameter.control_function(),
            parameter.control_value(),
        ))
    }

    fn send_message(&mut self, message: MidiMessage) -> Result<(), FM1Error> {
        self.fm1_out_conn
            .send(&message.to_vec())
            .map_err(convert_error)
    }
}

fn convert_error(e: impl Error) -> FM1Error {
    FM1Error(e.to_string())
}
