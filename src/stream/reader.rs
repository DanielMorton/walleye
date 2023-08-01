pub trait Reader {
    fn peek(&self) -> &String;

    fn drain(&mut self, range: usize) -> String;

    fn is_finished(&self) -> bool;

    fn has_error(&self) -> bool;

    fn bytes_buffered(&self) -> usize;

    fn popped_bytes(&self) -> usize;
}
