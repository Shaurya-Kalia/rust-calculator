# rust-calculator
A high-performance calculator built with Rust and Kirigami. Accepts mathematical arguments as natural expressions and displays the result. No frills or thrills. Primarily uses mavel crate for parsing. Tested only on Arch with Qt 6.10.1 on kde desktop.

## Dependencies
**Ubuntu/Debian:**

`sudo apt install qml6-module-org-kde-kirigami qml6-module-qtquick-controls`

**Fedora:**

`sudo dnf install kf6-kirigami qt6-qtdeclarative`

**Arch Linux:**

`sudo pacman -S kf6-kirigami qt6-declarative`

## Installation
1. Download the latest release (.tar.gz)
2. Extract the archive.
3. Open a terminal inside the extracted folder.
4. Run: `sudo ./install.sh`
5. Install the dependencies before running

## Uninstallation
Run: `sudo rm /usr/local/bin/rust_kirigami_calc /usr/share/applications/rust_kirigami_calc.desktop`
