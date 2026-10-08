use core::pin::Pin;
use cxx_qt::CxxQtType;
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
        #[qproperty(QString, display_text, cxx_name = "displayText")]
        type RustCalculator = super::RustCalculatorRust;

        #[qinvokable]
        #[cxx_name = "evaluateExpression"]
        fn evaluate_expression(self: Pin<&mut RustCalculator>, expression: &QString);

        #[qinvokable]
        fn clear(self: Pin<&mut RustCalculator>);
    }
}

impl my_object::RustCalculator {
    pub fn clear(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().get_mut().last_result = None;
        self.as_mut().set_display_text(QString::default());
    }

    pub fn evaluate_expression(mut self: Pin<&mut Self>, expression: &QString) {
        let raw_input = expression.to_string();
        let last_result = self.rust().last_result;
        let result = crate::calculator::evaluate(&raw_input, last_result);
        let result_string = match result {
            Ok(value) => {
                self.as_mut().rust_mut().get_mut().last_result = Some(value);
                crate::calculator::format_smart(value)
            }
            Err(_) => "Invalid Expression".to_owned(),
        };

        self.as_mut().set_display_text(QString::from(&result_string));
    }
}

#[derive(Default)]
pub struct RustCalculatorRust {
    display_text: QString,
    last_result: Option<f64>,
}
