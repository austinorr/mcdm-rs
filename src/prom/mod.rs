pub mod math;
mod multicriterion_flow;
pub use multicriterion_flow::multicriterion_flow;
pub mod interop;

#[cfg(feature = "io")]
pub use interop::polars::{df_from_csv, FromPolars};

mod pref_functions;
mod promethee;
pub use promethee::{Criteria, Prom};
pub mod types;
pub mod unicriterion_flow;
pub mod utils;
