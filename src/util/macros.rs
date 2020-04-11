//! Generic use macros.

/// Returns its argument if it is the `Some` case of an [`Option`].
macro_rules! return_some {
    ($e:expr) => {
        if $e.is_some() {
            return $e;
        }
    };
}
