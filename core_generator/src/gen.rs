use avr_mcu::*;
use std::io;
use std::collections::HashMap;
use std::io::prelude::*;

pub fn write_registers(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    let mut registers = HashMap::<String, HashMap<String, Bitfield>>::new();
    for register in mcu.registers() {
        if !registers.contains_key(&register.name) {
            registers.insert(register.name.clone(), HashMap::new());
            let ty = if register.size == 1 { "u8" } else { "u16" };

            writeln!(w, "#[allow(non_camel_case_types)]")?;
            writeln!(w, "pub struct {};", register.name)?;
            writeln!(w)?;

            writeln!(w, "impl Register for {} {{", register.name)?;
            writeln!(w, "    type T = {};", ty)?;
            writeln!(w, "    const ADDRESS: *mut {} = 0x{:x} as *mut {};", ty, register.offset, ty)?;
            writeln!(w, "}}")?;
        }

        for bitfield in &register.bitfields {
            registers.get_mut(&register.name).unwrap().insert(bitfield.name.clone(), bitfield.clone());
        }
    }

    for (register_name, bitfields) in registers {
        writeln!(w, "impl {} {{", register_name)?;
        for (_, bitfield) in &bitfields {
            // Create a mask for the whole bitset.
            writeln!(w, "    pub const {}: RegisterBits<Self> = RegisterBits::new(0x{:x});", bitfield.name, bitfield.mask)?;

            // We create masks for the individual bits in the field if there
            // is more than one bit in the field.
            if bitfield.mask.count_ones() > 1 {
                let mut current_mask = bitfield.mask;
                let mut current_mask_bit_num = 0;
                for current_register_bit_num in 0..15 {
                    if (current_mask & 0b1) == 0b1 {
                        let bit_name = format!("{}{}", bitfield.name, current_mask_bit_num);
                        if !bitfields.contains_key(&bit_name) {
                            writeln!(w, "    pub const {}: RegisterBits<Self> = RegisterBits::new(1<<{});",
                                     bit_name, current_register_bit_num)?;
                        }
                        current_mask_bit_num += 1;
                    }

                    current_mask >>= 1;
                }
            }
            writeln!(w)?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;
    }

    Ok(())
}

pub fn write_pins(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    if let Some(port) = mcu.peripheral("PORT") {
        writeln!(w, "pub mod port {{")?;
        writeln!(w, "    #![allow(unused_imports)]")?;
        writeln!(w)?;
        writeln!(w, "    use super::*;")?;
        writeln!(w, "    use crate::Pin;")?;
        writeln!(w)?;

        for instance in port.instances.iter() {
            let port_letter = instance.name.chars().rev().next().unwrap();

            for signal in instance.signals.iter() {
                let idx = signal.index.expect("signal with no index");
                let struct_name = format!("{}{}", port_letter, idx);

                let io_module = mcu.modules.iter().find(|m| m.name == "PORT")
                    .expect("no port io module defined for this port");
                let register_group = io_module.register_groups.iter()
                    .find(|rg| rg.name == instance.name)
                    .expect("no register group defined for this port");

                writeln!(w, "    pub struct {};", struct_name)?;
                writeln!(w)?;
                writeln!(w, "    impl Pin for {} {{", struct_name)?;
                for reg in register_group.registers.iter() {
                    let mut const_name = reg.name.clone();
                    const_name.pop(); // Pop port character from register name (DDRB/PORTB/etc)..

                    writeln!(w, "        /// {}.", reg.caption)?;
                    writeln!(w, "        type {} = {};", const_name, reg.name)?;
                }
                writeln!(w, "        /// {}", signal.pad)?;
                writeln!(w, "        const MASK: u8 = 1<<{};", idx)?;
                writeln!(w, "    }}")?;
                writeln!(w)?;
            }
        }

        writeln!(w, "}}")?;
        writeln!(w)?;
    }
    Ok(())
}

pub fn write_spi_modules(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    if let Some(module) = mcu.module("SPI") {
        let peripheral = mcu.peripheral("SPI").expect("found SPI module but no peripheral");
        let port_peripheral = mcu.port_peripheral();

        writeln!(w, "pub struct Spi;")?;
        writeln!(w)?;
        writeln!(w, "impl modules::HardwareSpi for Spi {{")?;

        for spi_signal in peripheral.signals() {
            let spi_signal_name = spi_signal.group.clone().expect("spi signal does not have group name");
            let (port_instance, port_signal) = port_peripheral.instance_signal_with_pad(&spi_signal.pad)
                .expect("no port signal associated with the spi signal pad");
            let pin_name = self::pin_name(port_instance, port_signal);

            let const_name = match &spi_signal_name[..] {
                "MISO" => "MasterInSlaveOut",
                "MOSI" => "MasterOutSlaveIn",
                "SCK" => "Clock",
                "SS" => "SlaveSelect",
                _ => panic!("unknown spi signal name: '{}'", spi_signal_name),
            };

            writeln!(w, "    type {} = {};", const_name, pin_name)?;
        }

        for reg in module.registers() {
            let const_name = match &reg.caption[..] {
                "SPI Data Register" => "DataRegister",
                "SPI Status Register" => "StatusRegister",
                "SPI Control Register" => "ControlRegister",
                _ => panic!("unknown SPI module register: {}", reg.caption),
            };

            writeln!(w, "    type {} = {};", const_name, reg.name)?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;
    }
    Ok(())
}

pub fn write_usarts(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    if let Some(module) = mcu.module("USART") {
        for usart in module.register_groups.iter() {
            writeln!(w, "/// The {} module.", usart.name)?;
            writeln!(w, "pub struct {};", usart.name)?;
            writeln!(w)?;
            //writeln!(w, "impl modules::HardwareUsart for {} {{", usart.name)?;
            //for register in usart.registers.iter() {
            //    let reg_ty = if register.name.starts_with("UDR") { // the data register.
            //        "DataRegister".to_owned()
            //    } else if register.name.starts_with("UCSR") { // one of the three control/status registers.
            //        let suffix = register.name.chars().rev().next().unwrap();
            //        format!("ControlRegister{}", suffix)
            //    } else if register.name.starts_with("UBRR") { // the baud rate register.
            //        "BaudRateRegister".to_owned()
            //    } else {
            //        panic!("unknown usart register '{}'", register.name);
            //    };
            //    writeln!(w, "    type {} = {};", reg_ty, register.name)?;
            //}
            //writeln!(w, "}}")?;
            writeln!(w)?;
        }
    }
    Ok(())
}

pub fn write_timers(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    if let Some(tc) = mcu.module("TC8") { // Timer/Counter, 8-bit.
        if is_double_output_compare(tc) {
            write_timer8o2(tc, w);
        } else {
            write_timer8(tc, w);
        }
    }

    if let Some(tc) = mcu.module("TC16") { // Timer/Counter, 16-bit.
        write_timer16(tc, w);
    }

    Ok(())
}

fn is_double_output_compare(tc: &Module) -> bool {
    return tc.registers().filter(|r| r.name.starts_with("OCR")).count() > 1;
}

fn write_timer8(tc: &Module, w: &mut dyn Write) -> Result<(), io::Error> {
    const TYPE_NAME: &'static str = "Timer8";

    let find_reg = |name: &'static str| {
        tc.registers().find(|r| r.name.starts_with(name))
            .expect(&format!("could not find '{}' register", name))
    };
    let has_bf_suffix = |r: &Register, name: &'static str, suffix: &'static str| {
        r.bitfields.iter().any(|r| r.name.starts_with(name) && r.name.ends_with(suffix))
    };
    let find_reg_suffix = |name: &'static str, suffix: &'static str| {
        tc.registers().find(|r| r.name.starts_with(name) && r.name.ends_with(suffix))
            .expect(&format!("could not find '{}' register", name))
    };
    let timer_number = tc.register_groups.first().unwrap().name.chars().last().unwrap()
        .to_digit(10).unwrap();

    let TIMSK = find_reg("TIMSK");

    writeln!(w, "/// 8-bit timer.")?;
    writeln!(w, "pub struct {};", TYPE_NAME)?;
    writeln!(w)?;
    writeln!(w, "impl modules::Timer8 for {} {{", TYPE_NAME)?;
    writeln!(w, "    type Compare = {};", find_reg("OCR").name)?;
    writeln!(w, "    type Counter = {};", find_reg("TCNT").name)?;
    writeln!(w, "    type Control = {};", find_reg("TCCR").name)?;
    writeln!(w, "    type InterruptMask = {};", TIMSK.name)?;
    writeln!(w, "    type InterruptFlag = {};", find_reg("TIFR").name)?;
    writeln!(w, "    const CS0: RegisterBits<Self::Control> = Self::Control::CS00;")?;
    writeln!(w, "    const CS1: RegisterBits<Self::Control> = Self::Control::CS01;")?;
    writeln!(w, "    const CS2: RegisterBits<Self::Control> = Self::Control::CS02;")?;
    writeln!(w, "    const WGM0: RegisterBits<Self::Control> = Self::Control::WGM00;")?;
    writeln!(w, "    const WGM1: RegisterBits<Self::Control> = Self::Control::WGM01;")?;
    if has_bf_suffix(TIMSK, "OCIE", "A") {
        writeln!(w, "    const OCIE: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE{}A;", timer_number)?;
    } else {
        writeln!(w, "    const OCIE: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE{};", timer_number)?;
    }
    writeln!(w, "}}")?;

    Ok(())
}

fn write_timer8o2(tc: &Module, w: &mut dyn Write) -> Result<(), io::Error> {
    const TYPE_NAME: &'static str = "Timer8";

    let find_reg = |name: &'static str| {
        tc.registers().find(|r| r.name.starts_with(name))
            .expect(&format!("could not find '{}' register", name))
    };
    let has_bf_suffix = |r: &Register, name: &'static str, suffix: &'static str| {
        r.bitfields.iter().any(|r| r.name.starts_with(name) && r.name.ends_with(suffix))
    };
    let find_reg_suffix = |name: &'static str, suffix: &'static str| {
        tc.registers().find(|r| r.name.starts_with(name) && r.name.ends_with(suffix))
            .expect(&format!("could not find '{}' register", name))
    };
    let timer_number = tc.register_groups.first().unwrap().name.chars().last().unwrap()
        .to_digit(10).unwrap();

    let TIMSK = find_reg("TIMSK");

    writeln!(w, "/// 8-bit timer.")?;
    writeln!(w, "pub struct {};", TYPE_NAME)?;
    writeln!(w)?;
    writeln!(w, "impl modules::Timer8O2 for {} {{", TYPE_NAME)?;
    writeln!(w, "    type CompareA = {};", find_reg_suffix("OCR", "A").name)?;
    writeln!(w, "    type CompareB = {};", find_reg_suffix("OCR", "B").name)?;
    writeln!(w, "    type Counter = {};", find_reg("TCNT").name)?;
    writeln!(w, "    type ControlA = {};", find_reg_suffix("TCCR", "A").name)?;
    writeln!(w, "    type ControlB = {};", find_reg_suffix("TCCR", "B").name)?;
    writeln!(w, "    type InterruptMask = {};", TIMSK.name)?;
    writeln!(w, "    type InterruptFlag = {};", find_reg("TIFR").name)?;
    writeln!(w, "    const CS0: RegisterBits<Self::ControlB> = Self::ControlB::CS00;")?;
    writeln!(w, "    const CS1: RegisterBits<Self::ControlB> = Self::ControlB::CS01;")?;
    writeln!(w, "    const CS2: RegisterBits<Self::ControlB> = Self::ControlB::CS02;")?;
    writeln!(w, "    const WGM0: RegisterBits<Self::ControlA> = Self::ControlA::WGM00;")?;
    writeln!(w, "    const WGM1: RegisterBits<Self::ControlA> = Self::ControlA::WGM01;")?;
    writeln!(w, "    const WGM2: RegisterBits<Self::ControlB> = Self::ControlB::WGM02;")?;
    writeln!(w, "    const OCIEA: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE{}A;", timer_number)?;
    writeln!(w, "    const OCIEB: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE{}B;", timer_number)?;
    writeln!(w, "}}")?;

    Ok(())
}

