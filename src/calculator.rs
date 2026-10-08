//! Pure calculator logic, independent of Qt.
//!
//! The expression engine is Exmex. A small compatibility layer keeps the
//! calculator's original user-facing syntax, most notably postfix `!` and
//! lowercase `pi`.

use exmex::{FloatOpsFactory, MakeOperators, Operator};
use exmex::{Express, FlatEx};

#[derive(Clone, Debug)]
struct CalculatorOps;

impl MakeOperators<f64> for CalculatorOps {
    fn make<'a>() -> Vec<Operator<'a, f64>> {
        let mut ops = FloatOpsFactory::<f64>::make();

        // Exmex evaluates expression nodes independently. A final cleanup is
        // too late for numerically unstable intermediates: for example,
        // sin(PI) is a tiny non-zero f64, and dividing by it amplifies the
        // residue before the final result can be cleaned. Replace the
        // transcendental operators with calculator-aware versions so cleanup
        // happens at the operation that introduces the floating-point error.
        const NORMALIZED_UNARY_OPS: &[&str] = &[
            "sin", "cos", "tan",
            "asin", "acos", "atan",
            "sinh", "cosh", "tanh",
            "asinh", "acosh", "atanh",
            "exp", "ln", "log10", "log2",
        ];

        ops.retain(|op| !NORMALIZED_UNARY_OPS.contains(&op.repr()));

        ops.extend([
            Operator::make_unary("sin", normalized_sin),
            Operator::make_unary("cos", normalized_cos),
            Operator::make_unary("tan", normalized_tan),
            Operator::make_unary("asin", normalized_asin),
            Operator::make_unary("acos", normalized_acos),
            Operator::make_unary("atan", normalized_atan),
            Operator::make_unary("sinh", normalized_sinh),
            Operator::make_unary("cosh", normalized_cosh),
            Operator::make_unary("tanh", normalized_tanh),
            Operator::make_unary("asinh", normalized_asinh),
            Operator::make_unary("acosh", normalized_acosh),
            Operator::make_unary("atanh", normalized_atanh),
            Operator::make_unary("exp", normalized_exp),
            Operator::make_unary("ln", normalized_ln),
            Operator::make_unary("log10", normalized_log10),
            Operator::make_unary("log2", normalized_log2),
        ]);

        // The UI's natural postfix factorial syntax is rewritten to this
        // function before parsing.
        ops.push(Operator::make_unary("fact", factorial));
        ops
    }
}

/// Normalize a floating-point result immediately after a transcendental
/// operation. This is deliberately more conservative than `clean_float`: it
/// corrects values that are mathematically expected to be exact (0, +1, -1)
/// without rounding ordinary transcendental results at every nesting level.
fn normalize_transcendental(value: f64) -> f64 {
    if !value.is_finite() {
        return value;
    }

    let scale = value.abs().max(1.0);
    let tolerance = 1e-14 * scale;

    if value.abs() <= tolerance {
        return 0.0;
    }

    if (value - 1.0).abs() <= tolerance {
        return 1.0;
    }

    if (value + 1.0).abs() <= tolerance {
        return -1.0;
    }

    value
}

fn normalized_sin(x: f64) -> f64 { normalize_transcendental(x.sin()) }
fn normalized_cos(x: f64) -> f64 { normalize_transcendental(x.cos()) }
fn normalized_tan(x: f64) -> f64 { normalize_transcendental(x.tan()) }
fn normalized_asin(x: f64) -> f64 { normalize_transcendental(x.asin()) }
fn normalized_acos(x: f64) -> f64 { normalize_transcendental(x.acos()) }
fn normalized_atan(x: f64) -> f64 { normalize_transcendental(x.atan()) }
fn normalized_sinh(x: f64) -> f64 { normalize_transcendental(x.sinh()) }
fn normalized_cosh(x: f64) -> f64 { normalize_transcendental(x.cosh()) }
fn normalized_tanh(x: f64) -> f64 { normalize_transcendental(x.tanh()) }
fn normalized_asinh(x: f64) -> f64 { normalize_transcendental(x.asinh()) }
fn normalized_acosh(x: f64) -> f64 { normalize_transcendental(x.acosh()) }
fn normalized_atanh(x: f64) -> f64 { normalize_transcendental(x.atanh()) }
fn normalized_exp(x: f64) -> f64 { normalize_transcendental(x.exp()) }
fn normalized_ln(x: f64) -> f64 { normalize_transcendental(x.ln()) }
fn normalized_log10(x: f64) -> f64 { normalize_transcendental(x.log10()) }
fn normalized_log2(x: f64) -> f64 { normalize_transcendental(x.log2()) }

type CalculatorExpression = FlatEx<f64, CalculatorOps>;

/// Evaluate an expression while preserving the calculator's existing syntax.
///
/// `ans` is the previous successful result. Exmex identifies variables in
/// alphabetical order, so we construct the value vector from the parsed
/// expression's actual variable list rather than assuming a fixed position.
pub fn evaluate(input: &str, ans: Option<f64>) -> Result<f64, exmex::ExError> {
    let processed = preprocess_expression(input);
    let expression: CalculatorExpression = CalculatorExpression::parse(&processed)?;

    let vars = expression
        .var_names()
        .iter()
        .map(|name| match name.as_str() {
            "ans" => ans.unwrap_or(f64::NAN),
            _ => f64::NAN,
        });

    let value = expression.eval(vars.collect::<Vec<_>>().as_slice())?;
    Ok(clean_float(value))
}

