use super::*;

pub struct ArchiveEditor {
  archive: Archive,
  data: TempDir,
}

impl ArchiveEditor {
  pub fn new(a: Archive, d: TempDir) -> Self {
    ArchiveEditor {
      archive: a,
      data: d,
    }
  }
}
