use crate::engine::Engine;
use crate::fn_native::SendSync;
use crate::module::Module;
use crate::result::EvalAltResult;
use crate::token::Position;

use crate::stdlib::boxed::Box;

mod collection;
pub use collection::ModuleResolversCollection;

#[cfg(not(feature = "no_std"))]
#[cfg(not(target_arch = "wasm32"))]
mod file;

#[cfg(not(feature = "no_std"))]
#[cfg(not(target_arch = "wasm32"))]
pub use file::FileModuleResolver;

#[cfg(not(feature = "no_std"))]
#[cfg(not(target_arch = "wasm32"))]
mod global_file;

#[cfg(not(feature = "no_std"))]
#[cfg(not(target_arch = "wasm32"))]
pub use global_file::GlobalFileModuleResolver;

mod stat;
pub use stat::StaticModuleResolver;

/// Trait that encapsulates a module resolution service.
pub trait ModuleResolver: SendSync {
    /// Resolve a module based on a path string.
    fn resolve(
        &self,
        engine: &Engine,
        path: &str,
        pos: Position,
    ) -> Result<Module, Box<EvalAltResult>>;
}