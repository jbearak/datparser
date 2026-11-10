/// One entry of a value label set.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueLabel {
    /// The labelled value; NaN when `missing` is not 0.
    pub value: f64,
    /// The missing code labelled, or 0 for a number.
    pub missing: u8,
    pub text: String,
}

/// A named table of value labels.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueLabelSet {
    pub name: String,
    pub labels: Vec<ValueLabel>,
}
