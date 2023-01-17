pub mod color_term {
    extern crate ansi_term;

    use ansi_term::Colour;
    use ansi_term::Style;

    pub fn color_term() {
        println!(
            "{}, {} and {}",
            Colour::Yellow.paint("This is colored"),
            Style::new().bold().paint("this is bold"),
            Colour::Yellow.bold().paint("this is bold and colored")
        );
    }
}
