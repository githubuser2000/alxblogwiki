//! lib4tables_concat - Rust implementation of Python lib4tables_concat module

pub mod center;
pub mod lib4tables;
pub mod lib4tables_enum;
pub mod concat;
pub mod errors;
pub mod types;

pub use concat::Concat;
pub use errors::ConcatError;
pub use types::*;

// Re-exports
pub use lib4tables::{
    OutputSyntax, BbCodeSyntax, CsvSyntax, EmacsSyntax, HtmlSyntax, MarkdownSyntax,
    could_be_prime_number_primzahlkreuz,
    could_be_prime_number_primzahlkreuz_fuer_aussen,
    could_be_prime_number_primzahlkreuz_fuer_innen,
    divisor_generator, is_prim_multiple, moon_number, prim_creativity,
    prim_fak, prim_multiple, prim_repeat,
};

pub use center::{
    DefaultOrderedDict, Multiplikationen, PrimzahlkreuzProContraStrs, alxp, cliout,
    get_text_wrap_things, i18n, info_log, multiples, output, primfaktoren,
    unique_everseen, n_pm_enum,
};

pub use lib4tables_enum::ST;
