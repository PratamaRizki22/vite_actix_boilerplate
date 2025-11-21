import { createBrowserRouter, Navigate } from 'react-router-dom';
import App from './App';
import HomePage from './pages/HomePage';
import UsersPage from './pages/UsersPage';
import UserProfilePage from './pages/UserProfilePage';
import PostsPage from './pages/PostsPage';
import ProfilePage from './pages/ProfilePage';
import NotFoundPage from './pages/NotFoundPage';
import LoginPage from './pages/LoginPage';
import RegisterPage from './pages/RegisterPage';
import Web3AuthPage from './pages/Web3AuthPage';
import TwoFactorSetupPage from './pages/TwoFactorSetupPage';
import TwoFactorVerifyPage from './pages/TwoFactorVerifyPage';
import OTPVerifyPage from './pages/OTPVerifyPage';
import DashboardPage from './pages/DashboardPage';
import AuthMethodSelectPage from './pages/AuthMethodSelectPage';
import ProtectedRoute from './components/ProtectedRoute';
import { AuthProvider, useAuth } from './context/AuthContext';

// Landing component that redirects based on authentication
const LandingPage = () => {
  const { isAuthenticated, loading } = useAuth();
  
  if (loading) {
    return (
      <div className="min-h-screen bg-white flex items-center justify-center p-4">
        <div className="text-center">
          <p className="text-black font-bold">Loading...</p>
        </div>
      </div>
    );
  }
  
  if (isAuthenticated) {
    return <HomePage />;
  }
  
  // Redirect to login if not authenticated
  return <Navigate to="/login" replace />;
};

const router = createBrowserRouter([
  {
    path: '/',
    element: (
      <AuthProvider>
        <App />
      </AuthProvider>
    ),
    children: [
      {
        index: true,
        element: <LandingPage />
      },
      {
        path: 'login',
        element: <LoginPage />
      },
      {
        path: 'register',
        element: <RegisterPage />
      },
      {
        path: 'auth-method-select',
        element: <AuthMethodSelectPage />
      },
      {
        path: 'web3-auth',
        element: <Web3AuthPage />
      },
      {
        path: '2fa-setup',
        element: <TwoFactorSetupPage />
      },
      {
        path: '2fa-verify',
        element: <TwoFactorVerifyPage />
      },
      {
        path: 'verify-otp',
        element: <OTPVerifyPage />
      },
      {
        path: 'dashboard',
        element: (
          <ProtectedRoute>
            <DashboardPage />
          </ProtectedRoute>
        )
      },
      {
        path: 'profile',
        element: (
          <ProtectedRoute>
            <ProfilePage />
          </ProtectedRoute>
        )
      },
      {
        path: 'user/:userId',
        element: (
          <ProtectedRoute>
            <UserProfilePage />
          </ProtectedRoute>
        )
      },
      {
        path: 'users',
        element: (
          <ProtectedRoute requiredRole="admin">
            <UsersPage />
          </ProtectedRoute>
        )
      },
      {
        path: 'posts', 
        element: (
          <ProtectedRoute>
            <PostsPage />
          </ProtectedRoute>
        )
      },
      {
        path: '*',
        element: <NotFoundPage />
      }
    ]
  }
]);

export default router;