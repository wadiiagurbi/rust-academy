<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=0:1a1b27,50:B7410E,100:DEA584&height=200&section=header&text=🦀%20Rust%20Academy&fontSize=52&fontColor=ffffff&fontAlignY=38&desc=From%20zero%20to%20bare-metal,%20one%20lesson%20at%20a%20time&descAlignY=60&descSize=18&animation=fadeIn" width="100%" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/Axum-DEA584?style=for-the-badge&logo=rust&logoColor=black" />
  <img src="https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white" />
  <img src="https://img.shields.io/badge/Vue_3-4FC08D?style=for-the-badge&logo=vuedotjs&logoColor=white" />
  <img src="https://img.shields.io/badge/Vuetify-1867C0?style=for-the-badge&logo=vuetify&logoColor=white" />
  <img src="https://img.shields.io/badge/Vite-646CFF?style=for-the-badge&logo=vite&logoColor=white" />
</p>

<p align="center">
  <b>137 lessons</b> · <b>27 chapters</b> · <b>3 levels</b> · <b>26 API tests</b> · <b>1 real final project</b>
</p>

---

# Rust Academy: From Scratch to Bare-Metal

A complete, beginner-friendly learning platform that takes a learner with **zero Rust knowledge** through the basics, ownership and borrowing, traits and generics, concurrency and unsafe Rust, all the way to **bare-metal** code. It finishes with a real **CLI Task Manager** built step by step in the final chapter.

The platform is itself a working full-stack app: an **Axum + SQLx JSON API** in Rust and a **Vue 3 + Vuetify SPA**, with real accounts, progress tracking, quizzes and search. It's a live example of the concepts the course teaches.

## ✨ Features

- 📚 **137 lessons in 27 chapters** (87 beginner · 29 intermediate · 21 advanced)
- 🧩 **The same structure in every lesson:** Learning Objective → Explanation → Why It Matters → Example → Code → Code Explanation → Common Mistakes → Best Practices → Exercise → Solution → Mini Quiz
- 🔐 **Real accounts:** register and log in with JWT. The server refuses to start without a strong `JWT_SECRET`.
- 📈 **Progress tracking:** mark lessons done, see a % per chapter, and a **Continue** button that takes you back to your last lesson
- ❓ **Interactive quizzes** with saved attempts
- 🔎 **Search** across all lessons
- 🎨 **Syntax highlighting**, copy-to-clipboard code blocks and **dark mode**
- 📝 **Content as files:** lessons are Markdown + YAML frontmatter in `backend/content/`. They're never hardcoded in the frontend or copied into the database.

## 🗺️ Curriculum

| # | Chapter | # | Chapter |
|---|---|---|---|
| 1 | Getting Started | 15 | Borrowing |
| 2 | Types and Values | 16 | Lifetimes |
| 3 | Control Flow | 17 | Iterators |
| 4 | Compound Types | 18 | Modules |
| 5 | References | 19 | Testing |
| 6 | User-Defined Types | 20 | Error Handling |
| 7 | Pattern Matching | 21 | Unsafe Rust Intro |
| 8 | Methods and Traits | 22 | Concurrency |
| 9 | Generics | 23 | Idiomatic Rust |
| 10 | Closures | 24 | Unsafe Deep Dive |
| 11 | Standard Library Types | 25 | Bare-Metal Rust |
| 12 | Standard Library Traits | 26 | Rust Beyond the Backend |
| 13 | Ownership and Memory | 27 | 🏁 **Final Project: CLI Task Manager** |
| 14 | Smart Pointers | | |

## 🏗️ Architecture

```
rust-academy/
├── backend/                 Axum JSON API (Rust)
│   ├── content/rust/        137 lessons: Markdown + YAML frontmatter
│   ├── migrations/          SQLx migrations (users, progress, quizzes)
│   ├── src/
│   │   ├── api/             auth · courses · progress handlers
│   │   ├── content_loader.rs   loads + indexes lessons at startup
│   │   ├── auth.rs          JWT + password hashing
│   │   └── main.rs          config, startup checks, server
│   └── tests/api_tests.rs   26 end-to-end API tests
├── frontend/                Vue 3 + Vuetify + Vite SPA
│   └── src/
│       ├── views/           Dashboard · Lesson · Search · Login · Register
│       ├── components/      CodeBlock · QuizWidget · ProgressBar · Sidebar…
│       └── stores/          Pinia: auth · courses · theme
└── final-project/           The CLI Task Manager learners build (clap + rusqlite)
```

### API

| Method | Route | What it does |
|---|---|---|
| `GET` | `/api/health` | Health check |
| `GET` | `/api/courses` | All tracks, chapters and lessons |
| `GET` | `/api/lessons/:slug` | One lesson (Markdown + metadata) |
| `GET` | `/api/search?q=` | Search lessons |
| `POST` | `/api/auth/register` · `/api/auth/login` | Create an account / get a JWT |
| `GET` | `/api/me` | Current user |
| `GET` | `/api/progress` | Completed lessons, last lesson, quiz scores |
| `POST` `DELETE` | `/api/lessons/:slug/complete` | Mark / unmark a lesson as done |
| `POST` | `/api/quizzes/:slug/attempt` | Save a quiz attempt |

## 🚀 Getting started

**Requirements:** Rust (stable) and Node.js 20+

### 1. Backend (port 8001)

```sh
cd backend
JWT_SECRET=change-me-to-a-long-random-string cargo run
```

| Variable | Default | Notes |
|---|---|---|
| `JWT_SECRET` | none | **Required**, at least 16 characters |
| `DATABASE_URL` | `sqlite://data/dev.db` | Created automatically |
| `PORT` | `8001` | 8001 so it can run next to the Symfony Academy on 8000 |
| `CONTENT_DIR` | `content` | Where the lessons live |
| `FRONTEND_ORIGIN` | none | Extra CORS origins, comma-separated |

### 2. Frontend (port 5173)

```sh
cd frontend
npm install
echo "VITE_API_BASE_URL=http://localhost:8001/api" > .env
npm run dev
```

Open **http://localhost:5173**, create an account and start learning 🦀

### 3. Final project

```sh
cd final-project
cargo run -- add "Learn lifetimes" --priority high
cargo run -- list
cargo run -- done 1
```

## 🧪 Tests

```sh
cd backend && cargo test          # 26 API tests (auth, courses, progress, quizzes)
cd final-project && cargo test    # CLI integration tests
```

## 🎓 Part of the Academy series

| | Academy | Stack |
|---|---|---|
| 🐍 | [Python Academy](https://github.com/wadiiagurbi/python-academy) | FastAPI · Vue 3 · Vuetify |
| 🐘 | [PHP & Symfony Academy](https://github.com/wadiiagurbi/Php-MySQL-Symfony-Academy) | Symfony · MySQL · Vue 3 |
| 🦀 | **Rust Academy** (you are here) | Axum · SQLx · Vue 3 · Vuetify |

<p align="center">
  Made with 🦀 by <a href="https://github.com/wadiiagurbi">wadiiagurbi</a>
</p>

<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=0:DEA584,50:B7410E,100:1a1b27&height=110&section=footer" width="100%" />
</p>
