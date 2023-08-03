use crate::integer::Wrap32;
use crate::reassembler::Reassembler;
use crate::stream::Writer;
use crate::tcp::tcp_receiver_message::TCPReceiverMessage;
use crate::tcp::tcp_sender_message::TCPSenderMessage;
use std::cmp::min;

struct TCPReceiver {
    syn: bool,
    start: Wrap32,
    ackno: Option<Wrap32>,
}

impl TCPReceiver {
    fn receive(
        &mut self,
        message: TCPSenderMessage,
        reassembler: &mut Reassembler,
        inbound_stream: &impl Writer,
    ) -> () {
        if message.syn || self.syn {
            let first_byte = message.seqno + message.syn as u32;
            if message.syn {
                self.syn = true;
                self.start = first_byte;
            }
            reassembler.insert(
                first_byte.unwrap(&self.start, &inbound_stream.bytes_pushed()),
                message.payload,
                message.fin,
                inbound_stream,
            );
            self.ackno = Some(
                Wrap32::wrap(&inbound_stream.bytes_pushed(), &self.start)
                    + inbound_stream.is_closed() as u32,
            )
        }
    }

    fn send(&self, inbound_stream: &impl Writer) -> TCPReceiverMessage {
        TCPReceiverMessage::new(
            self.ackno.clone(),
            min(u16::MAX as usize, inbound_stream.available_capacity()) as u16,
        )
    }
}
