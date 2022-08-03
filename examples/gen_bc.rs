use std::env::current_dir;

use chain_rs::{Blockchain, SledDb};

fn main() {
    tracing_subscriber::fmt().init();

    let path = current_dir().unwrap().join("data");
    let mut bc = Blockchain::new(SledDb::new(path));

    bc.mine_block("Jason -> Phil 2 btc");
    bc.mine_block("Jason -> Tony 2 btc");

    bc.blocks_info();
}
