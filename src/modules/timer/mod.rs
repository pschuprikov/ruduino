pub use self::timer8::{
    ClockSource as ClockSource8, Timer8, Timer8Setup,
    WaveformGenerationMode as WaveformGenerationMode8,
};

pub use self::timer8o2::{
    ClockSource as ClockSource8O2, Timer8O2, Timer8O2Setup, 
    WaveformGenerationMode as WaveformGenerationMode8O2,
};
pub use self::timer16::{
    ClockSource as ClockSource16, Timer16, Timer16Setup,
    WaveformGenerationMode as WaveformGenerationMode16,
};

mod timer8;
mod timer16;
mod timer8o2;
