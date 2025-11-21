import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { useGoogleLogin } from '@react-oauth/google';
import axios from 'axios';
import googleAuthService from '../services/googleAuthService';

const RegisterPage = () => {
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  
  const navigate = useNavigate();
  const { register } = useAuth();

  const validateForm = () => {
    if (!username || !email || !password || !confirmPassword) {
      setError('All fields are required');
      return false;
    }
    if (username.length < 3 || username.length > 50) {
      setError('Username must be 3-50 characters');
      return false;
    }
    if (password !== confirmPassword) {
      setError('Passwords do not match');
      return false;
    }
    if (password.length < 8) {
      setError('Password must be at least 8 characters');
      return false;
    }
    if (!/[A-Z]/.test(password) || !/[a-z]/.test(password) || !/[0-9]/.test(password)) {
      setError('Password must contain uppercase, lowercase, and number');
      return false;
    }
    return true;
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError('');

    if (!validateForm()) {
      return;
    }

    setLoading(true);
    try {
      const response = await register(username, email, password);
      console.log('Register success:', response);
      
      // Registration successful, show auth method selection page
      console.log('Navigating to auth method select');
      navigate('/auth-method-select', { 
        state: { 
          email,
          username,
          password,
          isRegistration: true
        } 
      });
    } catch (err) {
      console.log('Register error:', err.response?.data);
      const errorMsg = err.response?.data?.error || err.response?.data || 'Registration failed';
      
      // Check if error is username already exists
      if (errorMsg.includes('Username already exists') || errorMsg.includes('username')) {
        setError('Username already taken. Please choose a different username.');
      }
      // Check if error is email already exists
      else if (errorMsg.includes('Email already exists') || errorMsg.includes('email')) {
        setError('Email already registered. Please use a different email or login to your existing account.');
      } else {
        setError(errorMsg);
      }
    } finally {
      setLoading(false);
    }
  };

  const handleGoogleSuccess = async (credentialResponse) => {
    try {
      setLoading(true);
      console.log('Google response:', credentialResponse);
      
      // Get the token from the response
      const token = credentialResponse?.credential || credentialResponse?.access_token;
      
      if (!token) {
        setError('No credential received from Google');
        return;
      }
      
      const result = await googleAuthService.verifyGoogleToken(token);
      
      // Ensure token is actually stored before navigating
      const storedToken = localStorage.getItem('token');
      if (!storedToken) {
        setError('Failed to store authentication token');
        return;
      }
      
      // Dispatch event to notify AuthContext
      window.dispatchEvent(new Event('auth-update'));
      
      // Small delay to ensure state updates propagate
      setTimeout(() => {
        navigate('/dashboard');
      }, 100);
    } catch (err) {
      console.error('Google registration error:', err);
      setError(err.response?.data?.error || err.message || 'Google registration failed');
    } finally {
      setLoading(false);
    }
  };

  const googleLogin = useGoogleLogin({
    onSuccess: handleGoogleSuccess,
    onError: (error) => {
      console.error('Google registration error:', error);
      setError('Google registration failed');
    },
    flow: 'implicit',
  });

  return (
    <div className="min-h-screen bg-white flex items-center justify-center p-4">
      <div className="w-full max-w-md border border-black p-8">
        <h1 className="text-3xl font-bold text-black mb-8 text-center">Register</h1>

        {error && (
          <div className="border border-black bg-white p-4 mb-6 text-black">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label className="block text-black font-bold mb-2">Username</label>
            <input
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              required
              className="w-full border border-black p-2 bg-white text-black"
              placeholder="3-50 characters"
            />
          </div>

          <div>
            <label className="block text-black font-bold mb-2">Email</label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
              className="w-full border border-black p-2 bg-white text-black"
              placeholder="your@email.com"
            />
          </div>

          <div>
            <label className="block text-black font-bold mb-2">Password</label>
            <div className="relative">
              <input
                type={showPassword ? 'text' : 'password'}
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                className="w-full border border-black p-2 pr-12 bg-white text-black"
                placeholder="Password"
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                className="absolute right-2 top-1/2 transform -translate-y-1/2 px-2 py-1 text-black font-bold border border-black hover:bg-black hover:text-white"
              >
                {showPassword ? 'Hide' : 'Show'}
              </button>
            </div>
            <p className="text-xs text-black mt-1">
              Minimum 8 characters. Must contain uppercase, lowercase, and numbers.
            </p>
          </div>

          <div>
            <label className="block text-black font-bold mb-2">Confirm Password</label>
            <div className="relative">
              <input
                type={showConfirmPassword ? 'text' : 'password'}
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                required
                className="w-full border border-black p-2 pr-12 bg-white text-black"
                placeholder="Confirm password"
              />
              <button
                type="button"
                onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                className="absolute right-2 top-1/2 transform -translate-y-1/2 px-2 py-1 text-black font-bold border border-black hover:bg-black hover:text-white"
              >
                {showConfirmPassword ? 'Hide' : 'Show'}
              </button>
            </div>
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition disabled:opacity-50"
          >
            {loading ? 'Registering...' : 'Register'}
          </button>
        </form>

        <div className="mt-6 border-t border-black pt-6">
          <p className="text-center text-black mb-4">Or continue with:</p>
          <button
            onClick={() => googleLogin()}
            disabled={loading}
            className="w-full bg-white border border-black text-black font-bold py-2 px-4 mb-3 hover:bg-black hover:text-white transition disabled:opacity-50"
          >
            Google Register
          </button>
          <Link to="/web3-auth">
            <button
              type="button"
              className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition"
            >
              Web3 Register
            </button>
          </Link>
        </div>

        <div className="mt-6 text-center">
          <p className="text-black">
            Already have an account?{' '}
            <Link to="/login" className="font-bold border-b border-black hover:bg-black hover:text-white">
              Login
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
};

export default RegisterPage;
