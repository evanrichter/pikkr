#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (&[u8], &[u8])| {
    let (query, record) = data;
    let queries = vec![query];
    if let Ok(mut p) = pikkr::Pikkr::new(&queries, 0) {
        let _ = p.parse(record);
    }
});
