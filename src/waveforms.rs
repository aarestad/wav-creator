trait Clocked {
    fn clock(&mut self);
}

trait ClockedWaveformChannel: Clocked {
    fn sequence_step(&self) -> f64;

    fn sample(&self) -> f64;
}

struct LengthCounter {
    enabled: bool,
    halt: bool,
    counter: u8, // 5-bit value
}

struct LinearCounter {
    counter: u8,
    reload_value: u8,
    reload: bool,
    enabled: bool,
}

struct Envelope {
    looping: bool,
    constant: bool,
    period: u8,
    divider: u8,
    volume: u8,
    start: bool,
}

// https://wiki.nesdev.com/w/index.php/APU_Sweep
struct Sweep {
    negation:bool, // false=one's complement; true = two's complement
    enabled:bool,
    divider_period:u8,
    divider:u8,
    negate:bool,
    shift_count:u8,
    reload: bool,
    period: u16,
    timer: u16, // A copy of the timer on the unit
}

// https://wiki.nesdev.com/w/index.php/APU_Triangle
pub struct Triangle {
    timer_period: u16,
    timer: u16,
    sequencer_step: u8,
    length_counter: LengthCounter,
    linear_counter: LinearCounter,
}

pub struct Pulse {
    sweep: Sweep,
    timer_period: u16,
    timer: u16,
    duty_cycle: u8,
    sequencer_step: u8,
    envelope: Envelope,
    length_counter: LengthCounter,
}

pub struct Noise {
    envelope: Envelope,
    length_counter: LengthCounter,
    mode: bool,
    period: u16,
    feedback: u16,
    timer: u16,
}
