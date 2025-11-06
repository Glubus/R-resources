/// Tests to verify thread-safety and concurrent access to resources
use r_ressources::*;
use std::sync::Arc;
use std::thread;

#[test]
fn test_concurrent_string_access() {
    let handles: Vec<_> = (0..100)
        .map(|_| {
            thread::spawn(|| {
                // Access from multiple threads simultaneously
                let name = string::APP_NAME;
                assert_eq!(name, "My Awesome App");
                name
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_concurrent_int_access() {
    let handles: Vec<_> = (0..100)
        .map(|_| {
            thread::spawn(|| {
                let retries = int::MAX_RETRIES;
                assert_eq!(retries, 3);
                retries
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_concurrent_array_access() {
    let handles: Vec<_> = (0..100)
        .map(|_| {
            thread::spawn(|| {
                let langs = string_array::SUPPORTED_LANGS;
                assert_eq!(langs.len(), 3);
                assert_eq!(langs[0], "en");
                langs
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_shared_across_threads_with_arc() {
    // Even though Arc is unnecessary for const data,
    // this demonstrates the resources can be used in any threading scenario
    let data = Arc::new(string::APP_NAME);

    let handles: Vec<_> = (0..50)
        .map(|_| {
            let data_clone = Arc::clone(&data);
            thread::spawn(move || {
                assert_eq!(*data_clone, "My Awesome App");
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Demonstrates that resources implement Send + Sync automatically
#[test]
fn test_send_sync_bounds() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    // String constants are Send + Sync
    assert_send::<&'static str>();
    assert_sync::<&'static str>();

    // Int constants are Send + Sync
    assert_send::<i64>();
    assert_sync::<i64>();

    // Float constants are Send + Sync
    assert_send::<f64>();
    assert_sync::<f64>();

    // Array constants are Send + Sync
    assert_send::<&'static [&'static str]>();
    assert_sync::<&'static [&'static str]>();
}
