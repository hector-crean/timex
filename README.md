# 🕒 Timex - Smart Time Tracking

A powerful, AI-driven tool that analyzes git commits and automatically generates timesheet entries.

---

## 📌 Overview

Timex is a **Rust-based** tool designed to streamline time tracking for developers. By analyzing **git commit history**, it generates **AI-powered summaries** of work done and integrates with **time tracking systems** for automated timesheet submissions.

---

## 🚀 Key Features

✅ **Multi-project Git commit analysis**  
🤖 **AI-powered work summary generation**  
📊 **Detailed diff analysis with syntax highlighting**  
⚡ **Efficient commit traversal and analysis**  
📝 **Automated timesheet generation**  
🔗 **Time tracking system integration** _(coming soon)_  

---

## 🛠 Installation

### 1️⃣ Clone the repository
```bash
git clone https://github.com/yourusername/timex.git
cd timex
```

### 2️⃣ Install dependencies
```bash
cargo build --release
```

### 3️⃣ Set up API key
Create a `.env` file in the project root and add your OpenAI API key:
```ini
OPENAI_API_KEY=your_openai_api_key_here
```

---

## ⚙️ Configuration

Create a `workload.toml` file to specify your projects:

```toml
[user]
name = "Your Name"
email = "your.email@example.com"

[[projects]]
name = "Project Name"
code = "PROJECT_CODE"
description = "Project Description"
git_url = "/path/to/git/repository"
```

---

## 📖 Usage

To analyze git history and generate summaries, run:
```bash
cargo run --bin diffs
```
This will:
1. Analyze the git history of configured projects.
2. Generate **AI-powered summaries** of work done.
3. Prepare timesheet entries _(integration coming soon)_.

---

## 📂 Project Structure

```plaintext
📦 timex
├── 📂 crates/timex_core    # Core library functionality
│   ├── 📂 git             # Git analysis and diff generation
│   ├── 📂 project         # Project and workload management
├── 📂 cli                 # Command-line interface
├── 📄 workload.toml        # Project configuration file
```

---

## 🛠 Technical Details

### **Core Components**

- 🟢 **Git Analysis** → Utilizes the `gix` library for efficient git operations.
- 🤖 **AI Integration** → Leverages OpenAI's GPT models for work summary generation.
- 🎨 **Syntax Highlighting** → Implements code diff visualization.
- 📌 **Project Management** → Uses a TOML-based configuration for managing multiple projects.

---

## 🤝 Contributing

We welcome contributions! If you're interested in improving Timex, please submit a **Pull Request**.

---

## 📜 License

Timex is licensed under the **MIT License**. See the `LICENSE` file for details.

---

## 📅 Roadmap

🔲 Integration with popular time tracking systems  
🔲 Web interface for viewing and editing summaries  
🔲 Support for more granular time tracking  
🔲 Custom templates for summary generation  
🔲 Team collaboration features  

---

## 📦 Dependencies

Timex relies on several key dependencies:

- `gix` → Git implementation in Rust.
- `async-openai` → OpenAI API client.
- `syntect` → Syntax highlighting.
- `chrono` → Date and time functionality.
- `serde` → Serialization/deserialization.
- `tokio` → Async runtime.

For a complete list of dependencies, refer to the `Cargo.toml` files in the project.

---

## 🏗 Architecture Diagram

```plaintext
+-------------------+      +-----------------+
|  Git Repository  | ---> |  Timex (Rust)   |
+-------------------+      +-----------------+
         |                         |
         v                         v
+-----------------+       +-------------------+
| Git Analysis   | -----> | AI Summary (GPT) |
+-----------------+       +-------------------+
         |                         |
         v                         v
+----------------------+  +----------------------+
| Syntax Highlighting |  | Timesheet Generation |
+----------------------+  +----------------------+
```

This provides a high-level view of how Timex processes data from Git and generates timesheets.

---

## ✅ Ready to automate your timesheets? Try **Timex** today!

