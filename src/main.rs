use trustaction_cli::transaction::BTransaction;

pub mod tests;

fn main() {
    BTransaction::serialize_transaction().unwrap();
}