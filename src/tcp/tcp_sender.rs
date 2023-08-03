use crate::integer::Wrap32;
use crate::stream::Reader;
use crate::tcp::tcp_sender_message::TCPSenderMessage;
use crate::timer::Timer;
use std::cmp::min;
use std::collections::VecDeque;

struct TCPSender {
    isn: Wrap32,
    timer: Timer,
    seqno_absolute: usize,
    ackno_absolute: usize,
    consecutive_retransmissions: usize,
    seqnos_in_flight: usize,
    messages: VecDeque<TCPSenderMessage>,
    messages_outstanding: VecDeque<TCPSenderMessage>,
    receiver_window_size: usize,
    receiver_free_space: usize,
    fin_sent: bool,
}

impl TCPSender {
    pub fn send(&mut self) -> Option<TCPSenderMessage> {
        if self.can_send() {
            match &self.messages.front() {
                None => {None}
                Some(message) => {
                    if message.sequence_length() <= self.receiver_free_space
                        || self.consecutive_retransmissions > 0
                        || self.receiver_window_size == 0
                    {
                        self.receiver_free_space -= if self.consecutive_retransmissions > 0 {
                            0
                        } else {
                            message.sequence_length()
                        };
                        self.messages.pop_front()
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    fn can_send(&self) -> bool {
        !(self.consecutive_retransmissions > 0
            && self.timer.is_running()
            && self.timer.elapsed_time() > 0)
    }

    fn piggyback(&mut self) -> () {
        match self.messages.back_mut() {
            None => {}
            Some(mut message) => {
                message.fin = true;
                self.seqno_absolute += 1;
                self.seqnos_in_flight += 1;
                self.fin_sent = true;
                self.timer.start();
                match self.messages_outstanding.back_mut() {
                    None => {}
                    Some(mut message_outstanding) => {
                        message_outstanding.fin = true;
                    }
                }
            }
        }
    }

    fn push_message(
        &mut self,
        is_first_payload: bool,
        payload: String,
        is_last_payload: bool,
    ) -> () {
        let start = Wrap32::wrap(&self.seqno_absolute, &self.isn);
        let message = TCPSenderMessage {
            seqno: start,
            syn: is_first_payload,
            payload,
            fin: is_last_payload,
        };
        self.fin_sent = is_last_payload;
        let seq_len = message.sequence_length();
        self.seqno_absolute += seq_len;
        self.seqnos_in_flight += seq_len;
        self.messages.push_back(message.clone());
        self.messages_outstanding.push_back(message);
        self.timer.start()
    }

    fn push_if_window_empty(
        &mut self,
        is_first_payload: bool,
        outbound_stream: &mut impl Reader,
    ) -> () {
        if self.seqnos_in_flight() > 0 {
        } else if is_first_payload {
            self.push_message(is_first_payload, String::from(""), false)
        } else {
            let length = min(1, outbound_stream.bytes_buffered());
            let buffer = outbound_stream.peek()[..length].to_string();
            let is_last_payload = buffer.is_empty();
            &self.push_message(is_first_payload, buffer, is_last_payload);
            outbound_stream.drain(length);
        }
    }

    fn consecutive_retransmissions(&self) -> usize {
        self.consecutive_retransmissions
    }

    fn seqnos_in_flight(&self) -> usize {
        self.seqnos_in_flight
    }
}
