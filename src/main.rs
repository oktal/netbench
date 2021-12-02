mod bench;
mod eth;
mod mac;

use crate::mac::MacAddress;

const NET_INTERFACE: &'static str = "wlp2s0";
const SRC_ADDR: MacAddress = MacAddress([0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xdd]);
const DEST_ADDR: MacAddress = MacAddress([0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xdd]);

const PACKETS_TO_SEND: usize = 1_000;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 1 {
        println!("usage netbench [client|server]");
        return;
    }

    let bench_mode = &args[0];
    let bench_result = {
        if bench_mode == "client" {
            let config = bench::pcap::PcapBenchConfiguration{
                src_iface: NET_INTERFACE.into(),
                src_addr: SRC_ADDR,
                dst_addr: DEST_ADDR,
                packets_to_send: PACKETS_TO_SEND,
                verbose: false
            };

            let pcap_client = bench::pcap::client::PcapClient::new();
            pcap_client.run(&config)
        } else if bench_mode == "server" {
            let config = bench::pcap::PcapBenchConfiguration{
                src_iface: NET_INTERFACE.into(),
                src_addr: SRC_ADDR,
                dst_addr: DEST_ADDR,
                packets_to_send: PACKETS_TO_SEND,
                verbose: false
            };

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
