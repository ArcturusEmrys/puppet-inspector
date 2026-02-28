pub trait StrExt {
    /// Remove trailing nulls from Rust strings.
    ///
    /// Several Inochi files I have put trailing nulls everywhere.
    /// GTK chokes on them.
    /// This fixes that.
    fn trim_nulls(self) -> Self;
}

impl StrExt for &str {
    fn trim_nulls(self) -> Self {
        self.trim_matches(char::from(0))
    }
}
