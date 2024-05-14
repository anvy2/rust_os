#[test_case]
fn test_println_simple() {
    crate::println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        crate::println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = crate::vga_buffer::WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[crate::vga_buffer::BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
