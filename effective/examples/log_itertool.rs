use effective::log::{model::ServerLogEntry, read_log::read_log};
use itertools::Itertools;

fn main() {
    let log1 = read_log("./effective/server1.log").unwrap();
    let log2 = read_log("./effective/server2.log").unwrap();

    let log1 = log1.filter_map(|l| ServerLogEntry::try_from(l.ok()?.as_ref()).ok());

    let log2 = log2.filter_map(|l| ServerLogEntry::try_from(l.ok()?.as_ref()).ok());

    let log_final = log1.merge(log2).unique().sorted().collect_vec();

    for entry in log_final {
        println!("{entry:#?}");
    }
}
