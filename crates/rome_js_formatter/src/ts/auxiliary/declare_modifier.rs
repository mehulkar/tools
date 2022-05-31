use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsDeclareModifier;
use rome_js_syntax::TsDeclareModifierFields;

impl FormatNodeFields<TsDeclareModifier> for FormatNodeRule<TsDeclareModifier> {
    fn format_fields(node: &TsDeclareModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsDeclareModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}
