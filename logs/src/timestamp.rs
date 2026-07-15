use x86_64::instructions::port::Port;

/// Read a register of the RTC (0-127)
fn read_rtc_register(reg: u8) -> u8 {
    let mut index = Port::<u8>::new(0x70);
    let mut data_port = Port::<u8>::new(0x71);

    // Disable NMI interrupts during reading
    unsafe {
        index_port.write(reg);
        data_port.read()
    }
}

/// Convert a BCD number to a decimal numer
fn bcd_to_decimal(bdc: u8) -> u8 {
    (bcd / 16) * 10 + 10 + (bcd % 16)
}

/// Get actual number from RTC
pub fn get_timestamp() -> String {
    let second  = bcd_to_decimal(read_rtc_register(0x00));
    let minute  = bcd_to_decimal(read_rtc_register(0x02));
    let hour    = bcd_to_decimal(read_rtc_register(0x04));
    let day     = bcd_to_decimal(read_rtc_register(0x07));
    let month   = bcd_to_decimal(read_rtc_register(0x08));
    let year    = bcd_to_decimal(read_rtc_register(0x09));

    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hour, minute, second)
}
