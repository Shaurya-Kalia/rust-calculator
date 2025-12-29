use core::pin::Pin;
use cxx_qt_lib::QString;

#[cxx_qt::bridge]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

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

// STEP 2: IMPLEMENTATION
// The actual logic lives outside the bridge module.
// In src/cxxqt_object.rs

// ... inside src/cxxqt_object.rs

impl my_object::RustCalculator {
    pub fn evaluate_expression(self: Pin<&mut Self>, expression: &QString) {
        let raw_input = expression.to_string();

        let re = ::regex::Regex::new(r"(\d+)!").unwrap();
        let processed_input = re.replace_all(&raw_input, "fact($1)");

        let mut ctx = ::meval::Context::new();
        ctx.func("fact", |x| {
            let n = x as u64;
            if n > 170 { return f64::INFINITY; }
            (1..=n).fold(1.0, |acc, val| acc * (val as f64))
        });

        let result_string = match ::meval::eval_str_with_context(&processed_input, &ctx) {
            Ok(val) => format_smart(val), // Use our new formatter
            Err(_) => "Invalid Expression".to_string(),
        };

        self.set_display_text(QString::from(&result_string));
    }
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
