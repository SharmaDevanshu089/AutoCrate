```
 █████╗ ██╗   ██╗████████╗ ██████╗  ██████╗ ██████╗  █████╗ ████████╗███████╗
██╔══██╗██║   ██║╚══██╔══╝██╔═══██╗██╔════╝ ██╔══██╗██╔══██╗╚══██╔══╝██╔════╝
███████║██║   ██║   ██║   ██║   ██║██║      █████╔╝ ███████║   ██║   █████╗  
██╔══██║██║   ██║   ██║   ██║   ██║██║      ██╔══██╗██╔══██║   ██║   ██╔══╝  
██║  ██║╚██████╔╝   ██║   ╚██████╔╝╚██████╔╝██║  ██║██║  ██║   ██║   ███████╗
╚═╝  ╚═╝ ╚═════╝    ╚═╝    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚══════╝
```


**AutoCrate** is a smart command-line tool built with Rust to supercharge your development workflow. It eliminates the repetitive task of setting up new Rust projects. Just run one command, and AutoCrate creates a sequentially named project, initializes it, and opens it in your editor, getting you straight to coding. 🚀

Stop wasting time setting up projects — just run one command and start coding.  

---

## ✨ Core Features

* **Guided First-Run Setup**: An interactive, colorful setup wizard on the first launch to configure everything perfectly.
* **Sequential Project Creation**: Automatically names your projects in sequence (e.g., `my_project1`, `my_project2`, ...), so you never have to think of a name again.
* **Automatic `cargo new`**: Creates a fresh, valid Rust project using the official Cargo command.
* **Instant Editor Launch**: Opens the newly created project directly in VS Code, ready for you to start coding.
* **Smart Configuration**: Remembers your project's parent folder and naming preference in a simple JSON file.
* **Native Folder Picker**: Uses your operating system's native file explorer to choose your main project folder during setup—no manual path typing needed!
* **Robust Error Handling & Logging**: If something goes wrong, errors are clearly reported, and a detailed `log.txt` is created for easy debugging.

---

## 🚦 Getting Started

Getting started with AutoCrate is incredibly simple.

> **⚠️ Important Note:** Currently, AutoCrate is designed to work **only on Windows**. Support for other operating systems is planned for the future.

### **1. Installation**

The easiest way to install AutoCrate is by using the pre-compiled binary.

