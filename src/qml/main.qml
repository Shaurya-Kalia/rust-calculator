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

    RustCalculator {
        id: calculator
    }

    function calculate() {
        var expression = inputField.text.trim()
        if (expression.length === 0)
            return

        calculator.evaluateExpression(expression)
        if (calculator.displayText !== "Invalid Expression" &&
            calculator.displayText !== "Math Error" &&
            !calculator.displayText.startsWith("∞")) {
            historyModel.insert(0, { expression: expression, result: calculator.displayText })
            if (historyModel.count > 30)
                historyModel.remove(historyModel.count - 1)
        }
    }

    function clearCalculator() {
        inputField.text = ""
        calculator.clear()
        inputField.forceActiveFocus()
    }

    pageStack.initialPage: Kirigami.Page {
        title: "Calculator"

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.gridUnit
            spacing: Kirigami.Units.largeSpacing

            ScrollView {
                id: resultScroll
                Layout.fillWidth: true
                Layout.preferredHeight: 140
                Layout.alignment: Qt.AlignRight

                TextArea {
                    id: resultArea
                    text: calculator.displayText === "" ? "0" : calculator.displayText
                    width: resultScroll.availableWidth
                    color: "white"
                    font.pointSize: 26
                    horizontalAlignment: Text.AlignRight
                    readOnly: true
                    selectByMouse: true
                    wrapMode: Text.WrapAnywhere
                    background: null

                    Menu {
                        id: resultContextMenu

                        MenuItem {
                            text: "Copy"
                            enabled: resultArea.selectedText.length > 0
                            onTriggered: resultArea.copy()
                        }

                        MenuItem {
                            text: "Select All"
                            onTriggered: resultArea.selectAll()
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        acceptedButtons: Qt.RightButton
                        propagateComposedEvents: true
                        onClicked: resultContextMenu.popup()
                    }
                }
            }

            Kirigami.Separator { Layout.fillWidth: true }

            TextField {
                id: inputField
                placeholderText: "e.g. 23*4/(3^3+5!)"
                Layout.fillWidth: true
                font.pointSize: 14
                focus: true
                Component.onCompleted: forceActiveFocus()
                onAccepted: root.calculate()

                Menu {
                    id: inputContextMenu

                    MenuItem {
                        text: "Undo"
                        enabled: inputField.canUndo
                        onTriggered: inputField.undo()
                    }

                    MenuItem {
                        text: "Redo"
                        enabled: inputField.canRedo
                        onTriggered: inputField.redo()
                    }

                    MenuSeparator {}

                    MenuItem {
                        text: "Cut"
                        enabled: inputField.selectedText.length > 0
                        onTriggered: inputField.cut()
                    }

                    MenuItem {
                        text: "Copy"
                        enabled: inputField.selectedText.length > 0
                        onTriggered: inputField.copy()
                    }

                    MenuItem {
                        text: "Paste"
                        enabled: inputField.canPaste
                        onTriggered: inputField.paste()
                    }

                    MenuSeparator {}

                    MenuItem {
                        text: "Select All"
                        enabled: inputField.length > 0
                        onTriggered: inputField.selectAll()
                    }
                }

                MouseArea {
                    anchors.fill: parent
                    acceptedButtons: Qt.RightButton
                    propagateComposedEvents: true
                    onClicked: inputContextMenu.popup()
                }
            }

            Label {
                text: "Supports: +, -, *, /, ^, !, sin(pi), log(), ans, etc."
                color: Kirigami.Theme.disabledTextColor
                font.pointSize: 10
            }

            GridLayout {
                Layout.fillWidth: true
                columnSpacing: Kirigami.Units.largeSpacing
                rowSpacing: Kirigami.Units.largeSpacing
                columns: root.width < 350 ? 1 : 2

                Button {
                    text: "Calculate"
                    Layout.fillWidth: true
                    highlighted: true
                    onClicked: root.calculate()
                }

                Button {
                    text: "Clear (Esc)"
                    Layout.fillWidth: true
                    onClicked: root.clearCalculator()
                }
            }

            Kirigami.Heading {
                text: "History"
                level: 3
                visible: historyModel.count > 0
            }

            ListView {
                id: historyView
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                visible: historyModel.count > 0
                model: ListModel { id: historyModel }
                spacing: Kirigami.Units.smallSpacing

                delegate: ItemDelegate {
                    width: historyView.width
                    height: historyLine.implicitHeight + Kirigami.Units.largeSpacing
                    onClicked: {
                        inputField.text = expression
                        inputField.forceActiveFocus()
                        inputField.cursorPosition = inputField.length
                    }

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.leftMargin: Kirigami.Units.smallSpacing
                        anchors.rightMargin: Kirigami.Units.smallSpacing
                        spacing: 0

                        Label {
                            id: historyLine
                            text: expression + " = " + result
                            Layout.fillWidth: true
                            color: Kirigami.Theme.textColor
                            elide: Text.ElideRight
                        }
                    }
                }
            }

            Item { Layout.fillHeight: historyModel.count === 0 }

            Shortcut {
                sequence: "Esc"
                onActivated: root.clearCalculator()
            }

            Shortcut {
                sequence: "Ctrl+L"
                onActivated: root.clearCalculator()
            }
        }
    }
}
