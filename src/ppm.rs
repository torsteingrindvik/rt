use std::{
    io::{BufWriter, Write},
    path::Path,
};

use tracing::debug;

/// Data is RGB 8-bit per channel.
pub fn write(rows: usize, data: impl AsRef<[u8]>, writer: &mut impl Write) -> anyhow::Result<()> {
    let data = data.as_ref();
    let num_bytes = data.len();
    let cols = num_bytes / rows / 3;

    assert_eq!(
        cols * rows * 3,
        num_bytes,
        "cols and rows should fit exactly with no padding etc."
    );

    writer.write_all(b"P3\n")?;
    writer.write_all(format!("{cols} {rows}\n").as_bytes())?;
    writer.write_all(format!("255\n").as_bytes())?;

    let rows: Vec<_> = data.chunks_exact(3 * cols).collect();

    for (index, row) in rows.iter().enumerate() {
        debug!("writing row {}/{}", index + 1, rows.len());
        for rgb in row.chunks_exact(3) {
            let [r, g, b] = rgb.try_into()?;
            writer.write_all(format!("{r} {g} {b} ").as_bytes())?;
        }
        writer.write_all(b"\n")?;
    }

    Ok(())
}

/// Data is RGB 8-bit per channel.
pub fn write_pathlike(
    rows: usize,
    data: impl AsRef<[u8]>,
    pathlike: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let mut out = BufWriter::new(std::fs::File::create(pathlike.as_ref())?);

    write(rows, data, &mut out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() -> anyhow::Result<()> {
        let data = [100, 0, 0, 0, 100, 0, 0, 0, 0, 100, 100, 100];

        // to mem
        let mut writer = vec![];

        write(2, &data, &mut writer)?;

        let s = String::from_utf8(writer)?;
        dbg!(s);

        // to file
        write_pathlike(2, &data, "simple.ppm")?;

        Ok(())
    }

    #[test]
    fn book_example() -> anyhow::Result<()> {
        let mut buf = vec![];
        for row in 0..256 {
            for col in 0..256 {
                buf.extend([col as u8, row as u8, 0]);
            }
        }

        write_pathlike(256, &mut buf, "book_example.ppm")?;

        Ok(())
    }
}
