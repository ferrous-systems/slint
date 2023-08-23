// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-1.1 OR LicenseRef-Slint-commercial

//! Data structures common between LSP and previewer

use i_slint_compiler::CompilerConfiguration;
use std::path::{Path, PathBuf};

/// API used by the LSP to talk to the Preview. The other direction uses the
/// ServerNotifier
pub trait PreviewApi {
    fn set_design_mode(&self, enable: bool);
    fn design_mode(&self) -> bool;
    fn set_contents(&self, path: &Path, contents: &str);
    fn load_preview(&self, component: PreviewComponent, behavior: PostLoadBehavior);
    fn config_changed(&self, config: &CompilerConfiguration);
    fn highlight(
        &self,
        path: Option<PathBuf>,
        offset: u32,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

/// The Component to preview
#[allow(unused)]
#[derive(Default, Clone)]
pub struct PreviewComponent {
    /// The file name to preview
    pub path: PathBuf,
    /// The name of the component within that file.
    /// If None, then the last component is going to be shown.
    pub component: Option<String>,

    /// The list of include paths
    pub include_paths: Vec<PathBuf>,

    /// The style name for the preview
    pub style: String,
}

/// What to do after a preview is loaded
#[allow(unused)]
pub enum PostLoadBehavior {
    ShowAfterLoad,
    DoNothing,
}
