use anyhow::{bail, Result};
use std::io::Write;

pub mod frames;

#[derive(Debug, Clone, Copy)]
pub enum Position {
    Subsec,
    Second,
    Minute,
    Hour,
    Day,
}

pub fn gen<W: Write, It>(frames: It, output: W) -> Result<()>
where
    It: Iterator<Item = (u32, f64)> + Send,
{
    let (collector, writer) = gifski::new(gifski::Settings::default())?;
    std::thread::scope(|t| -> Result<()> {
        let frames_thread = t.spawn(move || {
            for (idx, (digit, time)) in frames.enumerate() {
                collector.add_frame_png_data(idx, frames::png_frame(digit).to_vec(), time)?;
            }
            drop(collector);
            Ok(())
        });
        writer.write(output, &mut gifski::progress::NoProgress {})?;
        frames_thread.join().unwrap()
    })?;
    Ok(())
}

struct DigitIter {
    digit: u32,
    start_second: u32,
    frame: u32,
    num_of_sec: u32,
    num_of_self_to_next: u32,
}
impl DigitIter {
    pub fn minute(minute: u32, second: u32) -> Self {
        Self {
            digit: minute,
            start_second: second,
            frame: 0,
            num_of_sec: 60,
            num_of_self_to_next: 60,
        }
    }
    pub fn hour(hour: u32, second: u32) -> Self {
        Self {
            digit: hour,
            start_second: second,
            frame: 0,
            num_of_sec: 3600,
            num_of_self_to_next: 24,
        }
    }
    pub fn day(day: u32, second: u32) -> Self {
        Self {
            digit: day,
            start_second: second,
            frame: 0,
            num_of_sec: 3600 * 24,
            num_of_self_to_next: 365,
        }
    }
}
impl Iterator for DigitIter {
    // digit, second
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        let (frame, digit) = (self.frame, self.digit);
        self.frame += 1;
        self.digit = (self.digit + self.num_of_self_to_next - 1) % self.num_of_self_to_next;
        let item = if frame == 0 {
            (digit, 0)
        } else {
            (digit, self.start_second + self.num_of_sec * (frame - 1))
        };
        Some(item)
    }
}

pub fn generate<W: Write>(countdown_sec: u32, pos: Position, output: W) -> Result<()> {
    const SEC_IN_DAY: u32 = 3600 * 24;
    if countdown_sec > (99 * SEC_IN_DAY + 23 * 3600 + 59 * 60 + 59) {
        bail!("too large");
    }
    match pos {
        Position::Subsec => {
            let iter = (0..100u32)
                .step_by(2)
                .rev()
                .enumerate()
                .map(|(idx, sub)| (sub, 0.02 * (idx + 1) as f64));
            gen(iter, output)
        }
        Position::Second => {
            let s = countdown_sec % 60;
            let iter = (0..60).chain(0..s as u32).rev().take(60);
            let iter = iter.enumerate().map(|(i, s)| (s, (i + 1) as f64));
            gen(iter, output)
        }
        Position::Minute => {
            let s = countdown_sec;
            let (minute, second) = ((s / 60) % 60, s % 60);
            let iter = DigitIter::minute(minute as u32, second as u32)
                .take(120)
                .map(|(d, t)| (d, t as f64));
            gen(iter, output)
        }
        Position::Hour => {
            let s = countdown_sec;
            let (hour, second) = ((s / 3600) % 24, s % 3600);
            let iter = DigitIter::hour(hour as u32, second as u32)
                .take(72)
                .map(|(d, t)| (d, t as f64));
            gen(iter, output)
        }
        Position::Day => {
            let s = countdown_sec;
            let (day, second) = (s / SEC_IN_DAY, s % SEC_IN_DAY);
            let iter = DigitIter::day(day as u32, second as u32)
                .take(3)
                .map(|(d, t)| (d, t as f64));
            gen(iter, output)
        }
    }
}
