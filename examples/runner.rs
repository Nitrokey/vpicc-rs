fn main() -> std::io::Result<()> {
    env_logger::init();
    vpicc::connect()?.run(&mut vpicc::DummySmartCard)
}
