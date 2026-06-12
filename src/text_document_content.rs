use crate::{StaticRegistrationOptions, Uri};
use serde::{Deserialize, Serialize};

/// Client capabilities for a text document content provider.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentClientCapabilities {
    /// Text document content provider supports dynamic registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
}

/// Text document content provider options.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentOptions {
    /// The schemes for which the server provides content.
    pub schemes: Vec<String>,
}

/// Text document content provider registration options.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentRegistrationOptions {
    #[serde(flatten)]
    pub text_document_content_options: TextDocumentContentOptions,

    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TextDocumentContentServerCapabilities {
    RegistrationOptions(TextDocumentContentRegistrationOptions),
    Options(TextDocumentContentOptions),
}

impl From<TextDocumentContentOptions> for TextDocumentContentServerCapabilities {
    fn from(from: TextDocumentContentOptions) -> Self {
        Self::Options(from)
    }
}

impl From<TextDocumentContentRegistrationOptions> for TextDocumentContentServerCapabilities {
    fn from(from: TextDocumentContentRegistrationOptions) -> Self {
        Self::RegistrationOptions(from)
    }
}

/// Parameters for the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentParams {
    /// The URI of the text document.
    pub uri: Uri,
}

/// Result of the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentResult {
    /// The text content of the text document.
    pub text: String,
}

/// Parameters for the `workspace/textDocumentContent/refresh` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentRefreshParams {
    /// The URI of the text document to refresh.
    pub uri: Uri,
}
