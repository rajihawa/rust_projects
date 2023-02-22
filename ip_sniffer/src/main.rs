use clap::Parser;
use futures::future::join_all;
use std::{
    net::{IpAddr, Ipv4Addr},
    ops::RangeInclusive,
    time::{Duration, Instant},
};
use tokio::task;
use tokio::{net::TcpStream, time::timeout};

const MAX_PORT: u16 = 65535;
const MIN_PORT: u16 = 1;

const PORT_RANGE: RangeInclusive<u16> = MIN_PORT..=MAX_PORT;

#[derive(Debug, Parser)]
#[command(author = "Raji H. <rajihawa@gmail.com>")]
#[command(about = "Scans all port in specified IP", long_about = None)]
struct Arguments {
    #[arg(short = 'i', long = "ip-address", default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    ip_address: IpAddr,
    #[arg(short, long, default_value_t = MIN_PORT, value_parser = validate_port)]
    start: u16,
    #[arg(short, long, default_value_t = MAX_PORT, value_parser = validate_port)]
    end: u16,
}

fn validate_port(s: &str) -> Result<u16, &'static str> {
    match s.parse::<u16>() {
        Ok(port) => {
            if PORT_RANGE.contains(&port) {
                Ok(port)
            } else {
                Err("Invalid port number")
            }
        }
        Err(_) => Err("Port isn't a number"),
    }
}

async fn scan_port(ip_address: IpAddr, port: u16) -> Option<u16> {
    let timeout_duration = Duration::from_secs(5);
    let connect_task = TcpStream::connect((ip_address, port));
    let result = timeout(timeout_duration, connect_task).await;
    match result {
        Ok(res) => {
            res.map_err(|e| println!("{:?}", e));
            Some(port)
        }
        Err(_) => None,
    }
}

#[tokio::main]
async fn main() {
    // let args = env::args();

    let opts = Arguments::parse();
    println!("Options: {:?}", opts);
    // let a = Arguments::new(args).unwrap_or_else(|err| match err {
    //     "help" => {
    //         process::exit(0);
    //     }
    //     err => {
    //         eprintln!("Error: {}", err);
    //         process::exit(1);
    //     }
    // });

    let start_time = Instant::now();
    let mut tasks = vec![];

    for i in opts.start..=opts.end {
        let handle = task::spawn(async move {
            match scan_port(opts.ip_address, i).await {
                Some(_) => {
                    println!("found port {}", i);
                }
                None => {}
            }
        });
        tasks.push(handle)
    }
    join_all(tasks.into_iter()).await;
    let duration = Instant::now() - start_time;
    println!("Elapsed time: {:?}", duration);
}
