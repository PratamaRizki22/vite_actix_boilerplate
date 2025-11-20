import { createBrowserRouter } from 'react-router-dom';
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
import ProtectedRoute from './components/ProtectedRoute';
import { AuthProvider } from './context/AuthContext';

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
        element: <HomePage />
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
        path: 'profile',
        element: <ProfilePage />
      },
      {
        path: 'user/:userId',
        element: <UserProfilePage />
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
        element: <PostsPage />
      },
      {
        path: '*',
        element: <NotFoundPage />
      }
    ]
  }
]);

export default router;