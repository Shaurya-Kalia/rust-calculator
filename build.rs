use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
    .qml_module(QmlModule {
        uri: "com.example.calculator",
        rust_files: &["src/cxxqt_object.rs"],
        qml_files: &["src/qml/main.qml"],
        ..Default::default()
    })
    .build();
}
