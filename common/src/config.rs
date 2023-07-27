use yaserde::ser::Config;

pub const YASERDE_CFG: Config = yaserde::ser::Config {
    perform_indent: true,
    write_document_declaration: false,
    indent_string: None,
};

pub const INNER_CFG: Config = yaserde::ser::Config {
    perform_indent: false,
    write_document_declaration: false,
    indent_string: None,
};
