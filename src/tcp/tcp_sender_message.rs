use crate::integer::Wrap32;

#[derive(Clone)]
pub struct TCPSenderMessage {
    pub seqno: Wrap32,
    pub syn: bool,
    pub payload: String,
    pub fin: bool,
}

impl TCPSenderMessage {
    pub fn sequence_length(&self) -> usize {
        self.syn as usize + &self.payload.len() + self.fin as usize
    }
}
