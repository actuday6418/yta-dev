#[macro_export]
macro_rules! money {
    // `()` indicates that the macro takes no argument.
    ($expr:expr) => {
        // The macro will expand into the contents of this block.
        ($expr * 10_000.0);
    };
}
