use nu_protocol::engine::{EngineState, StateWorkingSet};

use std::path::Path;

use crate::*;

pub fn create_default_context(cwd: impl AsRef<Path>) -> EngineState {
    let mut engine_state = EngineState::new();

    let delta = {
        let mut working_set = StateWorkingSet::new(&engine_state);

        macro_rules! bind_command {
            ( $( $command:expr ),* $(,)? ) => {
                $( working_set.add_decl(Box::new($command)); )*
            };
        }

        // Core
        bind_command! {
            Alias,
            Debug,
            Def,
            DefEnv,
            Describe,
            Do,
            //Du,
            Echo,
            ErrorMake,
            ExportAlias,
            ExportCommand,
            ExportDef,
            ExportDefEnv,
            ExportEnv,
            ExportExtern,
            Extern,
            For,
            Help,
            Hide,
            //History,
            If,
            Ignore,
            Overlay,
            OverlayAdd,
            OverlayList,
            OverlayNew,
            OverlayRemove,
            Let,
            Metadata,
            Module,
            Source,
            //Tutor,
            Use,
            //Version,
        };

        working_set.render()
    };

    let _ = engine_state.merge_delta(delta, None, &cwd);

    engine_state
}
