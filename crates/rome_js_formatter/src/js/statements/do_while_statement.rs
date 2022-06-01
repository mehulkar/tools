use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsDoWhileStatementFields;
use rome_js_syntax::{JsAnyStatement, JsDoWhileStatement};

impl FormatNodeFields<JsDoWhileStatement> for FormatNodeRule<JsDoWhileStatement> {
    fn format_fields(node: &JsDoWhileStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDoWhileStatementFields {
            do_token,
            body,
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [do_token.format()])?;

        match body? {
            JsAnyStatement::JsEmptyStatement(body) => {
                write!(f, [body.format(), hard_line_break()])?;
            }
            body => {
                write!(f, [space_token(), body.format()])?;
            }
        };

        write![
            f,
            [
                space_token(),
                while_token.format(),
                space_token(),
                format_delimited(&l_paren_token?, &test.format(), &r_paren_token?,)
                    .soft_block_indent(),
            ]
        ]?;

        match semicolon_token {
            Some(semicolon_token) => write!(f, [semicolon_token.format()]),
            None => write!(f, [token(";")]),
        }
    }
}
