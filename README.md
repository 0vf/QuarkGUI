# QuarkGUI
GUI for Quark
The actual quark.exe executable is made by Echnobas, all credits to him

it's cool

![gui](https://cdn.discordapp.com/attachments/1044585102384042005/1089953259608887386/image.png)

# Installation
To install, you can simply use the zip found in Releases. Alternitavely, you can build the program using the instructions below.

# Build Instructions
Building is rather tricky without a GNU/Linux-like environment. We'll be using MSYS2 for the build instructions to make it as simple as possible, assuming you already have MSYS2 installed:

1. Launch `MSYS2 MINGW64`
2. Clone the Git repository: `git clone https://github.com/z-ffqq/QuarkGUI.git`
3. Install required dependencies:
```pacman -S mingw-w64-x86_64-glib2 mingw-w64-x86_64-pango mingw-w64-x86_64-atk mingw-w64-x86_64-gtk3 mingw-w64-x86_64-zlib mingw-w64-x86_64-libiconv mingw-w64-x86_64-rust```
4. CD into the cloned git repository
5. Run `cargo run`, this should begin installing the dependencies and compiling the program.
