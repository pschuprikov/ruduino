//! Modules that can be implemented for specific cores.

pub use self::spi::HardwareSpi;
pub use self::timer::{
    Timer8, Timer8Setup, ClockSource8, WaveformGenerationMode8,
    Timer8O2, Timer8O2Setup, ClockSource8O2, WaveformGenerationMode8O2,
    Timer16, Timer16Setup, ClockSource16, WaveformGenerationMode16,
};
pub use self::usart::HardwareUsart;

mod spi;
mod timer;
mod usart;

