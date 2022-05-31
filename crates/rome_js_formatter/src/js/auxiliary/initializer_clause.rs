use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsInitializerClause;
use rome_js_syntax::JsInitializerClauseFields;

impl FormatNodeFields<JsInitializerClause> for FormatNodeRule<JsInitializerClause> {
    fn format_fields(node: &JsInitializerClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsInitializerClauseFields {
            eq_token,
            expression,
        } = node.as_fields();

        write![f, [eq_token.format(), space_token(), expression.format()]]
    }
}
