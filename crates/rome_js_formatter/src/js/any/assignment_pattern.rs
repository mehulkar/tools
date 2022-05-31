//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyAssignmentPattern;
use crate::prelude::*;
use rome_js_syntax::JsAnyAssignmentPattern;
impl FormatRule<JsAnyAssignmentPattern> for FormatJsAnyAssignmentPattern {
    type Context = JsFormatContext;
    fn format(node: &JsAnyAssignmentPattern, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyAssignmentPattern::JsAnyAssignment(node) => node.format().format(f),
            JsAnyAssignmentPattern::JsArrayAssignmentPattern(node) => node.format().format(f),
            JsAnyAssignmentPattern::JsObjectAssignmentPattern(node) => node.format().format(f),
        }
    }
}
