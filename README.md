# 📝 AI Text Paraphrasing App

A minimal, full-stack AI paraphrasing tool built with **React + TypeScript** and **Rust (Actix-Web)**. Users can type, select, and paraphrase text in real-time using an AI-powered API. This project was built as part of a technical assessment.

## 🌐 Live Demo

👉 [Deployed Website](https://task-hye8.vercel.app)

---

## 🧰 Tech Stack

### Frontend
- React + TypeScript
- Vite
- CSS (Responsive)
- `react-responsive` for device detection

### Backend
- Rust (Actix-Web)
- Shuttle deployment
- OpenRouter AI (Mistral 7B model)

---

## 🛠️ Local Development Setup

### Prerequisites

- Node.js (v14+)
- Rust (latest stable)
- Cargo
- Shuttle CLI

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/paraphrasing-app.git
cd paraphrasing-app
```

### 2. Backend Setup

```bash
cd backend
cargo shuttle run
```

➡️ Make sure to add your `OPENAI_API_KEY` using Shuttle secrets:

```bash
cargo shuttle secrets add OPENAI_API_KEY
```

### 3. Frontend Setup

```bash
cd ../frontend
npm install
echo "VITE_API_URL=http://localhost:8000" > .env
npm run dev
```

---

## 🧱 Architecture Overview

- **Frontend**: Vite-powered React SPA with a text area that captures selected text and sends it to the backend.
- **Backend**: Rust API that calls OpenRouter’s AI with a structured prompt and returns a clean paraphrased sentence.
- **AI Model**: Mistral 7B via OpenRouter
- **Deployment**: Frontend (Vercel), Backend (Shuttle)

---

## ✨ Features

- 📝 Editable text area with selection tracking
- 🔍 Button to paraphrase selected text
- 🤖 AI integration via OpenRouter
- ♻️ Seamless text replacement with AI result
- 📱 Mobile-friendly responsive layout

---

## ⚙️ Evaluation Criteria Fulfillment

- ✅ Functionality (text input, selection, paraphrasing)
- ✅ Code quality (modular, well-structured, comments)
- ✅ Error handling (API errors, invalid selection)
- ✅ Responsive UI
- ✅ Proper backend deployment & CORS config
- ✅ Secure API key management
- ✅ Fast execution (completed within assessment time)

---

## ⚠️ Challenges Faced

- Handling unpredictable AI output and extracting only the first sentence
- Working with Actix-Web and async/await as a Rust beginner
- Configuring CORS correctly between Shuttle and Vercel

---

## 🔄 Trade-Offs & Improvements

- Only the first sentence of AI output is used — could extend for multi-sentence
- No user auth or session storage — future improvement
- Minimal error UI — could use toasts or inline validation
- Could implement additional text transformation modes (e.g. pirate, formal)

---

## 📤 Deployment

### Frontend: Vercel

- GitHub-connected project with `VITE_API_URL` set to the Shuttle backend URL.

### Backend: Shuttle

- Deployed with:

```bash
cargo shuttle deploy
```

- Secrets managed via `cargo shuttle secrets`

---

## 📚 Future Ideas (Bonus)

- Add more AI modes (Shakespeare, Pirate, etc.)
- Text grammar correction, simplification
- Save edit history with localStorage or backend
- Better UI interactions (e.g., context menu for paraphrasing)

---

## 👨‍💻 Author

Made by [Sakib Shadman](https://sakibshadman.com)

---

## 🧾 License

MIT License
