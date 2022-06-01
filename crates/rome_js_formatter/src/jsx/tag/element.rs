use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxElement;
use rome_rowan::AstNode;

impl FormatNodeFields<JsxElement> for FormatNodeRule<JsxElement> {
    fn format_fields(node: &JsxElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).format(formatter)
    }
}
