use crate::integer::Wrap32;

pub struct TCPReceiverMessage {
    pub ackno: Option<Wrap32>,
    pub window_size: u16,
}

impl TCPReceiverMessage {
    pub(crate) fn new(ackno: Option<Wrap32>, window_size: u16) -> Self {
        TCPReceiverMessage { ackno, window_size }
    }
}
