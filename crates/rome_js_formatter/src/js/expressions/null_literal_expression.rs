use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsNullLiteralExpression;
use rome_js_syntax::JsNullLiteralExpressionFields;

impl FormatNodeFields<JsNullLiteralExpression> for FormatNodeRule<JsNullLiteralExpression> {
    fn format_fields(node: &JsNullLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNullLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
