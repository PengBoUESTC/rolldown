use std::{collections::HashMap, path::PathBuf};

use napi_derive::*;
use serde::Deserialize;

#[napi(object)]
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct InputOptions {
  // Not going to be supported
  // @deprecated Use the "inlineDynamicImports" output option instead.
  // inlineDynamicImports?: boolean;

  // acorn?: Record<string, unknown>;
  // acornInjectPlugins?: (() => unknown)[] | (() => unknown);
  // cache?: false | RollupCache;
  // context?: string;sssssssssss
  // experimentalCacheExpiry?: number;
  // pub external: ExternalOption,
  pub input: HashMap<String, String>,
  // makeAbsoluteExternalsRelative?: boolean | 'ifRelativeSource';
  // /** @deprecated Use the "manualChunks" output option instead. */
  // manualChunks?: ManualChunksOption;
  // maxParallelFileOps?: number;
  // /** @deprecated Use the "maxParallelFileOps" option instead. */
  // maxParallelFileReads?: number;
  // moduleContext?: ((id: string) => string | null | void) | { [id: string]: string };
  // onwarn?: WarningHandlerWithDefault;
  // perf?: boolean;
  // pub plugins: Vec<BuildPluginOption>,
  // preserveEntrySignatures?: PreserveEntrySignaturesOption;
  // /** @deprecated Use the "preserveModules" output option instead. */
  // preserveModules?: boolean;
  pub preserve_symlinks: bool,
  pub shim_missing_exports: bool,
  // strictDeprecations?: boolean;
  pub treeshake: Option<bool>,
  // watch?: WatcherOptions | false;

  // extra
  pub cwd: String,
  // pub builtins: BuiltinsOptions,
}

pub fn resolve_input_options(opts: InputOptions) -> napi::Result<rolldown::InputOptions> {
  let cwd = PathBuf::from(opts.cwd.clone());
  assert!(cwd != PathBuf::from("/"), "{:#?}", opts);

  Ok(rolldown::InputOptions {
    input: Some(
      opts
        .input
        .into_iter()
        .map(|(name, import)| rolldown::InputItem {
          name: Some(name),
          import,
        })
        .collect::<Vec<_>>(),
    ),
    cwd: Some(cwd),
  })
}