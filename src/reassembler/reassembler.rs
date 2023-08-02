use crate::reassembler::ReassemblerBuffer;
use crate::stream::Writer;
use std::cmp::{max, min};

pub struct Reassembler {
    write_index: usize,
    max_index: usize,
    buffer: ReassemblerBuffer,
    last_index_read: bool,
}

impl Reassembler {
    fn new() -> Self {
        Reassembler {
            write_index: 0,
            max_index: 0,
            buffer: ReassemblerBuffer::new(),
            last_index_read: false,
        }
    }

    fn bytes_pending(&self) -> usize {
        self.buffer.bytes_pending()
    }

    pub fn insert(
        &mut self,
        first_index: usize,
        data: String,
        is_last_substring: bool,
        output: &impl Writer,
    ) -> () {
        self.last_index_read |= is_last_substring;
        self.buffer.resize(output.available_capacity());
        let read_length = min(data.len(), output.available_capacity());
        if self.buffer.is_empty() && first_index < &self.write_index + &self.buffer.len() {
            self.write_to_buffer(&first_index, &read_length, &data)
        }
        let last_index = first_index + read_length;
        self.max_index = max(self.max_index, last_index)
    }

    fn write_to_buffer(&mut self, first_index: &usize, read_length: &usize, data: &String) -> () {
        let data_offset = if first_index > &self.write_index {
            0
        } else {
            &self.write_index - first_index
        };
        if data_offset < data.len() {
            let buffer_offset = first_index + data_offset - &self.write_index;
            let buffer_limit = min(
                min(data.len(), data_offset + read_length),
                &self.buffer.len() + &self.write_index - first_index,
            );
            self.buffer.replace(
                &data[data_offset..buffer_limit - data_offset],
                buffer_offset,
            )
        }
    }

    fn write_to_stream(&mut self, output: &mut impl Writer) -> () {
        let bytes_to_write = self.buffer.pop();
        if !bytes_to_write.is_empty() {
            self.write_index += bytes_to_write.len();
            output.push(bytes_to_write)
        }
    }
}
