#![no_std]
#![no_main]

#[flipperzero_test::tests]
mod tests {
    use flipperzero::dolphin::Dolphin;

    #[test]
    fn stats() {
        let mut dolphin = Dolphin::open();
        let stats = dolphin.stats();
        assert!(stats.level >= 1);
    }
}

flipperzero_test::tests_runner!(name = "Dolphin Integration Test", [crate::tests]);
