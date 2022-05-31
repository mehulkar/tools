use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsxName, JsxNameFields};

impl FormatNodeFields<JsxName> for FormatNodeRule<JsxName> {
    fn format_fields(node: &JsxName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
