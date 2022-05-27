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
pub fn hover(ws: usize, params: HoverParams) -> Option<Hover> {
    let path = params
        .text_document_position_params
        .text_document
        .uri
        .to_file_path()
        .unwrap();
    let pos = params.text_document_position_params.position;
    let markup = MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!(
            "{} {} Hover: {}",
            pos.line,
            pos.character,
            &path.as_path().to_str().unwrap()
        )
        .into(),
    };
    return Some(Hover {
        contents: HoverContents::Markup(markup),
        range: None,
    });
}
