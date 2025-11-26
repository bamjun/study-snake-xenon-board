import { useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { useDispatch, useSelector } from 'react-redux';
import { fetchUser, logout } from './store/authSlice';
import Login from './pages/Login';
import Register from './pages/Register';
import BoardList from './pages/BoardList';
import BoardDetail from './pages/BoardDetail';
import BoardForm from './pages/BoardForm';

const PrivateRoute = ({ children }) => {
  const { isAuthenticated, loading } = useSelector((state) => state.auth);
  
  if (loading) return <div className="loading">Loading...</div>;
  
  return isAuthenticated ? children : <Navigate to="/login" />;
};

const Navbar = () => {
  const dispatch = useDispatch();
  const { user, isAuthenticated } = useSelector((state) => state.auth);
  
  return (
    <nav className="navbar">
      <div className="nav-brand">Xenon Board</div>
      <div className="nav-links">
        {isAuthenticated && user ? (
          <>
            <span className="user-welcome">Hello, {user.username}</span>
            <button onClick={() => dispatch(logout())} className="btn-logout">Logout</button>
          </>
        ) : (
          <div className="auth-links">
             {/* Links handled in pages usually */}
          </div>
        )}
      </div>
    </nav>
  );
};

function App() {
  const dispatch = useDispatch();

  useEffect(() => {
    // Attempt to fetch user if token exists on app load
    if (localStorage.getItem('token')) {
      dispatch(fetchUser());
    }
  }, [dispatch]);

  return (
    <Router>
      <div className="app-layout">
        <Navbar />
        <main className="main-content">
          <Routes>
            <Route path="/login" element={<Login />} />
            <Route path="/register" element={<Register />} />
            <Route path="/" element={
              <PrivateRoute>
                <BoardList />
              </PrivateRoute>
            } />
            <Route path="/posts/:id" element={
              <PrivateRoute>
                <BoardDetail />
              </PrivateRoute>
            } />
            <Route path="/create" element={
              <PrivateRoute>
                <BoardForm />
              </PrivateRoute>
            } />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

export default App;
