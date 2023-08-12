//! The primary module containing microcontroller-specific core definitions

/// The ATmega16.
#[cfg(any(avr_mcu_atmega16, feature = "all-mcus"))] pub mod atmega16;
#[cfg(avr_mcu_atmega16)] pub use self::atmega16 as current;

/// The ATmega48.
#[cfg(any(avr_mcu_atmega48, feature = "all-mcus"))] pub mod atmega48;
#[cfg(avr_mcu_atmega48)] pub use self::atmega48 as current;

/// The ATmega32.
#[cfg(any(avr_mcu_atmega32, feature = "all-mcus"))] pub mod atmega32;
#[cfg(avr_mcu_atmega32)] pub use self::atmega32 as current;

/// The ATmega16HVA.
#[cfg(any(avr_mcu_atmega16hva, feature = "all-mcus"))] pub mod atmega16hva;
#[cfg(avr_mcu_atmega16hva)] pub use self::atmega16hva as current;

/// The ATmega328.
///
/// This device is chosen as the default when the crate is targeting non-AVR devices.
#[cfg(any(avr_mcu_atmega328, feature = "all-mcus", not(target_arch = "avr")))] pub mod atmega328;
#[cfg(any(avr_mcu_atmega328, not(target_arch = "avr")))] pub use self::atmega328 as current;

/// The ATmega168P.
#[cfg(any(avr_mcu_atmega168p, feature = "all-mcus"))] pub mod atmega168p;
#[cfg(avr_mcu_atmega168p)] pub use self::atmega168p as current;

/// The ATmega88A.
#[cfg(any(avr_mcu_atmega88a, feature = "all-mcus"))] pub mod atmega88a;
#[cfg(avr_mcu_atmega88a)] pub use self::atmega88a as current;

/// The ATmega32HVBrevB.
#[cfg(any(avr_mcu_atmega32hvbrevb, feature = "all-mcus"))] pub mod atmega32hvbrevb;
#[cfg(avr_mcu_atmega32hvbrevb)] pub use self::atmega32hvbrevb as current;

/// The ATmega162.
#[cfg(any(avr_mcu_atmega162, feature = "all-mcus"))] pub mod atmega162;
#[cfg(avr_mcu_atmega162)] pub use self::atmega162 as current;

/// The ATmega128A.
#[cfg(any(avr_mcu_atmega128a, feature = "all-mcus"))] pub mod atmega128a;
#[cfg(avr_mcu_atmega128a)] pub use self::atmega128a as current;

/// The ATmega32A.
#[cfg(any(avr_mcu_atmega32a, feature = "all-mcus"))] pub mod atmega32a;
#[cfg(avr_mcu_atmega32a)] pub use self::atmega32a as current;

/// The ATmega16HVBrevB.
#[cfg(any(avr_mcu_atmega16hvbrevb, feature = "all-mcus"))] pub mod atmega16hvbrevb;
#[cfg(avr_mcu_atmega16hvbrevb)] pub use self::atmega16hvbrevb as current;

/// The ATmega48A.
#[cfg(any(avr_mcu_atmega48a, feature = "all-mcus"))] pub mod atmega48a;
#[cfg(avr_mcu_atmega48a)] pub use self::atmega48a as current;

/// The ATmega328P.
#[cfg(any(avr_mcu_atmega328p, feature = "all-mcus"))] pub mod atmega328p;
#[cfg(avr_mcu_atmega328p)] pub use self::atmega328p as current;

/// The ATmega168.
#[cfg(any(avr_mcu_atmega168, feature = "all-mcus"))] pub mod atmega168;
#[cfg(avr_mcu_atmega168)] pub use self::atmega168 as current;

/// The ATmega64HVE2.
#[cfg(any(avr_mcu_atmega64hve2, feature = "all-mcus"))] pub mod atmega64hve2;
#[cfg(avr_mcu_atmega64hve2)] pub use self::atmega64hve2 as current;

/// The ATmega64.
#[cfg(any(avr_mcu_atmega64, feature = "all-mcus"))] pub mod atmega64;
#[cfg(avr_mcu_atmega64)] pub use self::atmega64 as current;

/// The ATmega88.
#[cfg(any(avr_mcu_atmega88, feature = "all-mcus"))] pub mod atmega88;
#[cfg(avr_mcu_atmega88)] pub use self::atmega88 as current;

/// The ATmega8535.
#[cfg(any(avr_mcu_atmega8535, feature = "all-mcus"))] pub mod atmega8535;
#[cfg(avr_mcu_atmega8535)] pub use self::atmega8535 as current;

/// The ATmega168PA.
#[cfg(any(avr_mcu_atmega168pa, feature = "all-mcus"))] pub mod atmega168pa;
#[cfg(avr_mcu_atmega168pa)] pub use self::atmega168pa as current;

/// The ATmega168A.
#[cfg(any(avr_mcu_atmega168a, feature = "all-mcus"))] pub mod atmega168a;
#[cfg(avr_mcu_atmega168a)] pub use self::atmega168a as current;

/// The ATmega8515.
#[cfg(any(avr_mcu_atmega8515, feature = "all-mcus"))] pub mod atmega8515;
#[cfg(avr_mcu_atmega8515)] pub use self::atmega8515 as current;

/// The ATmega32HVB.
#[cfg(any(avr_mcu_atmega32hvb, feature = "all-mcus"))] pub mod atmega32hvb;
#[cfg(avr_mcu_atmega32hvb)] pub use self::atmega32hvb as current;

