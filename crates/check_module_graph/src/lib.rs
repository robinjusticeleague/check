#![deny(clippy::use_self)]

mod diagnostics;
mod format_module_graph;
mod js_module_info;
mod module_graph;

pub use check_js_type_info::ImportSymbol;
pub use check_resolver::ResolvedPath;

pub use diagnostics::ModuleDiagnostic;
pub use js_module_info::{
    JsExport, JsImport, JsImportPath, JsImportPhase, JsModuleInfo, JsModuleInfoDiagnostic,
    JsOwnExport, JsReexport, ModuleResolver, SerializedJsModuleInfo,
};
pub use module_graph::{ModuleDependencies, ModuleGraph, SUPPORTED_EXTENSIONS};
