use syn::spanned::Spanned;
use thiserror::Error;
use proc_macro2::Span;
use proc_macro_error::abort;
use std::result::Result as StdResult;

#[derive(Error, Debug, Clone)]
#[error("{message}")]
pub struct DeriveError {
    span: Span,
    message: String,
    note: Option<String>,
    help: Option<String>,
}

impl DeriveError {
    pub fn new<T: Spanned, S: Into<String>>(span: T, message: S) -> Self {
        Self {
            span: span.span().clone(),
            message: message.into(),
            note: None,
            help: None,
        }
    }

    pub fn with_note<T: Into<String>>(mut self, note: T) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn with_help<T: Into<String>>(mut self, help: T) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn abort(&self) -> ! {
        let DeriveError { span, message, note, help } = self;

        match (note, help) {
            (None, None) => abort!(
                    span, message
                ),
            (Some(note), None) => abort!(
                    span, message;
                    note = note;
                ),
            (None, Some(help)) => abort!(
                    span, message;
                    help = help;
                ),
            (Some(note), Some(help)) => abort!(
                    span, message;
                    note = note;
                    help = help;
                ),
        }
    }
}

pub type Result<T> = StdResult<T, DeriveError>;