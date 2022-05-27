use std::path::PathBuf;

use lsp_server::{Connection, IoThreads};
use tower_lsp::{
    lsp_types::{
        ClientInfo, CodeActionOptions, CodeActionProviderCapability, CodeLensOptions,
        CompletionOptions, DeclarationCapability, DeclarationOptions, DocumentFilter,
        DocumentLinkOptions, DocumentSymbolOptions, ExecuteCommandOptions,
        FileOperationRegistrationOptions, HoverOptions, HoverProviderCapability, InitializeParams,
        InitializeResult, OneOf, SemanticTokensFullOptions, SemanticTokensLegend,
        SemanticTokensOptions, SemanticTokensRegistrationOptions, SemanticTokensServerCapabilities,
        ServerCapabilities, ServerInfo, StaticRegistrationOptions,
        StaticTextDocumentRegistrationOptions, TextDocumentRegistrationOptions,
        TextDocumentSyncCapability, TextDocumentSyncKind, TypeDefinitionProviderCapability,
        WorkDoneProgressOptions, WorkspaceFileOperationsServerCapabilities, WorkspaceFolder,
        WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
    },
    Client,
};
use url::Url;

use crate::semantic_token::LEGEND_TYPE;

pub struct ServerConfig {
    pub client_info: ClientInfo,
    pub workspace_dirs: Vec<WorkspaceFolder>,
    pub root: PathBuf,
}

impl ServerConfig {
    pub fn init_conn() -> Result<(Connection, IoThreads, ServerConfig), std::io::Error> {
        log::trace!("Init_conn() start");
        let (conn, io_thr) = Connection::stdio();
        let (id, params) = conn.initialize_start().unwrap();
        let init_params: InitializeParams = serde_json::from_value(params)?;
        let root = init_params
            .root_uri
            .clone()
            .unwrap()
            .to_file_path()
            .unwrap();
        let dirs: Vec<WorkspaceFolder> = init_params.workspace_folders.unwrap_or_default();
        let cinfo = init_params.client_info.unwrap();
        let sconf = ServerConfig {
            client_info: cinfo,
            workspace_dirs: dirs,
            root,
        };
        let sinfo = InitializeResult {
            capabilities: Self::capabilities(),
            server_info: Some(Self::info()),
        };
        let initres = serde_json::to_value(sinfo).unwrap();
        conn.initialize_finish(id, initres).unwrap();
        return Ok((conn, io_thr, sconf));
    }

    pub fn info() -> ServerInfo {
        return ServerInfo {
            name: "mdanals".to_string(),
            ..ServerInfo::default()
        };
    }

    pub fn capabilities() -> ServerCapabilities {
        ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            completion_provider: Some(Self::completion_prov()),
            execute_command_provider: Some(Self::cmd_prov()),
            semantic_tokens_provider: Some(Self::semantic_tokens_prov()),
            definition_provider: Some(OneOf::Left(true)),
            references_provider: Some(OneOf::Left(true)),
            document_symbol_provider: Some(OneOf::Right(DocumentSymbolOptions {
                work_done_progress_options: WorkDoneProgressOptions::default(),
                label: Some("sym".into()),
            })),
            document_highlight_provider: Some(OneOf::Left(true)),
            document_formatting_provider: Some(OneOf::Left(true)),
            document_link_provider: Some(DocumentLinkOptions {
                resolve_provider: Some(true),
                work_done_progress_options: WorkDoneProgressOptions::default(),
            }),
            rename_provider: Some(OneOf::Left(true)),
            workspace: Some(Self::workspace_prov()),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Options(HoverOptions {
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(true),
                },
            })),
            code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
                code_action_kinds: None,
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(true),
                },
                resolve_provider: Some(true),
            })),
            code_lens_provider: Some(CodeLensOptions {
                resolve_provider: Some(true),
            }),
            type_definition_provider: Some(TypeDefinitionProviderCapability::Options(
                StaticTextDocumentRegistrationOptions {
                    id: Some("id".into()),
                    document_selector: Some(vec![Self::primary_filter()]),
                },
            )),
            declaration_provider: Some(DeclarationCapability::Options(DeclarationOptions {
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(true),
                },
            })),

            ..ServerCapabilities::default()
        }
    }

    pub fn init_result() -> InitializeResult {
        InitializeResult {
            capabilities: Self::capabilities(),
            server_info: Some(Self::info()),
        }
    }

    pub fn cmd_prov() -> ExecuteCommandOptions {
        ExecuteCommandOptions {
            commands: vec![
                "mdals.file.showLinks".to_string(),
                "mdals.file.elements".to_string(),
                "mdals.workspace.elements".to_string(),
            ],
            work_done_progress_options: Default::default(),
        }
    }

    pub fn primary_filter() -> DocumentFilter {
        DocumentFilter {
            scheme: Some("file".into()),
            pattern: Some(".{md, mdx, idx, id}".into()),
            language: Some("markdown".into()),
        }
    }

    pub fn workspace_prov() -> WorkspaceServerCapabilities {
        WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                did_create: None,
                did_rename: None,
                will_rename: None,
                will_delete: None,
                did_delete: None,
                will_create: Some(FileOperationRegistrationOptions {
                    ..Default::default()
                }),
            }),
        }
    }

    pub fn semantic_tokens_prov() -> SemanticTokensServerCapabilities {
        return SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
            SemanticTokensRegistrationOptions {
                text_document_registration_options: {
                    TextDocumentRegistrationOptions {
                        document_selector: Some(Self::filetypes()),
                    }
                },
                semantic_tokens_options: SemanticTokensOptions {
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                    legend: SemanticTokensLegend {
                        token_types: LEGEND_TYPE.clone().into(),
                        token_modifiers: vec![],
                    },
                    range: Some(true),
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                },
                static_registration_options: StaticRegistrationOptions::default(),
            },
        );
    }

    pub fn ftype(langname: &str, pattern: &str) -> DocumentFilter {
        return DocumentFilter {
            scheme: Some("file".into()),
            language: Some(langname.into()),
            pattern: Some(pattern.into()),
        };
    }

    pub fn filetypes() -> Vec<DocumentFilter> {
        vec![
            Self::ftype("idlex", "*.{idx, mdi, id}"),
            Self::ftype("markdown", "*.{md, mdx}"),
        ]
    }
    pub fn completion_prov() -> CompletionOptions {
        CompletionOptions {
            resolve_provider: Some(true),
            trigger_characters: Some(vec![
                ":".into(),
                "@".into(),
                "#".into(),
                "-".into(),
                "*".into(),
                "!".into(),
                ";".into(),
                "$".into(),
                ".".into(),
                ",".into(),
            ]),
            work_done_progress_options: Default::default(),
            all_commit_characters: None,
        }
    }
}
