# Fullstack project - Bug Report
Created for the sole purpose of learning (rust)

## Stack (overall)
Backend - rust, actix-web, sqlx, jsonwebtoken, envy, argon2, ~serde

Frontend - nextJs, typescript, tailwindcss, nextauth, daisyui, hookform, framer-motion

# Features
- CRUD api
- full auth with jwt bearer tokens
- hashing passwords
- backend integration test
- CORS
- filtering

# How to run locally with Docker
1. Setup .env - [see this](#env) 
2. Run 
```bash
docker compose up -d
``` 
3. Open http://localhost:3000 in your browser (backend can take a while to start because of cargo-watch)


## Env
Create .env in top directory, it's shared by frontend and backend

Example env for ***local*** development with Docker
```env
# Backend related

POSTGRES_HOST=127.0.0.1
POSTGRES_PORT=5432
POSTGRES_USER=admin
POSTGRES_PASSWORD=password123
POSTGRES_DB=rust_sqlx

DATABASE_URL=postgresql://admin:password123@postgres:5432/rust_sqlx?schema=public

PGADMIN_DEFAULT_EMAIL=admin@admin.com
PGADMIN_DEFAULT_PASSWORD=password123

JWT_SECRET=unsafesecret
HASH_SECRET=unsafesecret

FRONTEND_URL=http://localhost:3000

# Frontend related

NEXTAUTH_SECRET=unsafesecret
NEXTAUTH_URL=http://localhost:3000
NEXT_PUBLIC_BACKEND_URL=http://localhost:8000

# Required for SSR while running everything in Docker
BACKEND_URL=http://backend:8000
```
