# Data Race in Rust with Mutable References

This repository demonstrates a common error in Rust: creating multiple mutable references to the same variable without proper synchronization. This can lead to data races and unpredictable behavior.

The `bug.rs` file contains the erroneous code. The `bugSolution.rs` file provides a corrected version using techniques to avoid data races.

**Key Concepts:**

* **Mutable References:** References using `&mut` allow modification of the referenced data.
* **Data Races:** Occur when multiple threads or parts of the code access and modify the same memory location concurrently without synchronization mechanisms.
* **Ownership and Borrowing:** Rust's ownership system helps prevent many concurrency issues, but requires careful consideration when working with mutable references.

**How to reproduce the bug:**

1. Clone this repository.
2. Navigate to the repository directory.
3. Compile and run `bug.rs` (the result will be unpredictable due to the data race).
4. Compare the output to the output of `bugSolution.rs` (which resolves the race condition).
