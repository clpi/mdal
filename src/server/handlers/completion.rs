use tower_lsp::lsp_types::{
    lsif::Document, CompletionContext, CompletionItem, CompletionItemTag, CompletionList,
    CompletionOptions, CompletionParams, CompletionTextEdit, CompletionTriggerKind, Position,
    TextDocumentIdentifier,
};

pub fn get_completions(c: CompletionParams) -> Option<CompletionList> {
    let pos: Position = c.text_document_position.position;
    let tdoc: TextDocumentIdentifier = c.text_document_position.text_document;
    let _uri: url::Url = tdoc.uri;
    if let Some(ctx) = c.context {
        match ctx.trigger_kind {
            CompletionTriggerKind::INVOKED => return None,
            CompletionTriggerKind::TRIGGER_FOR_INCOMPLETE_COMPLETIONS => return None,
            CompletionTriggerKind::TRIGGER_CHARACTER => {
                if let Some(ch) = ctx.trigger_character {
                    match ch.as_str() {
                        ":" => return None,
                        "@" => return None,
                        "-" => return None,
                        "." => return None,
                        "#" => return None,
                        _ => return None,
                    }
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }
    return None;
}
