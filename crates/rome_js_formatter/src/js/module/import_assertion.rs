use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsImportAssertion;
use rome_js_syntax::JsImportAssertionFields;

impl FormatNodeFields<JsImportAssertion> for FormatNodeRule<JsImportAssertion> {
    fn format_fields(node: &JsImportAssertion, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportAssertionFields {
            assert_token,
            l_curly_token,
            assertions,
            r_curly_token,
        } = node.as_fields();

        write![f, [assert_token.format(), space_token()]]?;

        write!(
            f,
            [
                f.delimited(&l_curly_token?, &assertions.format(), &r_curly_token?,)
                    .soft_block_spaces()
            ]
        )
    }
}
