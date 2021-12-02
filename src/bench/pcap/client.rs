use std::time::{SystemTime, UNIX_EPOCH};

use crate::eth;
use crate::mac;

use crate::bench::pcap::PcapBenchConfiguration;
use crate::bench::{BenchResult, BenchResultData};

use super::find_device;

pub struct PcapClient { }

impl PcapClient {
    pub fn new() -> PcapClient {
        PcapClient { }
    }

    pub fn run(self, config: &PcapBenchConfiguration) -> BenchResult {
        println!("Running client with configuration {:#?}", config);

        let src_addr = mac::get_hardware_address(&config.src_iface).ok_or("Failed to retrieve mac address")?;
        let device = find_device(&config.src_iface).ok_or("Failed to find device")?;

        let mut capture = device.open()?;

        let mut buffer = [0u8; eth::ETH_LEN];

        eth::writer::write_source_addr(&mut buffer, &src_addr);
        eth::writer::write_destination_addr(&mut buffer, &config.dst_addr);
        eth::writer::write_ether_type(&mut buffer, super::ETHER_TYPE);

        for i in 0..config.packets_to_send {
            let seq = i as u16;

            let utc_now = SystemTime::now();
            let nano_epoch = utc_now
                .duration_since(UNIX_EPOCH)?
                .as_nanos();

            // Copy sequence number
            eth::writer::slice_payload(&mut buffer, 0, 2)
                .copy_from_slice(&seq.to_le_bytes());

            // Copy timestamp
            eth::writer::slice_payload(&mut buffer, 2, std::mem::size_of_val(&nano_epoch))
                .copy_from_slice(&nano_epoch.to_le_bytes());

            if config.verbose {
                println!("Sending {:x?}", &buffer);
            }

            capture.sendpacket(buffer)?;

            if i > 0 && i % 1_00 == 0 {
                println!("Sent {} packets", i);
            }
        }

        Ok(
            BenchResultData{
                percentiles: Vec::new()
            }
        )
    }
}
