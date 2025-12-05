// Debug the SBT update issue
use doublets::{create_heap_store, Doublets, Result};

fn main() -> Result<(), usize> {
    let mut store = create_heap_store::<usize>()?;

    let a = store.create_point()?;
    println!("Created a={}", a);

    let b = store.create_point()?;
    println!("Created b={}", b);

    let c = store.create_point()?;
    println!("Created c={}", c);

    println!("\nBefore update: c={:?}", store.get(c));

    // This update will detach c from trees, change its source/target, then reattach
    println!("\nUpdating c to point (c, a)...");
    store.update_link(c, c, a)?;

    println!("After update: c={:?}", store.get(c));

    println!("\nSuccess!");
    Ok(())
}
