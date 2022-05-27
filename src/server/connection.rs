use super::config::ServerConfig;
use lsp_server::{Connection, IoThreads};
use lsp_server::{Message, RequestId, ResponseError};
use std::sync::Arc;
use tower_lsp::lsp_types::request::{
    CodeActionRequest, CodeLensRequest, DocumentLinkRequest, DocumentSymbolRequest, Request,
    SemanticTokensFullRequest, SemanticTokensRangeRequest,
};
use tower_lsp::lsp_types::WorkspaceFolder;
use tower_lsp::lsp_types::{InitializeParams, SemanticTokensFullDeltaResult};

pub async fn mainloop(con: Connection, config: ServerConfig) -> tokio::io::Result<()> {
    let connect = Arc::new(con);
    let mut ws = config.root;
    let (pend_not_tx, mut pend_not_rx) = tokio::sync::mpsc::channel(100);
    pend_not_tx.send("".into()).await.unwrap_or_default();

    // let not_connected = connect.handle_shutdown();
    let not_handler = tokio::spawn(async move {
        while let Some(n) = pend_not_rx.recv().await.unwrap() {
            // not_connected.sender
            //     .send(Message::Notification(n))
            //     .unwrap_or(());
        }
    });

    for msg in &connect.receiver {
        match msg {
            Message::Request(req) => match req.method.as_str() {
                DocumentLinkRequest::METHOD => {}
                DocumentSymbolRequest::METHOD => {}
                CodeLensRequest::METHOD => {}
                SemanticTokensFullRequest::METHOD => {}
                SemanticTokensRangeRequest::METHOD => {}
                CodeActionRequest::METHOD => {}
                _ => {
                    let _params: &serde_json::Value = &req.params;
                    let _id: &RequestId = &req.id;
                    let _method: &String = &req.method;
                    if connect.handle_shutdown(&req).unwrap() {
                        return Ok(());
                    }
                }
            },
            Message::Response(_res) => {
                let _rid: RequestId = _res.id;
                let _val: Option<serde_json::Value> = _res.result;
                let _err: Option<ResponseError> = _res.error;
            }
            Message::Notification(n) => {
                let _method = n.method;
                let _params = n.params;
            }
        }
    }
    Ok(())
}
