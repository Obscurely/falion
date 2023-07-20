#![no_main]
use falion::add;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let n1 = data.iter().map(|n| *n as u128).sum::<u128>() as u128;

    assert_eq!(add(n1, n1), n1 * 2);
});
