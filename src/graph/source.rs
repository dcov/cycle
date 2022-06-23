use anyhow::{self, Context};
use std::fs;
use std::path::Path;

use super::history::History;
use super::working::Working;

pub struct Source {
    path: Box<Path>,
    working: Working,
}

impl Source {
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        let bytes = fs::read(path).with_context(|| "Error reading source file")?;
        let history = read_bytes(&bytes)?;
        Ok(Self {
            path: Box::from(path),
            working: Working::new(history),
        })
    }

    pub fn write(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

// --- FILE FORMAT IMPLEMENTATION ---

const CURRENT_VERSION: u32 = 0;
const NUM_VERSION_BYTES: usize = 4;
const FILE_HEADING_NAME: &'static [u8] = "Cycle Graph-Source File".as_bytes();
const MIN_FILE_HEADING_BYTES: usize = FILE_HEADING_NAME.len() + NUM_VERSION_BYTES;
const FILE_HEADING_VERSION_OFFSET: usize = FILE_HEADING_NAME.len();
const FIRST_COMMIT_BYTE_OFFSET: usize = FILE_HEADING_VERSION_OFFSET + NUM_VERSION_BYTES;

fn read_bytes(bytes: &Vec<u8>) -> anyhow::Result<History> {
    anyhow::ensure!(
        bytes.len() >= MIN_FILE_HEADING_BYTES,
        "Invalid file heading"
    );

    for i in 0..FILE_HEADING_NAME.len() {
        anyhow::ensure!(bytes[i] == FILE_HEADING_NAME[i], "Invalid file heading");
    }

    let version = u32::from_be_bytes(
        bytes[FILE_HEADING_VERSION_OFFSET..(FILE_HEADING_VERSION_OFFSET + NUM_VERSION_BYTES)]
            .try_into()
            .unwrap(),
    );
    anyhow::ensure!(version == 0, "Unsupported version");

    if !(bytes.len() != MIN_FILE_HEADING_BYTES) {
        return Ok(History::new());
    }

    for b in bytes[FIRST_COMMIT_BYTE_OFFSET..bytes.len()].iter() {}

    Ok(History::new())
}

fn write_bytes(path: &Path, opt_history: Option<History>) -> anyhow::Result<Vec<u8>> {
    let result = Vec::<u8>::new();

    if opt_history.is_none() {
        fs::write(
            path,
            [FILE_HEADING_NAME, &CURRENT_VERSION.to_be_bytes()].concat(),
        )
        .with_context(|| "Error creating source file")?;

        return Ok(result);
    }

    let history = opt_history.unwrap();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{read_bytes, write_bytes};

    #[test]
    pub fn test_write_read_heading() {}
}
