//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyMethodSignatureModifier;
use crate::prelude::*;
use rome_js_syntax::TsAnyMethodSignatureModifier;
impl FormatRule<TsAnyMethodSignatureModifier> for FormatTsAnyMethodSignatureModifier {
    type Context = JsFormatContext;
    fn format(node: &TsAnyMethodSignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyMethodSignatureModifier::TsAccessibilityModifier(node) => node.format().format(f),
            TsAnyMethodSignatureModifier::JsStaticModifier(node) => node.format().format(f),
            TsAnyMethodSignatureModifier::TsOverrideModifier(node) => node.format().format(f),
            TsAnyMethodSignatureModifier::TsAbstractModifier(node) => node.format().format(f),
        }
    }
}
