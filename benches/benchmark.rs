use criterion::{criterion_group, criterion_main, Criterion, Throughput,Bencher};
use std::path::PathBuf;
use std::env;
use crate::function1;

fn get_path(s: &str) -> PathBuf {
    PathBuf::from(format!(
        "{}/testcases/{}",
        env::var("CARGO_MANIFEST_DIR").unwrap(),
        s
    ))
}

fn test1(b: &mut Bencher) {
    let path = get_path("case1");
    b.iter(|| {
        let _ = function1();
    });
}