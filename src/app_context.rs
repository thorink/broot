use crate::cli::AppLaunchArgs;
use crate::verbs::VerbStore;

/// The immutable container that can be passed around to provide
/// the configuration things
pub struct AppContext {
    pub launch_args: AppLaunchArgs,
    pub verb_store: VerbStore,
}
