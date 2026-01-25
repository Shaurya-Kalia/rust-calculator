use core::pin::Pin;
use cxx_qt_lib::QString;

#[cxx_qt::bridge]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_cxx_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, display_text)]
        type RustCalculator = super::RustCalculatorRust;

        // STEP 1: DECLARATION ONLY
        // We only tell Qt that this function exists. No { body } here.
        #[qinvokable]
        fn evaluate_expression(self: Pin<&mut RustCalculator>, expression: &QString);
    }
}


impl my_object::RustCalculator {
    pub fn evaluate_expression(self: Pin<&mut Self>, expression: &QString) {
        let raw_input = expression.to_string();

        // 1. SMART PRE-PROCESSOR (Handles '!' on numbers AND parentheses)
        let processed_input = preprocess_expression(&raw_input);

        // 2. Setup Context
        let mut ctx = ::meval::Context::new();
        ctx.func("fact", |x| {
            let n = x as u64;
            if n > 170 { return f64::INFINITY; }
            (1..=n).fold(1.0, |acc, val| acc * (val as f64))
        });
        // Bind 'pi' explicitly to ensure high precision (meval defaults to standard pi, but good to be safe)
        ctx.var("pi", std::f64::consts::PI);

        // 3. Evaluate
        let result_string = match ::meval::eval_str_with_context(&processed_input, &ctx) {
            Ok(val) => format_smart(val),
            Err(_) => "Invalid Expression".to_string(),
        };

        self.set_display_text(QString::from(&result_string));
    }
}


fn preprocess_expression(input: &str) -> String {
    let mut expr = input.to_string();

    // Keep finding '!' until none are left
    while let Some(idx) = expr.find('!') {
        let chars: Vec<char> = expr.chars().collect();
        let mut start = idx - 1;

        // Safety check: if '!' is at start, it's invalid, just remove it to avoid panic
        if idx == 0 {
            expr.remove(0);
            continue;
        }

        // Logic: Scan backwards to find the operand
        if chars[start] == ')' {
            // Case 1: It's a group like (1+2)!
            let mut depth = 1;
            while start > 0 {
                start -= 1;
                if chars[start] == ')' { depth += 1; }
                else if chars[start] == '(' { depth -= 1; }

                if depth == 0 { break; }
            }
        } else {
            // Case 2: It's a number/variable like 123! or pi!
            // Scan back while char is a digit, letter, or dot
            while start > 0 {
                let prev = chars[start - 1];
                if prev.is_alphanumeric() || prev == '.' || prev == '_' {
                    start -= 1;
                } else {
                    break;
                }
            }
        }

        // Extract the part to wrap (e.g., "123" or "(1+2)")
        let operand: String = chars[start..idx].iter().collect();

        // Replace "operand!" with "fact(operand)"
        // This removes the '!' so the loop eventually ends
        expr.replace_range(start..=idx, &format!("fact({})", operand));
    }

    expr
}

// --- HELPER FUNCTION ---
fn format_smart(val: f64) -> String {
    if val.is_infinite() { return "∞ Value too large".to_string(); }
    if val.is_nan() { return "Math Error".to_string(); }

    let abs_val = val.abs();

    // If number is huge (> 1 trillion) or tiny (< 1 millionth), use Scientific Mode
    if abs_val != 0.0 && (abs_val >= 1e12 || abs_val <= 1e-6) {
        // 1. Get standard format (e.g., "7.1099e+74")
        let scientific = format!("{:.6e}", val);

        // 2. Split into base and exponent
        let parts: Vec<&str> = scientific.split('e').collect();
        if parts.len() == 2 {
            let base = parts[0];
            let exponent = parts[1].replace("+", ""); // Remove the plus sign

            // 3. Convert exponent digits to Superscript Unicode
            let superscript_exp: String = exponent.chars().map(|c| match c {
                '0' => '⁰', '1' => '¹', '2' => '²', '3' => '³', '4' => '⁴',
                '5' => '⁵', '6' => '⁶', '7' => '⁷', '8' => '⁸', '9' => '⁹',
                '-' => '⁻', _ => c
            }).collect();

            // 4. Return pretty format: 7.1099 × 10⁷⁴
            return format!("{} × 10{}", base, superscript_exp);
        }
    }

    // Otherwise, just standard number
    val.to_string()
}

#[derive(Default)]
pub struct RustCalculatorRust {
    display_text: QString,
}
