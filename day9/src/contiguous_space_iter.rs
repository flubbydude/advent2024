#[derive(Debug)]
pub enum ContiguousDiskSpace {
    File(DiskFile),
    EmptySpace(EmptySpace),
}

#[derive(Debug)]
pub struct DiskFile {
    pub id: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct EmptySpace {
    pub length: usize,
}

pub struct ContiguousDiskSpaceIter<'a> {
    input: &'a [u8],
    cur_index: usize,
}

impl<'a> ContiguousDiskSpaceIter<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            cur_index: 0,
        }
    }
}

impl Iterator for ContiguousDiskSpaceIter<'_> {
    type Item = ContiguousDiskSpace;

    fn next(&mut self) -> Option<Self::Item> {
        let length = loop {
            let length = *self.input.get(self.cur_index)? as usize;
            if length != 0 {
                break length;
            }
            self.cur_index += 1;
        };
        if self.cur_index % 2 == 0 {
            let result = ContiguousDiskSpace::File(DiskFile {
                id: self.cur_index / 2,
                length,
            });
            self.cur_index += 1;
            return Some(result);
        }

        let mut length = length;
        loop {
            self.cur_index += 1;
            if let Some(&0) = self.input.get(self.cur_index) {
                self.cur_index += 1;
                if let Some(&next_len) = self.input.get(self.cur_index) {
                    length += next_len as usize;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Some(ContiguousDiskSpace::EmptySpace(EmptySpace { length }))
    }
}
