mod bench;
mod eth;
mod mac;

use crate::mac::MacAddress;

const NET_INTERFACE: &'static str = "wlp2s0";
const DEST_ADDR: MacAddress = MacAddress([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 1 {
        println!("usage netbench [client|server]");
        return;
    }

    let config = bench::pcap::PcapBenchConfiguration{
        src_iface: NET_INTERFACE.into(),
        dst_addr: DEST_ADDR,
        packets_to_send: 1_000_000,
        verbose: true
    };

    let bench_mode = &args[0];
    let bench_result = {
        if bench_mode == "client" {
            let pcap_client = bench::pcap::client::PcapClient::new();
            pcap_client.run(&config)
        } else if bench_mode == "server" {
            let pcap_server = bench::pcap::server::PcapServer::new();
            pcap_server.run(&config)
        } else {
            Err(format!("invalid bench mode {}", bench_mode).into())
        }
    };
    match bench_result {
        Ok(result) => {
            println!("{:?}", result);

        },
        Err(e) => println!("Failed to run bench: {}", e),
    }
}
