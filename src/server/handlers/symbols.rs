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
#[allow(deprecated)]
pub fn symbol(
    name: &str,
    kind: &str,
    tags: Vec<String>,
    loc: Location,
    container: &str,
) -> SymbolInformation {
    let t: Vec<SymbolTag> = vec![];
    return SymbolInformation {
        name: name.into(),
        kind: SymbolKind::try_from(kind).unwrap_or(SymbolKind::STRING),
        tags: Some(t),
        location: loc,
        deprecated: None,
        container_name: Some(container.into()),
    };
}

pub fn document_elements(doc: PathBuf, query: &str) -> Vec<DocumentSymbol> {
    let mut o: Vec<DocumentSymbol> = vec![];
    let uri = Url::from_file_path(&doc.as_path()).unwrap();
    let loc: Location = Location::new(uri, Range::new(Position::new(0, 0), Position::new(1, 1)));
    return o;
}

pub fn workspace_elements(ws: &Path, query: &str) -> Vec<WorkspaceSymbol> {
    let o: Vec<WorkspaceSymbol> = vec![];
    return o;
}
