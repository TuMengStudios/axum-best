use human_panic::setup_panic;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    setup_panic!();
    panic!("test panic");
}
