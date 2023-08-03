pub struct Timer {
    is_running: bool,
    initial_rto: usize,
    rto: usize,
    elapsed_time: usize,
}

impl Timer {
    pub fn new(initial_rto: usize) -> Self {
        Timer {
            is_running: false,
            initial_rto,
            rto: initial_rto,
            elapsed_time: 0,
        }
    }

    pub fn double_rto(&mut self) -> () {
        self.rto *= 2;
        self.elapsed_time = 0
    }

    pub fn elapsed_time(&self) -> usize {
        self.elapsed_time
    }

    pub fn increment(&mut self, ms_since_last_tick: usize) -> bool {
        if self.is_running {
            self.elapsed_time += ms_since_last_tick;
            &self.rto <= &self.elapsed_time
        } else {
            false
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn reset(&mut self) -> () {
        self.rto = self.initial_rto;
        self.elapsed_time = 0;
    }

    pub fn start(&mut self) -> () {
        if !self.is_running {
            self.is_running = true;
            self.reset()
        }
    }

    pub fn stop(&mut self) -> () {
        self.is_running = false
    }
}
