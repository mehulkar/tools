//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyTemplateElement;
use crate::prelude::*;
use rome_js_syntax::JsAnyTemplateElement;
impl FormatRule<JsAnyTemplateElement> for FormatJsAnyTemplateElement {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyTemplateElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyTemplateElement::JsTemplateChunkElement(node) => node.format().fmt(f),
            JsAnyTemplateElement::JsTemplateElement(node) => node.format().fmt(f),
        }
    }
}
