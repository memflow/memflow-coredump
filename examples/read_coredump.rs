use log::Level;

use memflow::connector::ConnectorArgs;
use memflow_win32::*;

use memflow_coredump::create_connector;

fn main() {
    simple_logger::init_with_level(Level::Debug).unwrap();

    let connector = create_connector(&ConnectorArgs::with_default("./coredump.raw")).unwrap();

    let mut kernel = Kernel::builder(connector).build().unwrap();

    let eprocess_list = kernel.eprocess_list().unwrap();
    println!("eprocess_list.len() = {}", eprocess_list.len());
}
