# 🕒 Timex - Smart Time Tracking

A powerful, AI-driven tool that analyzes git commits and automatically generates timesheet entries.

---

## 📌 Overview

Timex is a tool designed to streamline time tracking for developers. By analyzing **git commit history**, it generates **AI-powered summaries** of work done and integrates with **time tracking systems** for automated timesheet submissions.


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



