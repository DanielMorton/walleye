use crate::integer::Wrap32;

#[derive(Clone)]
pub struct TCPSenderMessage {
    pub seqno: Wrap32,
    pub syn: bool,
    pub payload: String,
    pub fin: bool,
}

impl TCPSenderMessage {
    pub fn sequence_length(&self) -> u16 {
        self.syn as u16 + self.payload.len() as u16 + self.fin as u16
    }
}
