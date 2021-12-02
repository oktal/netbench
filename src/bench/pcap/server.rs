use std::time::{SystemTime, UNIX_EPOCH};

use super::{PcapBenchConfiguration, find_device};
use crate::bench::{BenchResult, BenchResultData, PercentileValue};

use crate::eth;
use crate::mac;

use hdrhistogram::Histogram;

const MIN_LATENCY_NS: u64 = 10;
const MAX_LATENCY_NS: u64 = 10 * 1_000_000;

pub struct PcapServer { }

impl PcapServer {
    pub fn new() -> PcapServer {
        PcapServer {
        }
    }

    pub fn run(&self, config: &PcapBenchConfiguration) -> BenchResult {
        println!("Running server with configuration {:#?}", config);

        let src_addr = mac::get_hardware_address(&config.src_iface).ok_or("Failed to retrieve mac address")?;
        let device = find_device(&config.src_iface).ok_or("Failed to find device")?;

        let mut capture = device.open()?;

        let filter = &format!("ether src host {} and ether dst host {}", &src_addr, &config.dst_addr);
        println!(r#"Using filter "{}""#, filter);

        capture.filter(&filter, true)?;

        let mut packets_received = 0usize;

        let mut hist = Histogram::<u64>::new_with_bounds(MIN_LATENCY_NS, MAX_LATENCY_NS, 3)?;

        let mut cur_seq = 0;

        loop {
            let packet = capture.next()?;
            let ether_type = eth::reader::read_ether_type(packet.data).ok_or("Failed to read ether type")?;
            if ether_type != super::ETHER_TYPE {
                if config.verbose {
                    println!("Skipping frame with ethtype {}", ether_type);
                    continue;
                }
            }

            let seq = u16::from_le_bytes(
                eth::reader::slice_payload(&packet.data, 0, 2).try_into()?
            );

            if seq <= cur_seq {
                return Err(format!("Received sequence {} <= current sequence {}", seq, cur_seq).into());
            }

            cur_seq = seq;

            let timestamp = u64::from_le_bytes(
                eth::reader::slice_payload(&packet.data, 2, std::mem::size_of::<u64>()).try_into()?
            );

            let utc_now = SystemTime::now();
            let nano_epoch = utc_now
                .duration_since(UNIX_EPOCH)?;

            let latency =  nano_epoch - std::time::Duration::from_nanos(timestamp);
            hist.record(latency.as_nanos() as u64)?;

            packets_received += 1;

            if packets_received % 1_00 == 0 {
                println!("Received {} packets", packets_received);
            }

            if packets_received == config.packets_to_send {
                break;
            }
        }

        let percentiles = hist
            .iter_recorded()
            .map(|v| PercentileValue(v.percentile(), v.value_iterated_to()))
            .collect::<Vec<_>>();

        Ok(BenchResultData{ percentiles })
    }
}
