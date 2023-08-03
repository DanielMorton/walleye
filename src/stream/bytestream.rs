use crate::stream::reader::Reader;
use crate::stream::writer::Writer;
use std::cmp::min;

struct ByteStream {
    capacity: usize,
    total_bytes: usize,
    popped_bytes: usize,
    buffer: String,
    closed: bool,
    error: bool,
}

impl ByteStream {
    fn new(capacity: usize) -> Self {
        ByteStream {
            capacity,
            total_bytes: 0,
            popped_bytes: 0,
            buffer: String::with_capacity(capacity),
            closed: false,
            error: false,
        }
    }
}

impl Writer for ByteStream {
    fn push(&mut self, data: String) {
        let max_pos = min(data.len(), self.available_capacity());
        self.buffer += &data[..max_pos];
        self.total_bytes += max_pos;
    }

    fn close(&mut self) {
        self.closed = true;
    }

    fn set_error(&mut self) {
        self.error = true
    }

    fn is_closed(&self) -> bool {
        self.closed
    }

    fn available_capacity(&self) -> usize {
        self.capacity - &self.buffer.len()
    }

    fn bytes_pushed(&self) -> usize {
        self.total_bytes
    }
}

impl Reader for ByteStream {
    fn peek(&self) -> &String {
        return &self.buffer;
    }

    fn drain(&mut self, range: usize) -> String {
        self.popped_bytes += range;
        self.buffer.drain(..range).collect()
    }

    fn is_finished(&self) -> bool {
        self.closed && self.buffer.is_empty()
    }

    fn has_error(&self) -> bool {
        self.error
    }

    fn bytes_buffered(&self) -> usize {
        self.total_bytes
    }

    fn popped_bytes(&self) -> usize {
        self.popped_bytes
    }
}
