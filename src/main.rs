use nsf::NsfHeader;
use tetanes::{
    audio::Audio,
    common::NesRegion,
    control_deck::ControlDeck,
    memory::{MemWrite, RamState},
};

use pix_engine::prelude::*;

use crate::nsf::load_nsf_as_cart_data;

use std::iter::zip;

mod nsf;

struct NesMusicPlayer {
    header: NsfHeader,
    control_deck: ControlDeck,
    audio: Audio,
}

impl NesMusicPlayer {
    pub fn from_nsf(nsf_file_name: &str) -> anyhow::Result<Self> {
        let (header, rom) = load_nsf_as_cart_data(nsf_file_name)?;
        println!("{:X?}", header);

        let mut control_deck = ControlDeck::new(NesRegion::Ntsc, RamState::AllZeros);
        control_deck.cpu_mut().debugging = true;

        control_deck.load_rom(
            header.song_name.to_str().expect("bad song name"),
            &mut &rom[..],
        )?;

        for addr in 0..=0x07FF {
            control_deck.cpu_mut().write(addr, 0x0);
        }

        for addr in 0x6000..=0x7FFF {
            control_deck.cpu_mut().write(addr, 0x0);
        }

        for addr in 0x4000..=0x4013 {
            control_deck.cpu_mut().write(addr, 0x0);
        }

        control_deck.cpu_mut().write(0x4015, 0x0);
        control_deck.cpu_mut().write(0x4015, 0x0F);
        control_deck.cpu_mut().write(0x4017, 0x40);

        for (addr, val) in zip(0x5FF8..=0x5FFF, header.bankswitch_init) {
            control_deck.cpu_mut().write(addr, val);
        }

        let sp = control_deck.cpu().sp;
        let play_addr_bytes = (header.play_address - 1).to_le_bytes();

        control_deck
            .cpu_mut()
            .write(0x0100 + u16::from(sp) + 1, play_addr_bytes[0]);

        control_deck
            .cpu_mut()
            .write(0x0100 + u16::from(sp) + 2, play_addr_bytes[1]);

        control_deck.cpu_mut().acc = header.starting_song;
        control_deck.cpu_mut().pc = header.init_address;

        let audio = Audio::new(control_deck.apu().sample_rate(), 44_100.0, 4096);

        Ok(NesMusicPlayer {
            header,
            control_deck,
            audio,
        })
    }
}

impl AppState for NesMusicPlayer {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        self.audio.open_playback(s)?;
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        let seconds_to_run = s.delta_time().as_secs_f32().clamp(0.0, 1.0 / 60.0);
        self.control_deck.clock_seconds(seconds_to_run)?;
        let samples = self.control_deck.audio_samples();
        self.audio.output(samples, false, 0.0005);
        self.control_deck.clear_audio_samples();
        self.control_deck.cpu_mut().pc = self.header.play_address;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut music_player = NesMusicPlayer::from_nsf("mario.nsf")?;

    let mut engine = PixEngine::builder()
        .hidden()
        .with_frame_rate()
        .target_frame_rate(60)
        .build()?;

    engine.run(&mut music_player)?;

    Ok(())
}
