//! Colors writable type characters with the ascii rainbow
//!
//! You can use this any type that requires a `io::Write` type
//!
//! # examples
//!
//! ```rust
//! use std::io::Write;
//! write!(
//!   &mut rainbowcoat::stdout(),
//!   "hello rainbow"
//! );
//! ```
extern crate  ansi_term;

use std::f64::consts::PI;
use std::io::{self, Stdout, Write};

use ansi_term::Color;

/// Colors writable type characters with the ascii rainbow
///
/// You can use this any type that requires a `io::Write` type
///
/// # examples
///
/// Write direcly to stdout
///
/// ```rust
/// use std::io::{self, Write};
/// write!(
///   &mut rainbowcoat::Colors::new(io::stdout()),
///   "hello rainbow"
/// );
/// ```
///
/// A convenience function makes this common case even easier
///
/// ```rust
/// use std::io::Write;
/// write!(
///   &mut rainbowcoat::stdout(),
///   "hello rainbow"
/// );
/// ```
pub struct Colors<W> {
    wrapped: W,
    spread: f64,
    frequency: f64,
    seed: f64,
}

/// Convenience function that wraps `std::io::Stdout` in a`Colors` instance
pub fn stdout() -> Colors<Stdout> {
    Colors::new(io::stdout())
}

impl<W> Colors<W> {
    /// Creates a new `Colors` instance with default spread of `3.0`,
    /// frequency of `0.1` and seed valud of `0.0`
    pub fn new(wrapped: W) -> Self {
        Self::configure(wrapped, 3.0, 0.1, 0.0)
    }

    /// Creates a more configurable instance with provided params
    pub fn configure(wrapped: W, spread: f64, frequency: f64, seed: f64) -> Self {
        Self {
            wrapped,
            spread,
            frequency,
            seed,
        }
    }

    fn color(&mut self, seed: f64) -> Color {
        let i = self.frequency * seed / self.spread;
        let red = i.sin() * 127.00 + 128.00;
        let green = (i + (PI * 2.00 / 3.00)).sin() * 127.00 + 128.00;
        let blue = (i + (PI * 4.00 / 3.00)).sin() * 127.00 + 128.00;
        Color::RGB(red as u8, green as u8, blue as u8)
    }
}

impl<W: Write> Write for Colors<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for line in ::std::str::from_utf8(buf).unwrap().lines() {
            let mut seed = self.seed;
            for c in line.chars() {
                seed += 1.0;
                // fixme: Color#paint doesn't work with scalar char types
                let mut s = String::with_capacity(1);
                s.push(c);
                let color = self.color(seed);
                write!(&mut self.wrapped, "{}", color.paint(s))?
            }
            write!(&mut self.wrapped, "\n")?;
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.wrapped.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::{stdout, Colors};
    use ansi_term::Color;
    use std::io::{self, Write};

    #[test]
    fn colors_colors() {
        assert_eq!(Colors::new(io::sink()).color(0.0), Color::RGB(128, 237, 18))
    }

    #[test]
    fn colors_writes() {
        assert!(write!(&mut Colors::new(Vec::new()), "hello").is_ok());
    }

    #[test]
    fn stdout_convenience() {
        assert!(write!(&mut stdout(), "hello").is_ok());
    }
}
