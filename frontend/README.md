# Frontend - Snake Xenon Board

This is the frontend application for the Snake Xenon Board, built with React and Vite.

## Tech Stack
- **Framework**: React (Vite)
- **Routing**: React Router DOM
- **HTTP Client**: Axios
- **State Management**: Redux Toolkit (Authentication)
- **Data Fetching**: TanStack React Query (Server State)
- **Styling**: Vanilla CSS

## Features
- **Authentication**: Login and Register pages (managed by Redux).
- **Board**: List posts, view post details, create new posts (managed by React Query).
- **Protected Routes**: Requires login to access board features.

## Getting Started

1.  **Install Dependencies**
    ```bash
    npm install
    ```

2.  **Run Development Server**
    ```bash
    npm run dev
    ```
    The app will be available at `http://localhost:5173`.

## Project Structure
- `src/api`: Axios client configuration.
- `src/store`: Redux store and slices.
- `src/pages`: Page components (Login, Register, BoardList, BoardDetail, BoardForm).
- `src/App.jsx`: Main application component with routing.