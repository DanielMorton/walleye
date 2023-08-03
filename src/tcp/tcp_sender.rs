use crate::integer::Wrap32;
use crate::stream::Reader;
use crate::tcp::tcp_receiver_message::TCPReceiverMessage;
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
    seqnos_in_flight: u16,
    messages: VecDeque<TCPSenderMessage>,
    messages_outstanding: VecDeque<TCPSenderMessage>,
    receiver_window_size: u16,
    receiver_free_space: u16,
    fin_sent: bool,
}

impl TCPSender {
    pub fn consecutive_retransmissions(&self) -> usize {
        self.consecutive_retransmissions
    }

    pub fn push(&mut self, outbound_stream: &mut impl Reader) {
        if !&self.fin_sent {
            let mut is_first_payload = self.seqno_absolute == 0;
            let mut stream_finished = outbound_stream.is_finished();
            if (self.receiver_window_size == 0 && !stream_finished) {
                &self.push_if_window_empty(&is_first_payload, outbound_stream);
                return;
            }
            let length = (min(
                (&self.receiver_window_size - &self.seqnos_in_flight) as usize,
                outbound_stream.bytes_buffered(),
            ) - is_first_payload as usize);
            let mut buffer = outbound_stream.drain(length);
            let mut is_last_payload = stream_finished;

            while !buffer.is_empty() {
                let payload = buffer.drain(..1000).collect::<String>();
                is_last_payload = stream_finished
                    && buffer.is_empty()
                    && (is_first_payload as u16 + payload.len() as u16 + self.seqnos_in_flight
                        < self.receiver_window_size);
                &self.push_message(&is_first_payload, payload, &is_last_payload);
                is_first_payload = false
            }
            stream_finished = outbound_stream.is_finished();
            if stream_finished {
                if self.receiver_window_size == 0 && !is_first_payload {
                    self.push_empty(&is_first_payload, &is_last_payload);
                    self.fin_sent = stream_finished;
                    return;
                } else if self.receiver_has_room(&is_first_payload, &stream_finished) {
                    if !self.messages.is_empty() {
                        self.piggyback()
                    } else {
                        &self.push_empty(&is_first_payload, &is_last_payload);
                        self.fin_sent = stream_finished
                    }
                    return;
                }
            }
            if is_first_payload {
                is_last_payload =
                    (is_first_payload as u16 + is_last_payload as u16) < self.receiver_window_size;
                self.push_empty(&is_first_payload, &is_last_payload);
            }
        }
    }

    pub fn receive(&mut self, msg: &TCPReceiverMessage) {
        self.receiver_window_size = msg.window_size;
        self.receiver_free_space = msg.window_size;
        match msg.ackno {
            None => {}
            Some(ackno_wrap) => {
                let ackno = ackno_wrap.unwrap(&self.isn, &self.seqno_absolute);
                if ackno <= self.seqno_absolute && ackno >= self.ackno_absolute {
                    self.ackno_absolute = ackno;
                    let mut ackno_update = false;
                    while !&self.messages_outstanding.is_empty() {
                        let message = &self.messages_outstanding.pop_front().unwrap();
                        if message.seqno.unwrap(&self.isn, &self.seqno_absolute)
                            + message.sequence_length() as usize
                            <= self.ackno_absolute
                        {
                            self.seqnos_in_flight -= message.sequence_length();
                            ackno_update = true;
                        } else {
                            break;
                        }
                    }
                    if self.messages_outstanding.is_empty() {
                        self.timer.stop()
                    }
                    if !ackno_update {
                        self.timer.reset();
                        self.consecutive_retransmissions = 0
                    }
                }
            }
        }
    }

    pub fn send(&mut self) -> Option<TCPSenderMessage> {
        if self.can_send() {
            match &self.messages.front() {
                None => None,
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

    pub fn send_empty(&self) -> TCPSenderMessage {
        let start = Wrap32::wrap(&self.seqno_absolute, &self.isn);
        return TCPSenderMessage {
            seqno: start,
            syn: false,
            payload: "".to_string(),
            fin: false,
        };
    }

    pub fn seqnos_in_flight(&self) -> u16 {
        self.seqnos_in_flight
    }

    pub fn tick(&mut self, ms_since_last_tick: usize) {
        if self.timer.increment(ms_since_last_tick) {
            if self.receiver_window_size > 0 {
                self.timer.double_rto()
            } else {
                self.timer.reset()
            }
            self.consecutive_retransmissions += 1;
            self.messages
                .push_back(self.messages_outstanding.front().unwrap().clone())
        }
    }

    fn can_send(&self) -> bool {
        !(self.consecutive_retransmissions > 0
            && self.timer.is_running()
            && self.timer.elapsed_time() > 0)
    }

    fn piggyback(&mut self) {
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

    fn push_empty(&mut self, &is_first_payload: &bool, &is_last_payload: &bool) {
        self.push_message(&is_first_payload, String::from(""), &is_last_payload)
    }

    fn push_if_window_empty(
        &mut self,
        &is_first_payload: &bool,
        outbound_stream: &mut impl Reader,
    ) {
        if self.seqnos_in_flight() > 0 {
        } else if is_first_payload {
            self.push_empty(&is_first_payload, &false)
        } else {
            let length = min(1, outbound_stream.bytes_buffered());
            let buffer = outbound_stream.peek()[..length].to_string();
            let is_last_payload = buffer.is_empty();
            self.push_message(&is_first_payload, buffer, &is_last_payload);
            outbound_stream.drain(length);
        }
    }

    fn push_message(&mut self, &is_first_payload: &bool, payload: String, &is_last_payload: &bool) {
        let start = Wrap32::wrap(&self.seqno_absolute, &self.isn);
        let message = TCPSenderMessage {
            seqno: start,
            syn: is_first_payload,
            payload,
            fin: is_last_payload,
        };
        self.fin_sent = is_last_payload;
        let seq_len = message.sequence_length();
        self.seqno_absolute += seq_len as usize;
        self.seqnos_in_flight += seq_len;
        self.messages.push_back(message.clone());
        self.messages_outstanding.push_back(message);
        self.timer.start()
    }

    fn receiver_has_room(&self, &is_first_payload: &bool, &stream_finished: &bool) -> bool {
        self.seqnos_in_flight + (is_first_payload as u16) + (stream_finished as u16)
            < self.receiver_window_size
    }
}