fn write_timer16(tc: &Module, w: &mut dyn Write) -> Result<(), io::Error> {
    const TYPE_NAME: &'static str = "Timer16";

    let find_reg = |name: &'static str| {
        tc.registers().find(|r| r.name.starts_with(name))
            .expect(&format!("could not find '{}' register", name))
    };
    let find_reg_suffix = |name: &'static str, suffix: &'static str| {
        tc.registers().find(|r| r.name.starts_with(name) && r.name.ends_with(suffix))
            .expect(&format!("could not find '{}' register", name))
    };
    let timer_number = tc.register_groups.first().unwrap().name.chars().last().unwrap()
        .to_digit(10).unwrap();

    writeln!(w, "/// 16-bit timer.")?;
    writeln!(w, "pub struct {};", TYPE_NAME)?;
    writeln!(w)?;
    writeln!(w, "impl modules::Timer16 for {} {{", TYPE_NAME)?;
    writeln!(w, "    type CompareA = {};", find_reg_suffix("OCR", "A").name)?;
    writeln!(w, "    type CompareB = {};", find_reg_suffix("OCR", "B").name)?;
    writeln!(w, "    type Counter = {};", find_reg("TCNT").name)?;
    writeln!(w, "    type ControlA = {};", find_reg_suffix("TCCR", "A").name)?;
    writeln!(w, "    type ControlB = {};", find_reg_suffix("TCCR", "B").name)?;
    writeln!(w, "    type InterruptMask = {};", find_reg("TIMSK").name)?;
    writeln!(w, "    type InterruptFlag = {};", find_reg("TIFR").name)?;
    writeln!(w, "    const CS0: RegisterBits<Self::ControlB> = Self::ControlB::CS10;")?;
    writeln!(w, "    const CS1: RegisterBits<Self::ControlB> = Self::ControlB::CS11;")?;
    writeln!(w, "    const CS2: RegisterBits<Self::ControlB> = Self::ControlB::CS12;")?;
    writeln!(w, "    const WGM0: RegisterBits<Self::ControlA> = Self::ControlA::WGM10;")?;
    writeln!(w, "    const WGM1: RegisterBits<Self::ControlA> = Self::ControlA::WGM11;")?;
    writeln!(w, "    const WGM2: RegisterBits<Self::ControlB> = Self::ControlB::WGM10;")?;
    writeln!(w, "    const WGM3: RegisterBits<Self::ControlB> = Self::ControlB::WGM11;")?;
    writeln!(w, "    const OCIEA: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE{}A;", timer_number)?;
    writeln!(w, "}}")?;

    Ok(())
}




/// Gets the name of a pin.
fn pin_name(instance: &Instance, signal: &Signal) -> String {
    let idx = signal.index.expect("signal with no index");
    let letter = instance.name.chars().rev().next().unwrap();
    format!("port::{}{}", letter, idx)
}
