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
pub use types::{Fl, Result};
mod cmp;
mod outranking;
pub use outranking::outranking_adjacency_matrix;
pub mod unicriterion_flow;
pub mod utils;
