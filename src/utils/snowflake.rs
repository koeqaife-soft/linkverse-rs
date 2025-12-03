use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task_local;

const EPOCH: u64 = 1_725_513_600_000;
const COUNTER_BITS: u64 = 12;
const PID_BITS: u64 = 5;
const SID_BITS: u64 = 5;

#[derive(Debug)]
pub struct SnowflakeGenerator {
    last_timestamp: AtomicU64,
    counter: AtomicU64,
    server_id: u8,
}

impl SnowflakeGenerator {
    pub fn new(server_id: u8) -> Self {
        Self {
            last_timestamp: AtomicU64::new(0),
            counter: AtomicU64::new(0),
            server_id,
        }
    }

    fn current_time_ms() -> u64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        now.as_millis() as u64
    }

    pub async fn generate(&self) -> u64 {
        todo!("Separate generator between worker threads");

        let tid = 0;
        let mut ts = Self::current_time_ms() - EPOCH;

        let mut counter: u64;

        loop {
            let last_ts = self.last_timestamp.load(Ordering::Relaxed);
            if ts == last_ts {
                counter = self.counter.fetch_add(1, Ordering::Relaxed);
                if counter >= (1 << COUNTER_BITS) {
                    ts = Self::current_time_ms() - EPOCH;
                    self.counter.store(0, Ordering::Relaxed);
                    continue;
                }
            } else {
                counter = 0;
                self.counter.store(0, Ordering::Relaxed);
            }
            self.last_timestamp.store(ts, Ordering::Relaxed);
            break;
        }

        (ts << (COUNTER_BITS + SID_BITS + PID_BITS))
            | ((tid & ((1 << PID_BITS) - 1)) << (COUNTER_BITS + SID_BITS))
            | ((self.server_id as u64 & ((1 << SID_BITS) - 1)) << COUNTER_BITS)
            | counter
    }

    pub fn parse(id: u64) -> (f64, u8, u8, u16) {
        let ts = (id >> (COUNTER_BITS + SID_BITS + PID_BITS)) + EPOCH;
        let pid = ((id >> (COUNTER_BITS + SID_BITS)) & ((1 << PID_BITS) - 1)) as u8;
        let sid = ((id >> COUNTER_BITS) & ((1 << SID_BITS) - 1)) as u8;
        let counter = (id & ((1 << COUNTER_BITS) - 1)) as u16;
        (ts as f64 / 1000.0, sid, pid, counter)
    }
}
