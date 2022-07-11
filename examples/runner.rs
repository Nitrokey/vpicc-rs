fn main() {
    env_logger::init();
    vpicc::connect().run(&mut vpicc::DummySmartCard)
}
