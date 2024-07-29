use std::{
    fs::{File, OpenOptions},
    io::{Error, Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::{node_type::Offset, page::Page, page_layout::PAGE_SIZE};

pub struct Pager {
    file: File,
    cursor: usize,
}

impl Pager {
    pub fn new(path: &Path) -> Result<Pager, Error> {
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        Ok(Pager { file: fd, cursor: 0 })
    }

    pub fn read_page(&mut self, offset: &Offset) -> Result<Page, Error> {
        let mut buffer = [0x00; PAGE_SIZE];
        self.file.seek(SeekFrom::Start(offset.0 as u64))?;
        self.file.read_exact(&mut buffer)?;
        Ok(Page::new(buffer))
    }

    pub fn write_page(&mut self,  page: &Page) -> Result<Offset, Error> {
        self.file.seek(SeekFrom::Start(self.cursor as u64))?;
        self.file.write_all(&page.get_data())?;
        let res = Offset(self.cursor);
        self.cursor += PAGE_SIZE;
        Ok(res)
    }
}
