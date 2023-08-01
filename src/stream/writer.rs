pub trait Writer {
    fn push(&mut self, data: String) -> ();

    fn close(&mut self) -> ();

    fn set_error(&mut self) -> ();

    fn is_closed(&self) -> bool;

    fn available_capacity(&self) -> usize;

    fn bytes_pushed(&self) -> usize;
}
