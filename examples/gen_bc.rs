use chain_rs::Blockchain;

fn main() {
    tracing_subscriber::fmt().init();

    let mut bc = Blockchain::new();

    bc.mine_block("Jason -> Phil 2 btc");
    // bc.mine_block("Jason -> Tony 2 btc");

    bc.blocks_info();
}
