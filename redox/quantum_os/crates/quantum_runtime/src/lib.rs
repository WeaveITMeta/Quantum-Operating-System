// =============================================================================
// MACROHARD Quantum OS - Quantum Runtime
// =============================================================================
// Table of Contents:
//   1. Module Declarations
//   2. Error Types
//   3. LogosQ Compatibility (feature-gated)
//   4. Async Execution (feature-gated)
//   5. Prelude Module
// =============================================================================
// Purpose: Quantum runtime layer providing typed circuit IR, state backends,
//          execution engine, and measurement streaming. Wraps LogosQ with
//          full-word snake_case naming conventions.
// =============================================================================

pub mod circuit_program;
pub mod error;
pub mod execution;
pub mod gate_operations;
pub mod measurement;
pub mod state_backend;

#[cfg(feature = "logosq_compat")]
pub mod logosq_compat;

pub mod async_runtime;
pub mod streaming;
pub mod optimization;

#[cfg(feature = "gpu_backend")]
pub mod gpu_backend;

pub mod webtransport;

pub mod prelude {
    pub use crate::circuit_program::*;
    pub use crate::error::*;
    pub use crate::execution::*;
    pub use crate::gate_operations::*;
    pub use crate::measurement::*;
    pub use crate::state_backend::*;

    #[cfg(feature = "logosq_compat")]
    pub use crate::logosq_compat::*;

    pub use crate::async_runtime::*;
    pub use crate::streaming::*;
    pub use crate::webtransport::*;
}