/// Removes numerical artefacts that are below normal calculator precision.
///
/// Exmex evaluates floating-point expressions using `f64`, so values such as
/// `0.1 + 0.2` can otherwise surface as `0.30000000000000004`, and
/// `sin(pi)` can surface as a tiny residual around zero. We do not pretend
/// this makes f64 arithmetic exact; this is a calculator-facing normalization
/// step that removes errors well below 15 significant decimal digits.
fn clean_float(value: f64) -> f64 {
    if !value.is_finite() {
        return value;
    }

    let abs = value.abs();

    // Values this close to zero are numerical residue (e.g. sin(pi)).
    if abs < 1e-14 {
        return 0.0;
    }

    // Snap values that are indistinguishable from an integer at calculator
    // precision. The scale makes the tolerance work for large values too.
    let nearest_integer = value.round();
    let integer_tolerance = 1e-14 * abs.max(1.0);
    if (value - nearest_integer).abs() <= integer_tolerance {
        return nearest_integer;
    }

    // Keep about 15 significant decimal digits, enough to retain normal f64
    // calculator precision while eliminating binary-to-decimal artefacts.
    let magnitude = abs.log10().floor();
    let scale = 10_f64.powf(14.0 - magnitude);
    (value * scale).round() / scale
}

fn factorial(x: f64) -> f64 {
    let n = x as u64;
    if n > 170 {
        return f64::INFINITY;
    }
    (1..=n).fold(1.0, |acc, value| acc * value as f64)
}

/// Rewrites postfix `!` into `fact(operand)`, normalizes standalone
/// lowercase `pi` to Exmex's built-in `PI` constant, and preserves the
/// calculator convention that `log(x)` means the natural logarithm.
///
/// This keeps the user's natural input unchanged: `5!`, `(1+2)!`,
/// `sin(pi)!`, `5!!`, etc. remain valid exactly as entered.
fn preprocess_expression(input: &str) -> String {
    let normalized = normalize_compatibility(input);
    if !normalized.as_bytes().contains(&b'!') {
        return normalized;
    }

    let mut output: Vec<char> = Vec::with_capacity(normalized.chars().count() + 8);

    for ch in normalized.chars() {
        if ch != '!' {
            output.push(ch);
            continue;
        }

        if output.is_empty() {
            continue;
        }

        let end = output.len();
        let mut start = end - 1;

        if output[start] == ')' {
            let mut depth = 1usize;
            while start > 0 {
                start -= 1;
                match output[start] {
                    ')' => depth += 1,
                    '(' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
            }

            // Include a function identifier preceding the parenthesis, e.g.
            // `fact(5)!` -> `fact(fact(5))` and `sin(pi)!` -> `fact(sin(PI))`.
            if start > 0 {
                let mut name_start = start;
                while name_start > 0 {
                    let previous = output[name_start - 1];
                    if previous.is_alphanumeric() || previous == '_' {
                        name_start -= 1;
                    } else {
                        break;
                    }
                }
                start = name_start;
            }
        } else {
            while start > 0 {
                let previous = output[start - 1];
                if previous.is_alphanumeric() || matches!(previous, '.' | '_') {
                    start -= 1;
                } else {
                    break;
                }
            }
        }

        let mut replacement = Vec::with_capacity(end - start + 6);
        replacement.extend("fact(".chars());
        replacement.extend(output[start..end].iter().copied());
        replacement.push(')');
        output.splice(start..end, replacement);
    }

    output.into_iter().collect()
}

/// Normalize compatibility spellings while leaving identifiers such as
/// `pilot` or `piston` untouched.
fn normalize_compatibility(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Preserve the calculator's existing convention: log(x) == ln(x).
        if i + 3 < chars.len()
            && chars[i..i + 4] == ['l', 'o', 'g', '(']
        {
            output.push_str("ln(");
            i += 4;
            continue;
        }

        if i + 1 < chars.len()
            && chars[i] == 'p'
            && chars[i + 1] == 'i'
            && (i == 0 || !is_identifier_char(chars[i - 1]))
            && (i + 2 == chars.len() || !is_identifier_char(chars[i + 2]))
        {
            output.push_str("PI");
            i += 2;
        } else {
            output.push(chars[i]);
            i += 1;
        }
    }

    output
}

fn is_identifier_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

pub fn format_smart(value: f64) -> String {
    if value.is_infinite() {
        return "∞ Value too large".to_owned();
    }
    if value.is_nan() {
        return "Math Error".to_owned();
    }

    let abs_value = value.abs();
    if abs_value != 0.0 && (abs_value >= 1e12 || abs_value <= 1e-6) {
        let scientific = format!("{value:.6e}");
        if let Some(e_pos) = scientific.find('e') {
            let base = &scientific[..e_pos];
            let exponent = scientific[e_pos + 1..].trim_start_matches('+');
            let superscript = exponent.chars().map(to_superscript).collect::<String>();
            return format!("{base} × 10{superscript}");
        }
    }

    value.to_string()
}

fn to_superscript(c: char) -> char {
    match c {
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        '-' => '⁻',
        other => other,
    }
}
