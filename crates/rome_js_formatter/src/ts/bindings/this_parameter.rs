use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{TsThisParameter, TsThisParameterFields};

impl FormatNodeFields<TsThisParameter> for FormatNodeRule<TsThisParameter> {
    fn format_fields(node: &TsThisParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsThisParameterFields {
            this_token,
            type_annotation,
        } = node.as_fields();

        write![f, [this_token.format(), type_annotation.format()]]
    }
}
