use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("com.example.calculator").qml_file("src/qml/main.qml"),
    )
    .files(["src/cxxqt_object.rs"])
    .build();
}
