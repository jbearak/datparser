/// The smallest Stata storage type that holds a variable's values, which
/// decides whether R gets integers or doubles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Byte,
    Int,
    Long,
    Double,
    Str,
}
