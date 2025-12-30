# rust-calculator
A high-performance calculator built with Rust and Kirigami. Accepts mathematical arguments as natural expressions and displays the result. No frills or thrills. Primarily uses mavel crate for parsing. Tested only on Arch with Qt 6.10.1 on kde desktop.

<img width="542" height="762" alt="image" src="https://github.com/user-attachments/assets/8a571de2-12ed-45a2-9315-e2433719bc02" /> <img width="542" height="762" alt="image" src="https://github.com/user-attachments/assets/f03b1afd-0e02-40dc-9702-65f3f5e7a48a" />



## Dependencies
**Ubuntu/Debian:**

`sudo apt install qml6-module-org-kde-kirigami qml6-module-qtquick-controls`

**Fedora:**

`sudo dnf install kf6-kirigami qt6-qtdeclarative`

**Arch Linux:**

`sudo pacman -S kirigami qt6-declarative`

## Installation
1. Download the latest release (.tar.gz)
2. Extract the archive.
3. Open a terminal inside the extracted folder.
4. Run: `sudo ./install.sh`
5. Install the dependencies before running

## Uninstallation
Run: `sudo rm /usr/local/bin/rust_kirigami_calc /usr/share/applications/rust_kirigami_calc.desktop`
