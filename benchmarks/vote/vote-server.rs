extern crate clap;

extern crate distributary;

extern crate slog;
extern crate slog_term;

mod graph;

use distributary::srv;

use std::net::ToSocketAddrs;

fn main() {
    use clap::{Arg, App};
    let args = App::new("vote")
        .version("0.1")
        .about("Benchmarks user-curated news aggregator throughput for different storage \
                backends.")
        .arg(Arg::with_name("ADDR")
                 .index(1)
                 .help("Address and port to listen on")
                 .required(true))
        .get_matches();

    let addr = args.value_of("ADDR").unwrap();
    println!("Attempting to start soup on {}", addr);
    let g = graph::make(true,
                        false,
                        Some(distributary::BaseDurabilityLevel::Buffered));

    // start processing
    // TODO: what about the node indices?
    srv::run(g.graph, addr.to_socket_addrs().unwrap().next().unwrap());
}
