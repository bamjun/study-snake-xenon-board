# Frontend 구축 가이드

이 문서는 `frontend` 디렉토리의 React 애플리케이션을 처음부터 구축하기 위한 가이드입니다.

## 1. 프로젝트 초기화 (Vite)

프로젝트 루트에서 Vite를 사용하여 React 프로젝트를 생성합니다.

```bash
npm create vite@latest frontend -- --template react
cd frontend
npm install
```

## 2. 의존성 설치

필요한 라이브러리들을 설치합니다. (Redux, Router, Axios, React Query 등)

```bash
npm install @reduxjs/toolkit react-redux react-router-dom axios @tanstack/react-query
```

## 3. 프로젝트 구조 생성

다음 명령어로 필요한 디렉토리 구조를 만듭니다.

```bash
mkdir -p src/api src/pages src/store src/assets
```

## 4. API 클라이언트 설정 (`src/api/client.js`)

Axios 인스턴스를 생성하고 인증 토큰 처리를 위한 인터셉터를 설정합니다.

```javascript
import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api';

const client = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

client.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

export default client;
```

## 5. Redux 스토어 설정

### `src/store/authSlice.js`
인증 상태 관리를 위한 Slice를 생성합니다.

```javascript
import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import client from '../api/client';

export const loginUser = createAsyncThunk(
  'auth/login',
  async ({ username, password }, { rejectWithValue }) => {
    try {
      const response = await client.post('/users/login', { username, password });
      const { token } = response.data;
      localStorage.setItem('token', token);
      return token;
    } catch (error) {
      return rejectWithValue(error.response?.data || 'Login failed');
    }
  }
);

export const registerUser = createAsyncThunk(
  'auth/register',
  async ({ username, email, password }, { rejectWithValue }) => {
    try {
      await client.post('/users/register', { username, email, password });
      return true;
    } catch (error) {
      return rejectWithValue(error.response?.data || 'Registration failed');
    }
  }
);

export const fetchUser = createAsyncThunk(
  'auth/fetchUser',
  async (_, { rejectWithValue }) => {
    try {
      const response = await client.get('/users/me');
      return response.data;
    } catch (error) {
      return rejectWithValue(error.response?.data || 'Fetch user failed');
    }
  }
);

const authSlice = createSlice({
  name: 'auth',
  initialState: {
    user: null,
    token: localStorage.getItem('token'),
    isAuthenticated: !!localStorage.getItem('token'),
    loading: false,
    error: null,
  },
  reducers: {
    logout: (state) => {
      localStorage.removeItem('token');
      state.user = null;
      state.token = null;
      state.isAuthenticated = false;
      state.error = null;
    },
    clearError: (state) => { state.error = null; }
  },
  extraReducers: (builder) => {
    builder
      .addCase(loginUser.fulfilled, (state, action) => {
        state.loading = false;
        state.token = action.payload;
        state.isAuthenticated = true;
      })
      .addCase(fetchUser.fulfilled, (state, action) => {
        state.user = action.payload;
        state.isAuthenticated = true;
      })
      .addCase(loginUser.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload;
      });
      // 필요한 경우 다른 케이스 추가
  },
});

export const { logout, clearError } = authSlice.actions;
export default authSlice.reducer;
```

### `src/store/store.js`

```javascript
import { configureStore } from '@reduxjs/toolkit';
import authReducer from './authSlice';

export const store = configureStore({
  reducer: {
    auth: authReducer,
  },
});
```

## 6. 페이지 컴포넌트 구현

### `src/pages/Login.jsx`

```jsx
import { useState, useEffect } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useDispatch, useSelector } from 'react-redux';
import { loginUser, clearError, fetchUser } from '../store/authSlice';

const Login = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const dispatch = useDispatch();
  const navigate = useNavigate();
  const { loading, error, isAuthenticated } = useSelector((state) => state.auth);

  useEffect(() => {
    if (isAuthenticated) dispatch(fetchUser()).then(() => navigate('/'));
    return () => dispatch(clearError());
  }, [isAuthenticated, navigate, dispatch]);

  const handleSubmit = (e) => {
    e.preventDefault();
    dispatch(loginUser({ username, password }));
  };

  return (
    <div className="auth-container">
      <h2>Login</h2>
      {error && <div className="error">{JSON.stringify(error)}</div>}
      <form onSubmit={handleSubmit}>
        <input value={username} onChange={(e) => setUsername(e.target.value)} placeholder="Username" required />
        <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder="Password" required />
        <button disabled={loading}>Login</button>
      </form>
      <Link to="/register">Register</Link>
    </div>
  );
};
export default Login;
```

### `src/pages/BoardList.jsx`

```jsx
import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import client from '../api/client';

const fetchPosts = async () => {
  const response = await client.get('/boards');
  return response.data;
};

const BoardList = () => {
  const { data: posts, isLoading, error } = useQuery({
    queryKey: ['posts'],
    queryFn: fetchPosts,
  });

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error loading posts</div>;

  return (
    <div className="board-container">
      <h1>Board</h1>
      <Link to="/create">New Post</Link>
      <div className="post-list">
        {posts?.map(post => (
          <div key={post.id} className="post-card">
            <h3><Link to={`/posts/${post.id}`}>{post.title}</Link></h3>
            <p>{post.content.substring(0, 100)}...</p>
          </div>
        ))}
      </div>
    </div>
  );
};
export default BoardList;
```

*참고: `Register.jsx`, `BoardDetail.jsx`, `BoardForm.jsx` 파일도 유사한 방식으로 구현이 필요합니다.*

## 7. 메인 앱 설정

### `src/App.jsx`
라우팅 및 인증 보호(PrivateRoute)를 설정합니다.

```jsx
import { useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { useDispatch, useSelector } from 'react-redux';
import { fetchUser, logout } from './store/authSlice';
import Login from './pages/Login';
// import Register from './pages/Register';
import BoardList from './pages/BoardList';
// import BoardDetail from './pages/BoardDetail';
// import BoardForm from './pages/BoardForm';

const PrivateRoute = ({ children }) => {
  const { isAuthenticated } = useSelector((state) => state.auth);
  return isAuthenticated ? children : <Navigate to="/login" />;
};

function App() {
  const dispatch = useDispatch();
  useEffect(() => {
    if (localStorage.getItem('token')) dispatch(fetchUser());
  }, [dispatch]);

  return (
    <Router>
      <Routes>
        <Route path="/login" element={<Login />} />
        {/* <Route path="/register" element={<Register />} /> */}
        <Route path="/" element={<PrivateRoute><BoardList /></PrivateRoute>} />
        {/* 추가 라우트 설정 */}
      </Routes>
    </Router>
  );
}

export default App;
```

### `src/main.jsx`
Provider들을 연결합니다.

```jsx
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { Provider } from 'react-redux';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { store } from './store/store';
import App from './App.jsx';
import './index.css';

const queryClient = new QueryClient();

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <Provider store={store}>
      <QueryClientProvider client={queryClient}>
        <App />
      </QueryClientProvider>
    </Provider>
  </StrictMode>,
);
```

## 8. CSS 스타일링 (`src/index.css`)
기본적인 스타일을 초기화합니다.

```css
:root {
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;
  color-scheme: light dark;
  color: rgba(255, 255, 255, 0.87);
  background-color: #242424;
}

body {
  margin: 0;
  display: flex;
  place-items: center;
  min-width: 320px;
  min-height: 100vh;
}

/* 추가 스타일 정의... */
```

## 9. 실행

```bash
npm run dev
```
