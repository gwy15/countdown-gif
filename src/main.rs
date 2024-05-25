use anyhow::Result;
use countdown_gif::Position;

fn main() -> Result<()> {
    let delta = 86400 * 3 + 72;

    let files = [
        "subsec.gif",
        "second.gif",
        "minute.gif",
        "hour.gif",
        "day.gif",
    ];
    let pos = [
        Position::Subsec,
        Position::Second,
        Position::Minute,
        Position::Hour,
        Position::Day,
    ];
    for (file, pos) in files.iter().zip(pos.iter()) {
        let output = std::fs::File::create(format!("tmp/{file}"))?;
        countdown_gif::generate(delta, *pos, output)?;
    }

    Ok(())
}
