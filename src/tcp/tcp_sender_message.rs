use crate::integer::Wrap32;

pub struct TCPSenderMessage {
    pub seqno: Wrap32,
    pub syn: bool,
    pub payload: String,
    pub fin: bool
}

impl TCPSenderMessage {
    fn new() -> Self {
        TCPSenderMessage {seqno: Wrap32::new(0),
        syn: false,
        payload: String::from(""),
        fin: false
        }
    }

    fn sequence_length(&self) -> usize {
        &self.syn as usize + &self.payload.len() + &self.fin as usize
    }
}