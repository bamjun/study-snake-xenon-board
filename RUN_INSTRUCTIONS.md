# How to Run the Application

## Prerequisites
- Rust (cargo)
- Node.js (npm)
- PostgreSQL (running and accessible via DATABASE_URL)

## 1. Backend Setup
The backend is a Rust Axum application.

1.  Navigate to the backend directory:
    ```bash
    cd backend
    ```
2.  Ensure `.env` file exists and is configured (copy from `.env.example` if needed).
3.  Run the server:
    ```bash
    cargo run
    ```
    The server will start at `http://localhost:3000`.
    API Documentation (Swagger UI) is available at `http://localhost:3000/docs`.

## 2. Frontend Setup
The frontend is a React Vite application.

1.  Navigate to the frontend directory:
    ```bash
    cd frontend
    ```
2.  Install dependencies (if not already done):
    ```bash
    npm install
    ```
3.  Run the development server:
    ```bash
    npm run dev
    ```
    The frontend will be available at `http://localhost:5173`.

## 3. Usage
1.  Open `http://localhost:5173` in your browser.
2.  You will be redirected to the Login page.
3.  Click "Register" to create a new account.
4.  Log in with your new credentials.
5.  You can now view the board, create posts, and view post details.