/// The ATmega88PA.
#[cfg(any(avr_mcu_atmega88pa, feature = "all-mcus"))] pub mod atmega88pa;
#[cfg(avr_mcu_atmega88pa)] pub use self::atmega88pa as current;

/// The ATmega88P.
#[cfg(any(avr_mcu_atmega88p, feature = "all-mcus"))] pub mod atmega88p;
#[cfg(avr_mcu_atmega88p)] pub use self::atmega88p as current;

/// The ATmega128.
#[cfg(any(avr_mcu_atmega128, feature = "all-mcus"))] pub mod atmega128;
#[cfg(avr_mcu_atmega128)] pub use self::atmega128 as current;

/// The ATmega48P.
#[cfg(any(avr_mcu_atmega48p, feature = "all-mcus"))] pub mod atmega48p;
#[cfg(avr_mcu_atmega48p)] pub use self::atmega48p as current;

/// The ATmega16A.
#[cfg(any(avr_mcu_atmega16a, feature = "all-mcus"))] pub mod atmega16a;
#[cfg(avr_mcu_atmega16a)] pub use self::atmega16a as current;

/// The ATmega16HVB.
#[cfg(any(avr_mcu_atmega16hvb, feature = "all-mcus"))] pub mod atmega16hvb;
#[cfg(avr_mcu_atmega16hvb)] pub use self::atmega16hvb as current;

/// The ATmega64A.
#[cfg(any(avr_mcu_atmega64a, feature = "all-mcus"))] pub mod atmega64a;
#[cfg(avr_mcu_atmega64a)] pub use self::atmega64a as current;

/// The ATmega8HVA.
#[cfg(any(avr_mcu_atmega8hva, feature = "all-mcus"))] pub mod atmega8hva;
#[cfg(avr_mcu_atmega8hva)] pub use self::atmega8hva as current;

/// The ATmega48PA.
#[cfg(any(avr_mcu_atmega48pa, feature = "all-mcus"))] pub mod atmega48pa;
#[cfg(avr_mcu_atmega48pa)] pub use self::atmega48pa as current;

/// The ATtiny261.
#[cfg(any(avr_mcu_attiny261, feature = "all-mcus"))] pub mod attiny261;
#[cfg(avr_mcu_attiny261)] pub use self::attiny261 as current;

/// The ATtiny26.
#[cfg(any(avr_mcu_attiny26, feature = "all-mcus"))] pub mod attiny26;
#[cfg(avr_mcu_attiny26)] pub use self::attiny26 as current;

/// The ATtiny20.
#[cfg(any(avr_mcu_attiny20, feature = "all-mcus"))] pub mod attiny20;
#[cfg(avr_mcu_attiny20)] pub use self::attiny20 as current;

/// The ATtiny13.
#[cfg(any(avr_mcu_attiny13, feature = "all-mcus"))] pub mod attiny13;
#[cfg(avr_mcu_attiny13)] pub use self::attiny13 as current;

/// The ATtiny461A.
#[cfg(any(avr_mcu_attiny461a, feature = "all-mcus"))] pub mod attiny461a;
#[cfg(avr_mcu_attiny461a)] pub use self::attiny461a as current;

/// The ATtiny261A.
#[cfg(any(avr_mcu_attiny261a, feature = "all-mcus"))] pub mod attiny261a;
#[cfg(avr_mcu_attiny261a)] pub use self::attiny261a as current;

/// The ATtiny2313.
#[cfg(any(avr_mcu_attiny2313, feature = "all-mcus"))] pub mod attiny2313;
#[cfg(avr_mcu_attiny2313)] pub use self::attiny2313 as current;

/// The ATtiny4313.
#[cfg(any(avr_mcu_attiny4313, feature = "all-mcus"))] pub mod attiny4313;
#[cfg(avr_mcu_attiny4313)] pub use self::attiny4313 as current;

/// The ATtiny40.
#[cfg(any(avr_mcu_attiny40, feature = "all-mcus"))] pub mod attiny40;
#[cfg(avr_mcu_attiny40)] pub use self::attiny40 as current;

/// The ATtiny25.
#[cfg(any(avr_mcu_attiny25, feature = "all-mcus"))] pub mod attiny25;
#[cfg(avr_mcu_attiny25)] pub use self::attiny25 as current;

/// The ATtiny2313A.
#[cfg(any(avr_mcu_attiny2313a, feature = "all-mcus"))] pub mod attiny2313a;
#[cfg(avr_mcu_attiny2313a)] pub use self::attiny2313a as current;

/// The ATtiny1634.
#[cfg(any(avr_mcu_attiny1634, feature = "all-mcus"))] pub mod attiny1634;
#[cfg(avr_mcu_attiny1634)] pub use self::attiny1634 as current;

/// The ATtiny861.
#[cfg(any(avr_mcu_attiny861, feature = "all-mcus"))] pub mod attiny861;
#[cfg(avr_mcu_attiny861)] pub use self::attiny861 as current;

/// The ATtiny461.
#[cfg(any(avr_mcu_attiny461, feature = "all-mcus"))] pub mod attiny461;
#[cfg(avr_mcu_attiny461)] pub use self::attiny461 as current;

/// The ATtiny861A.
#[cfg(any(avr_mcu_attiny861a, feature = "all-mcus"))] pub mod attiny861a;
#[cfg(avr_mcu_attiny861a)] pub use self::attiny861a as current;

/// The ATtiny45.
#[cfg(any(avr_mcu_attiny45, feature = "all-mcus"))] pub mod attiny45;
#[cfg(avr_mcu_attiny45)] pub use self::attiny45 as current;


