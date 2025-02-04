use crate::prelude::*;
use crate::utils::{is_simple_expression, token_has_comments};
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsCallArgumentsFields;
use rome_js_syntax::{JsAnyCallArgument, JsCallArguments};
use rome_rowan::{AstSeparatedList, SyntaxResult};

impl FormatNodeFields<JsCallArguments> for FormatNodeRule<JsCallArguments> {
    fn fmt_fields(node: &JsCallArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = node.as_fields();

        if is_simple_function_arguments(node)? {
            return write![
                f,
                [
                    l_paren_token.format(),
                    group_elements(&args.format()),
                    r_paren_token.format(),
                ]
            ];
        }

        write!(
            f,
            [
                format_delimited(&l_paren_token?, &args.format(), &r_paren_token?,)
                    .soft_block_indent()
            ]
        )
    }
}

/// Returns true if the passed [JsCallArguments] has a single argument
/// that is a simple function expression, array expression or object expression
fn is_simple_function_arguments(node: &JsCallArguments) -> SyntaxResult<bool> {
    let JsCallArgumentsFields {
        l_paren_token,
        args,
        r_paren_token,
    } = node.as_fields();

    if token_has_comments(&l_paren_token?) || token_has_comments(&r_paren_token?) {
        return Ok(false);
    }

    if args.len() > 1 {
        return Ok(false);
    }

    for item in args.elements() {
        if let Some(separator) = item.trailing_separator()? {
            if token_has_comments(separator) {
                return Ok(false);
            }
        }

        match item.node() {
            Ok(JsAnyCallArgument::JsAnyExpression(expr)) => {
                if !is_simple_expression(expr)? {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
    }

    Ok(true)
}
