use super::engine::FilterStack;
use super::filters::{
    Filter, StmtFilter, TokenListFilter,
    KeywordCaseFilter, IdentifierCaseFilter, StripWhitespaceFilter, StripCommentsFilter, StripBeforeNewline, 
    SpacesAroundOperatorsFilter, ReindentFilter, AlignedIndentFilter,
};

/// sql format options
#[derive(Default)]
pub struct FormatOption<'a> {
    /// Changes how keywords are formatted. Allowed values are "upper", "lower".
    pub keyword_case: &'a str,
    /// Changes how identifiers are formatted. Allowed values are "upper", "lower".
    pub identifier_case: &'a str,
    pub output_format: &'a str,
    /// If True comments are removed from the statements.
    pub strip_comments: bool,
    /// If True spaces are used around all operators.
    pub use_space_around_operators: bool,
    /// if True extra spaces are removed.
    pub strip_whitespace: bool,
    /// If True the indentations of the statements are changed.
    pub reindent: bool,
    pub indent_columns: bool,
    /// If True the indentations of the statements are changed, and statements are aligned by keywords.
    pub reindent_aligned: bool,
    pub indent_after_first: bool,
    /// If True tabs instead of spaces are used for indentation.
    pub indent_tabs: bool,
    /// The width of the indentation, defaults to 2.
    pub indent_width: usize,
    /// set indent char, defaults to 1 whitespace.
    pub indent_char: &'a str,
    /// The column limit (in characters) for wrapping comma-separated lists. If unspecified, it puts every item in the list on its own line.
    pub wrap_after: usize,
    /// If True comma-first notation for column names is used.
    pub comma_first: bool,
    pub right_margin: usize,
    pub(crate) grouping: bool,
}

impl<'a> FormatOption<'a> {

    pub fn default_reindent() -> Self {
        let mut options = Self::default();
        options.reindent = true;
        options.indent_width = 2;
        options.indent_char = " ";
        options
    }

    pub fn default_reindent_aligned() -> Self {
        let mut options = Self::default();
        options.reindent_aligned = true;
        // options.indent_char = " ";
        options
    }
}

pub fn validate_options(options: &mut FormatOption) {
    if options.reindent {
        options.strip_whitespace = true;
    }
    if options.reindent_aligned {
        options.strip_whitespace = true
    }
    options.indent_char = " ";
    if options.indent_tabs {
        options.indent_char = "\t";
    }
    options.indent_width = usize::max(options.indent_width, 2);
}


pub fn build_filter_stack(stack: &mut FilterStack, options: &mut FormatOption) {
    if options.keyword_case.len() > 0 {
        let keyword_case = options.keyword_case.to_lowercase();
        let filter = Box::new(KeywordCaseFilter::new(&keyword_case)) as Box<dyn Filter>;
        stack.preprocess.push(filter);
    }
    if options.identifier_case.len() > 0 {
        let identifier_case = &options.identifier_case.to_lowercase();
        let filter = Box::new(IdentifierCaseFilter::new(identifier_case)) as Box<dyn Filter>;
        stack.preprocess.push(filter);
    }
    if options.use_space_around_operators {
        options.grouping = true;
        let filter = Box::new(SpacesAroundOperatorsFilter{}) as Box<dyn TokenListFilter>;
        stack.tlistprocess.push(filter);
    }
    if options.strip_comments {
        options.grouping = true;
        let filter = Box::new(StripCommentsFilter::default()) as Box<dyn TokenListFilter>;
        stack.tlistprocess.push(filter);
    }
    if options.strip_whitespace || options.reindent {
        options.grouping = true;
        let filter = Box::new(StripWhitespaceFilter{}) as Box<dyn StmtFilter>;
        stack.stmtprocess.push(filter);
    }
    if options.reindent {
        options.grouping = true;
        // width, char, wrap_after, n, comma_first, indent_after_first, indent_columns
        let filter = ReindentFilter::new(
            options.indent_width,
            options.indent_char,
            options.wrap_after,
            "\n", 
            options.comma_first,
            options.indent_after_first, 
            options.indent_columns);
        let filter = Box::new(filter) as Box<dyn TokenListFilter>;
        stack.tlistprocess.push(filter);
    }

    if options.reindent_aligned {
        options.grouping = true;
        let filter = AlignedIndentFilter::new(options.indent_char, "\n");
        let filter = Box::new(filter) as Box<dyn TokenListFilter>;
        stack.tlistprocess.push(filter);
    }

    // TODO: right_margin

    let filter = Box::new(StripBeforeNewline{}) as Box<dyn StmtFilter>;
    stack.postprocess.push(filter);
}