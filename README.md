```
 █████╗ ██╗   ██╗████████╗ ██████╗  ██████╗ ██████╗  █████╗ ████████╗███████╗
██╔══██╗██║   ██║╚══██╔══╝██╔═══██╗██╔════╝ ██╔══██╗██╔══██╗╚══██╔══╝██╔════╝
███████║██║   ██║   ██║   ██║   ██║██║      █████╔╝ ███████║   ██║   █████╗  
██╔══██║██║   ██║   ██║   ██║   ██║██║      ██╔══██╗██╔══██║   ██║   ██╔══╝  
██║  ██║╚██████╔╝   ██║   ╚██████╔╝╚██████╔╝██║  ██║██║  ██║   ██║   ███████╗
╚═╝  ╚═╝ ╚═════╝    ╚═╝    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚══════╝
```

# 🚀 AutoCrate
**AutoCrate** is a Rust-powered CLI tool that automates your workflow by creating sequential Rust projects via a usique name, initializing git repositories, as well as automatically adding various functions as boilerplate and opening them directly in your editor to save your Precious time.  

Stop wasting time setting up projects — just run one command and start coding.  

---

## ✨ Features  

- 📂 Creates new Rust projects in sequence (`gptN → gptN+1`).  
- ⚡ Initializes local git repo with first commit.  
- 🖥️ Opens project in your preferred editor (default: VS Code).  
- 🛠️ Simple config file to remember your project superfolder.  
- 🔒 Future-proof: GitHub integration, templates, encryption, background mode, and GUI coming soon.  

---

## 📦 Installation  

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

Copy the binary to your `$PATH` (Linux example):

```bash
cp target/release/autocrate ~/.local/bin/
```

---

## ⚙️ Configuration

Config is stored at:

```bash
~/.autocrate/config.json
```

Example:

```json
{
  "superfolder": "/home/dev/Projects",
  "editor": "code"
}
```

---

## 🛣 Roadmap

* [x] Config management (superfolder path).
* [x] Cargo + project creation.
* [x] Local git repo init + commit.
* [x] Open in VS Code.
* [ ] Encription via Keyring and storing credentials.
* [ ] GitHub repo creation + push.
* [ ] Support multiple editors (RustRover, Vim, Helix).
* [ ] Project templates (CLI, GUI, web).
* [ ] Config encryption for GitHub tokens.
* [ ] Background service mode.
* [ ] GUI frontend with project browser.

---

## ⚖️ License

Licensed under **MIT OR Apache-2.0** (your choice).
This gives you freedom to use, modify, and distribute.

---

## 🤝 Contributing

Pull requests are welcome! For big changes, open an issue first to discuss what you’d like to improve.

---

## 💡 Inspiration

This project was built to automate repetitive project setup and help developers (especially Rust learners) focus more on coding and less on boilerplate.

```
