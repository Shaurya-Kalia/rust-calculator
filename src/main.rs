use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

//  CRITICAL FIX: This line registers the file so it gets compiled.
mod cxxqt_object;

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML file
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/example/calculator/src/qml/main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
