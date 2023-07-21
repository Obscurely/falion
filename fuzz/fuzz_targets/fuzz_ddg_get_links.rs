#![no_main]
use libfuzzer_sys::fuzz_target;
use falion::search::ddg;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let ddg = ddg::Ddg::new();

    let input: String = data.iter().map(|b| char::from_u32(*b as u32).unwrap()).collect();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            match ddg.get_links(&input, None, None).await {
                Ok(_) => (),
                Err(ddg::DdgError::NoResults) => (),
                Err(ddg::DdgError::QueryTooLong) => (),
                Err(_) => panic!("error"),
            }
        })
});
