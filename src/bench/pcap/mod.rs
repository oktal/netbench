use crate::mac::MacAddress;

use std::string::String;
use pcap::Device;

#[derive(Debug)]
pub struct PcapBenchConfiguration {
    pub src_iface: String,

    pub src_addr: MacAddress,
    pub dst_addr: MacAddress,

    pub packets_to_send: usize,
    pub verbose: bool
}

const ETHER_TYPE: u16 = 0xDEAD;

fn find_device<S: AsRef<str>>(iface: S) -> Option<Device> {
    let devices = Device::list().ok()?;
    devices.into_iter().find(
        |d| d.name == iface.as_ref()
    )
}

pub mod client;
pub mod server;
