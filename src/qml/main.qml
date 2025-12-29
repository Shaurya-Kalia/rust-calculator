import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import org.kde.kirigami 2.20 as Kirigami
import com.example.calculator 1.0

Kirigami.ApplicationWindow {
    id: root
    width: 400
    height: 600
    title: "Rust Calculator"

    // 1. Instantiate the Rust Logic
    RustCalculator {
        id: calculator
    }

    pageStack.initialPage: Kirigami.Page {
        title: "Calculator"

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.gridUnit
            spacing: Kirigami.Units.largeSpacing

            // 2. RESULT DISPLAY (Scrollable + Copyable)
            ScrollView {
                id: resultScroll
                Layout.fillWidth: true
                Layout.preferredHeight: 140
                Layout.alignment: Qt.AlignRight

                TextArea {
                    text: calculator.displayText == "" ? "0" : calculator.displayText

                    // Force text to wrap by constraining width
                    width: resultScroll.availableWidth

                    color: "white"
                    font.pointSize: 26
                    horizontalAlignment: Text.AlignRight

                    readOnly: true         // Output only
                    selectByMouse: true    // Allow copying
                    wrapMode: Text.WrapAnywhere
                    background: null
                }
            }

            Kirigami.Separator { Layout.fillWidth: true }

            // 3. INPUT FIELD (Focus + Enter Key)
            TextField {
                id: inputField
                placeholderText: "e.g. 23*4/(3^3+5!)"
                Layout.fillWidth: true
                font.pointSize: 14

                // Focus: Start typing immediately
                focus: true
                Component.onCompleted: forceActiveFocus()

                // Trigger calculation on 'Enter'
                onAccepted: {
                    calculator.evaluateExpression(this.text)
                }
            }

            Label {
                text: "Supports: +, -, *, /, ^, !, sin(pi), log(), etc."
                color: Kirigami.Theme.disabledTextColor
                font.pointSize: 10
            }

            // 4. BUTTONS (Responsive Grid)
            GridLayout {
                Layout.fillWidth: true
                columnSpacing: Kirigami.Units.largeSpacing
                rowSpacing: Kirigami.Units.largeSpacing

                // If window is thin (<350px), stack buttons vertically.
                // Otherwise, put them side-by-side.
                columns: root.width < 350 ? 1 : 2

                Button {
                    text: "Calculate"
                    Layout.fillWidth: true
                    highlighted: true
                    onClicked: {
                        calculator.evaluateExpression(inputField.text)
                    }
                }

                Button {
                    text: "Clear (Esc)"
                    Layout.fillWidth: true
                    onClicked: {
                        inputField.text = ""
                        calculator.displayText = ""
                        inputField.forceActiveFocus()
                    }
                }
            }

            // 5. SHORTCUTS
            Shortcut {
                sequence: "Esc"
                onActivated: {
                    inputField.text = ""
                    calculator.displayText = ""
                    inputField.forceActiveFocus()
                }
            }

            Item { Layout.fillHeight: true } // Spacer to push everything up
        }
    }
}
