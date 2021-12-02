use std::vec::Vec;

#[derive(Debug, Copy, Clone)]
pub struct PercentileValue(f64, u64);

impl PercentileValue {
    fn nth(&self) ->  f64 {
        self.0
    }

    fn value(&self) -> u64 {
        self.1
    }
}

#[derive(Debug)]
pub struct BenchResultData {
    percentiles: Vec<PercentileValue>
}

pub type BenchResult = std::result::Result<BenchResultData, Box<dyn std::error::Error>>;

pub mod pcap;
