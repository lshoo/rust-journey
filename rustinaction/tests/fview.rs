//! cargo test -p rustinaction --test fview  -- --nocapture

#[cfg(test)]
mod tests {
    use rustinaction::ch07::fview::run_str;

    #[test]
    fn run_should_work() {
        let res = run_str();

        assert!(res.is_ok());
    }
}
