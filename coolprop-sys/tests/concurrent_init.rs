use std::{
    sync::{Arc, Barrier},
    thread,
};

use coolprop_sys::COOLPROP;

#[test]
fn concurrent_first_access_publishes_initialized_library() {
    // Given
    const THREADS: usize = 16;
    let barrier = Arc::new(Barrier::new(THREADS));

    // When
    let threads = (0..THREADS)
        .map(|_| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                let coolprop = COOLPROP.shared_access();
                unsafe { coolprop.Props1SI(c"Water".as_ptr(), c"Tcrit".as_ptr()) }
            })
        })
        .collect::<Vec<_>>();
    let values = threads
        .into_iter()
        .map(|thread| thread.join().expect("initialization thread should not panic"))
        .collect::<Vec<_>>();

    // Then
    assert_eq!(values.len(), THREADS);
    assert!(values.iter().all(|value| value.is_finite()));
}
