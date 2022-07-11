// Copyright (C) 2022 Nitrokey GmbH
// SPDX-License-Identifier: CC0-1.0

fn main() -> std::io::Result<()> {
    env_logger::init();
    vpicc::connect()?.run(&mut vpicc::DummySmartCard)
}
