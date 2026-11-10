//! Read fixed-width text data through the SAS setup file that describes
//! it, as CDC publishes the National Survey of Family Growth. R's
//! datparser builds its columns from a [`FixedWidth`].
//!
//! The crate does no I/O of its own: it reads the bytes it is given.
//! It began as the fixed-width reader of dta-parser's `dta` crate.

mod encoding;
mod error;
mod fixed_width;
pub mod missing;
pub mod threads;
mod types;
mod value_label;

pub use error::{Error, Result};
pub use fixed_width::{FixedWidth, FixedWidthSetup, FixedWidthWarning, Recodes, WarningKind};
pub use types::StorageType;
pub use value_label::{ValueLabel, ValueLabelSet};
