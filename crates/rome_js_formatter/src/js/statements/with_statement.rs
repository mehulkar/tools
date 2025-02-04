use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatBodyStatement;
use crate::FormatNodeFields;
use rome_js_syntax::JsWithStatement;
use rome_js_syntax::JsWithStatementFields;

impl FormatNodeFields<JsWithStatement> for FormatNodeRule<JsWithStatement> {
    fn fmt_fields(node: &JsWithStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsWithStatementFields {
            with_token,
            l_paren_token,
            object,
            r_paren_token,
            body,
        } = node.as_fields();

        write!(
            f,
            [
                with_token.format(),
                space_token(),
                format_delimited(&l_paren_token?, &object.format(), &r_paren_token?,)
                    .soft_block_indent(),
                FormatBodyStatement::new(&body?)
            ]
        )
    }
}
