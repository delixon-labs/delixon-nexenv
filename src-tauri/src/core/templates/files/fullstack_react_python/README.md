# {{project_name}}

Fullstack monorepo: React (Vite) + FastAPI.

## Structure

```
frontend/   - React + Vite + TypeScript
backend/    - FastAPI + Python
```

## Getting Started

### Backend
```bash
cd backend
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
uvicorn app.main:app --reload
```

### Frontend
```bash
cd frontend
npm install
npm run dev
```
