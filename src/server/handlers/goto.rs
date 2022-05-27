use std::path::{Path, PathBuf};

use tower_lsp::lsp_types::{
    request::{
        GotoDeclarationParams, GotoImplementationParams, GotoTypeDefinitionParams, WorkspaceSymbol,
    },
    DocumentLink, DocumentSymbol, GotoDefinitionParams, Hover, HoverContents, HoverParams,
    Location, LocationLink, MarkupContent, MarkupKind, Position, Range, SymbolInformation,
    SymbolKind, SymbolTag,
};
use url::Url;

pub fn definition(ws: usize, params: GotoDefinitionParams) -> Option<Location> {
    None
}

pub fn declaration(ws: usize, params: GotoDeclarationParams) -> Option<Location> {
    None
}

pub fn imlementation(ws: usize, params: GotoImplementationParams) -> Option<Location> {
    None
}

pub fn typedef(ws: usize, params: GotoTypeDefinitionParams) -> Option<Location> {
    None
}
