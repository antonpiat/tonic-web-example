# Rust gRPC-Web + Vite 4 Frontend

A minimal full-stack example of a **Rust gRPC-Web backend** with a **Vue 3 + TypeScript frontend** using **Vite 4**.  
This project demonstrates:

- gRPC services in Rust using **tonic**
- gRPC-Web support with **tonic-web**
- Vue 3 + TypeScript frontend with **protobuf-ts** generated clients
- Echo and Math RPC examples

---

## Table of Contents

1. [Features](#features)
2. [Project Structure](#project-structure)
3. [Setup](#setup)
    - [Backend](#backend)
    - [Frontend](#frontend)
4. [Preview](#preview)
---

## Features

- **Echo Service**: send a message and receive it back from the backend
- **Math Service**: perform basic arithmetic operations (Add, Subtract, Multiply, Divide) via gRPC
- **TypeScript + Vue 3 frontend** fully typed via `protobuf-ts`
- **CORS enabled** for local development
- **Responsive card layout** for clean UI

---

## Project Structure

client/<br>
├─ index.html<br>
├─ package.json<br>
├─ vite.config.ts<br>
├─ src/<br>
│ ├─ components/<br>
│ ├─ proto/ ← protobuf-ts generated files<br>
│ ├─ App.vue<br>
│ ├─ client.ts/<br>
│ ├─ main.ts<br>
│ └─ styles.css/<br>
server/<br>
├─ Cargo.toml<br>
└─ src/<br>
├─ main.rs<br>
proto/ ← .proto files<br>

---

## Setup

### Backend

(Rust + Tonic)

1. Ensure you have **Rust + Cargo** installed.
2. Navigate to the backend folder:

```bash
cd server
cargo build
cargo run
```

### Frontend

(Vite4 + Vue3 + Typescript)

```bash
cd client
pnpm install
pnpm generate
pnpm dev
```

## Preview
<p align="center">
  <img src="https://raw.githubusercontent.com/antonpiat/tonic-web-example/main/client/public/screenshot.png" width="600" alt="App Screenshot">
</p>
