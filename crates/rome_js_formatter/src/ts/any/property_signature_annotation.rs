//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertySignatureAnnotation;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureAnnotation;
impl FormatRule<TsAnyPropertySignatureAnnotation> for FormatTsAnyPropertySignatureAnnotation {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyPropertySignatureAnnotation,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        match node {
            TsAnyPropertySignatureAnnotation::TsTypeAnnotation(node) => node.format().format(f),
            TsAnyPropertySignatureAnnotation::TsOptionalPropertyAnnotation(node) => {
                node.format().format(f)
            }
        }
    }
}
