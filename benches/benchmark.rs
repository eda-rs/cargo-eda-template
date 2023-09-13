use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use crate::function1;

fn get_path(s: &str) -> PathBuf {
    PathBuf::from(format!(
        "{}/testcases/{}",
        env::var("CARGO_MANIFEST_DIR").unwrap(),
        s
    ))
}

#[bench]
fn test1(b: &mut Bencher) {
    let defines = HashMap::new();
    let includes: Vec<PathBuf> = Vec::new();
    let path = get_path("case1");
    b.iter(|| {
        let _ = function1();
    });
}