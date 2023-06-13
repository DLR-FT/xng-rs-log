use core::fmt::{Display, Write};
use log::{max_level, Log, Metadata, Record};

extern "C" {
    pub fn XalPutchar(c: i32) -> i32;
}

pub struct XalLogger;

impl Log for XalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() < max_level()
    }

    fn log(&self, record: &Record) {
        let mut writer = XalWriter;
        write(&mut writer, record).unwrap();
    }

    fn flush(&self) {}
}

fn write(writer: &mut impl Write, record: &Record) -> Result<(), core::fmt::Error> {
    core::writeln!(
        writer,
        "{}: {} {}",
        record.target(),
        ColourLogLevel::from(record.level()),
        record.args(),
    )
}

pub struct XalWriter;

impl Write for XalWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            unsafe { XalPutchar(c as i32) };
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        unsafe { XalPutchar(c as i32) };
        Ok(())
    }
}

pub struct ColourLogLevel(log::Level);

impl ColourLogLevel {
    const RESET_COLOUR: &str = "\x1b[39m";
    const RED_COLOUR: &str = "\x1b[31m";
    const GREEN_COLOUR: &str = "\x1b[32m";
    const YELLOW_COLOUR: &str = "\x1b[33m";
    const BLUE_COLOUR: &str = "\x1b[34m";
    const MAGENTA_COLOUR: &str = "\x1b[35m";

    pub fn as_colour_code(&self) -> &'static str {
        match self.0 {
            log::Level::Error => Self::RED_COLOUR,
            log::Level::Warn => Self::YELLOW_COLOUR,
            log::Level::Info => Self::GREEN_COLOUR,
            log::Level::Debug => Self::BLUE_COLOUR,
            log::Level::Trace => Self::MAGENTA_COLOUR,
        }
    }
}

impl From<log::Level> for ColourLogLevel {
    fn from(level: log::Level) -> Self {
        Self(level)
    }
}

impl Display for ColourLogLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            Self::RESET_COLOUR,
            self.as_colour_code(),
            self.0,
            Self::RESET_COLOUR
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use core::fmt::Write;
    use heapless::String;

    #[derive(Default)]
    struct TestWriter {
        test_output: String<100>,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            for c in s.chars() {
                self.test_output.push(c).unwrap();
            }
            Ok(())
        }

        fn write_char(&mut self, c: char) -> core::fmt::Result {
            self.test_output.push(c).unwrap();
            Ok(())
        }
    }

    #[test]
    fn format() {
        let mut writer = TestWriter::default();
        let args = format_args!("test");
        let record = Record::builder()
            .level(log::Level::Info)
            .target("partition")
            .file(Some("main.rs"))
            .line(Some(33))
            .args(args)
            .build();
        assert!(write(&mut writer, &record).is_ok());
        assert_eq!(
            writer.test_output,
            "partition: \u{1b}[39m\u{1b}[32mINFO\u{1b}[39m test\n",
        );
    }
}
