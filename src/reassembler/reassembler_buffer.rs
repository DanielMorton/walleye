use std::collections::VecDeque;

pub(crate) struct ReassemblerBuffer {
    buffer: String,
    bitmap: VecDeque<bool>,
    pending: usize,
}

impl ReassemblerBuffer {
    pub(crate) fn new() -> Self {
        ReassemblerBuffer {
            buffer: String::new(),
            bitmap: VecDeque::new(),
            pending: 0,
        }
    }

    pub(crate) fn bytes_pending(&self) -> usize {
        self.pending
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.buffer.len()
    }

    pub(crate) fn pop(&mut self) -> String {
        match self.bitmap.iter().position(|&r| !r) {
            None => {
                self.pending = 0;
                let _ = &self.bitmap.clear();
                self.buffer.drain(..).collect()
            }
            Some(pos) => {
                self.pending -= &pos;
                let _ = &self.bitmap.drain(..&pos);
                self.buffer.drain(..&pos).collect()
            }
        }
    }

    pub(crate) fn replace(&mut self, data: &str, buffer_offset: usize) -> () {
        let max_offset = buffer_offset + data.len();
        self.buffer.replace_range(buffer_offset..max_offset, &data);
        for offset in buffer_offset..max_offset {
            if !self.bitmap[offset] {
                self.bitmap[offset] = true;
                self.pending += 1;
            }
        }
    }

    pub(crate) fn resize(&mut self, size: usize) -> () {
        let _ = &self.buffer.push_str(&" ".repeat(size));
        let _ = &self.bitmap.resize(size + &self.bitmap.len(), false);
    }
}
