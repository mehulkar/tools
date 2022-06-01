use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxExpressionChild;
use rome_rowan::AstNode;

impl FormatNodeFields<JsxExpressionChild> for FormatNodeRule<JsxExpressionChild> {
    fn format_fields(node: &JsxExpressionChild, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).format(formatter)
    }
}
