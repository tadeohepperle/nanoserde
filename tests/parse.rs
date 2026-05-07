#![cfg(all(feature = "binary", feature = "json", feature = "ron"))]
use nanoserde::{DeBin, DeJson, DeRon, SerBin, SerJson, SerRon};

// https://github.com/not-fl3/nanoserde/issues/83
#[test]
fn test_trailing_comma() {
    #[rustfmt::skip]
    #[derive(Debug, DeBin, SerBin, DeJson, SerJson, DeRon, SerRon)]
    #[allow(unused)]
    enum TestEnum {
        A
    }
}

// https://github.com/not-fl3/nanoserde/issues/88
#[test]
fn test_empty_brackets() {
    #[rustfmt::skip]
    #[derive(SerJson, DeJson, SerBin, DeBin, SerRon, DeRon)]
    #[allow(unused)]
    enum Message { Goodbye, Greeting{} }
}
