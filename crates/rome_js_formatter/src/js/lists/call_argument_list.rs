use crate::generated::FormatJsCallArgumentList;
use crate::prelude::*;
use crate::utils::format_separated_for_call_arguments;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsCallArgumentList;

impl FormatRule<JsCallArgumentList> for FormatJsCallArgumentList {
    type Context = JsFormatContext;

    fn fmt(node: &JsCallArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        let args = format_with(|f| {
            let separated = node
                .format_separated(token(","))
                .with_options(
                    FormatSeparatedOptions::default()
                        .with_trailing_separator(TrailingSeparator::Disallowed),
                )
                .map(|e| e.memoized());
            format_separated_for_call_arguments(separated, node.len(), f)
        });

        write!(
            f,
            [&group_elements(&format_args![&soft_block_indent(
                &format_args![args]
            )])]
        )
    }
}