1.  Head over to the [**Releases Page**](https://github.com/SharmaDevanshu089/AutoCrate/releases) on GitHub.
2.  Download the zip and Extract it.
3.  Open the zip and **run `install.bat`**. This script will automatically set up AutoCrate and add it to your system's PATH.
4.  Now Open Command Prompt from Start Menu

### **2. First-Time Setup**

After installation, open a **new terminal** (CMD, PowerShell, or Windows Terminal) and run:

```bash
autocrate
```

The first time you run this, a guided setup wizard will launch:
1.  It will ask you for a **base name** for your sequential projects (e.g., `project`, `leet_code`, `learning_rust`).
2.  A **file explorer window will pop up**, allowing you to graphically select the main "super-folder" where all these projects will be stored.
3.  That's it! Your configuration is saved.

### **3. Everyday Use**

Once the setup is complete, creating a new project is as simple as running `autocrate` in your terminal. It will automatically find the next number in the sequence, create the project folder (e.g., `learning_rust3`), and ask if you'd like to open it in VS Code.

---
### **Or. You Can Compile from Source**
## 📦 Building  

Clone the repo and build with Cargo:  

```bash
git clone https://github.com/your-username/autocrate.git
cd autocrate

### 🔨 Compile

Build in **debug mode** (faster, good for testing):

```bash
cargo build
```

Build in **release mode** (optimized, good for daily use):

```bash
cargo build --release
```


## ⚙️ How It Works

AutoCrate is designed to be transparent and easy to manage.

### **Configuration File**

Your preferences are stored in a `_Config.json` file located in your user's AppData directory:
`%APPDATA%\sharmadevanshu089\autocrate\config\_Config.json`

A typical config file looks like this:
```json
{
  "serial_name": "my_rust_project",
  "super_folder_path": "D:\\Development\\Rust_Projects",
  "add_shortcut": true,
  "editor": "code"
}
```

### **Log File**

For troubleshooting, AutoCrate creates a `log.txt` file in the same directory as the config file. If you encounter any issues, this log file is the first place to look for details.

---

## 🛣️ Project Roadmap

This project is actively developed. Here is the current plan:

* [x] Smart configuration management (`_Config.json`).
* [x] First-run interactive setup wizard.
* [x] Automatic project creation with `cargo new`.
* [x] Sequential project naming (`project1`, `project2`, etc.).
* [x] Launch project in VS Code.
* [x] Robust error handling and logging system.
* [ ] Initialize a local Git repository automatically.
* [ ] Create a remote GitHub repository and push the initial commit.
* [ ] Support for more editors (JetBrains RustRover, Neovim, Helix).
* [ ] Project templates (CLI app, library, web server).
* [ ] ~~GUI frontend with a project browser.~~ *(Idea deprecated in favor of a powerful CLI experience)*.

---

## 📜 Project Changelog

### **v0.9.7: First Public Release Candidate**
* Packaged the first stable, feature-complete release candidate for public use. 🚀
* Finalized in-app user guidance and documentation to ensure a smooth out-of-the-box experience.

### **v0.9.0: IDE Integration**
* Enabled seamless workflow integration by adding functionality to automatically launch newly created projects in **Visual Studio Code**.

### **v0.8.0: Cargo Automation Engine**
* Integrated direct command-line calls to `cargo` to fully automate the creation and scaffolding of new Rust projects.

### **v0.7.1: Edge Case Patch**
* Addressed a critical edge case where the sequential counter failed if no previous project folders existed, enhancing reliability.

### **v0.7.0: Sequential Project Intelligence**
* Deployed the core logic for intelligently scanning directories to determine the next sequential project number, forming the heart of the "auto-crate" feature.

### **v0.6.0: Configuration Persistence Engine**
* Engineered a comprehensive file I/O system for robustly reading from and writing to the `_Config.json` file.

### **v0.5.6: Major Stability Overhaul**
* Deployed significant patches to address and resolve multiple underlying stability problems, resulting in a more resilient application.

### **v0.5.3: Native File Dialog Integration**
* Leveraged the `rfd` crate to integrate **native OS file dialogs**, enabling a seamless and intuitive graphical folder selection experience during setup.

### **v0.5.1: Input Validation & Sanitization**
* Implemented a regex-based input filter to validate and sanitize user-defined project names, ensuring compatibility and preventing errors.

### **v0.5.0: Enhanced User Experience (UI)**
* Revamped the interactive setup with a **vibrant, color-coded UI** for a more intuitive and engaging user onboarding process.

### **v0.4.8: Core Logic Refactor**
* Addressed memory management challenges by resolving key borrowing issues within the Rust codebase, improving performance and stability.

### **v0.4.7: Enhanced Log Readability**
* Optimized log file formatting for improved clarity and easier diagnostic parsing.

### **v0.4.6: Configuration Architecture Revamp**
* Overhauled the configuration-saving mechanism to use system-standard directories, ensuring platform compliance and predictability.

### **v0.4.5: Pathing & Stability Patch**
* Resolved a critical issue where configuration was incorrectly saved to the current working directory instead of the user's dedicated profile folder.

### **v0.4.0: Inaugural Setup Process**
* Rolled out the first version of the interactive user setup sequence.

### **v0.3.0: Automated Directory Management**
* Engineered automated creation of the root configuration directory, complete with resilient error handling.

### **v0.2.0: Robust Error Handling & Diagnostics**
* Introduced a scalable error-handling framework and integrated **diagnostic logging** to a dedicated `log.txt` file for streamlined troubleshooting.

### **v0.1.0: Foundation & Configuration Check**
* Implemented the initial logic to detect existing configuration files, laying the groundwork for the entire application.

---

## 🛠️ Technical Details & Dependencies

For those interested in building from source or contributing, here are the key crates that power AutoCrate:

* **`colored`**: Adds beautiful, easy-to-read colors to the terminal output, enhancing the user experience.
* **`rfd` (Rust File Dialog)**: Provides the native, OS-level "select a folder" dialog during setup.
* **`directories`**: A helper library to find the standard, platform-specific paths for config files, ensuring data is stored in the correct location.
* **`serde` & `serde_json`**: Used for seamless serialization and deserialization of the `_Config.json` file, making configuration management robust and simple.
* **`regex`**: Ensures that the project base names provided by the user are valid and won't cause issues with file systems or Cargo.

---

## 🤝 Contributing

Got an idea? Found a bug? Contributions are welcome!

Please feel free to **open an issue** to discuss a new feature or report a problem. If you'd like to contribute code, **pull requests** are highly encouraged.

---

## ⚖️ License

This project is licensed under the **MIT License**. You can find the full license text in the `LICENSE.md` file.