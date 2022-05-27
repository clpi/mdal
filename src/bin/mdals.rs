use dashmap::DashMap;
use mdals::server::Backend;
use tower_lsp::{lsp_types::*, Client, LanguageServer, LspService, Server};

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        ast_map: DashMap::new(),
        document_map: DashMap::new(),
        semantic_token_map: DashMap::new(),
    })
    .custom_method("custom/inlay_hint", Backend::inlay_hint)
    .finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}
