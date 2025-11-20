import React, { useState, useEffect } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { useGoogleLogin } from '@react-oauth/google';
import axios from 'axios';
import googleAuthService from '../services/googleAuthService';

const LoginPage = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const [rateLimitRemaining, setRateLimitRemaining] = useState(0);
  
  const navigate = useNavigate();
  const { login } = useAuth();

  // Check localStorage untuk rate limit yang sebelumnya
  useEffect(() => {
    const storedLimitTime = localStorage.getItem('loginRateLimitExpiry');
    if (storedLimitTime) {
      const expiryTime = parseInt(storedLimitTime);
      const now = Date.now();
      const remaining = Math.ceil((expiryTime - now) / 1000);
      
      if (remaining > 0) {
        setRateLimitRemaining(remaining);
      } else {
        localStorage.removeItem('loginRateLimitExpiry');
      }
    }
  }, []);

  // Countdown timer untuk rate limit
  useEffect(() => {
    if (rateLimitRemaining > 0) {
      const timer = setTimeout(() => {
        setRateLimitRemaining(rateLimitRemaining - 1);
        
        // Update localStorage dengan time remaining
        if (rateLimitRemaining - 1 > 0) {
          const expiryTime = Date.now() + ((rateLimitRemaining - 1) * 1000);
          localStorage.setItem('loginRateLimitExpiry', expiryTime.toString());
        } else {
          localStorage.removeItem('loginRateLimitExpiry');
        }
      }, 1000);
      return () => clearTimeout(timer);
    }
  }, [rateLimitRemaining]);

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
      
      // Check if user has 2FA enabled
      if (result.user && result.user.two_factor_enabled) {
        // Redirect to 2FA verification page
        navigate('/2fa-verify', { state: { username: result.user.username } });
      } else {
        // Small delay to ensure state updates propagate
        setTimeout(() => {
          navigate('/');
        }, 100);
      }
    } catch (err) {
      console.error('Google login error:', err);
      setError(err.response?.data?.error || err.message || 'Google login failed');
    } finally {
      setLoading(false);
    }
  };

  const googleLogin = useGoogleLogin({
    onSuccess: handleGoogleSuccess,
    onError: (error) => {
      console.error('Google login error:', error);
      setError('Google login failed');
    },
    flow: 'implicit',
  });

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      // Trim whitespace dari username dan password
      const trimmedUsername = username.trim();
      const trimmedPassword = password.trim();

      if (!trimmedUsername || !trimmedPassword) {
        setError('Username and password are required');
        setLoading(false);
        return;
      }

      const response = await login(trimmedUsername, trimmedPassword);
      
      // Check if user has 2FA enabled
      if (response.user && response.user.two_factor_enabled) {
        // Redirect to 2FA verification page
        navigate('/2fa-verify', { state: { username: trimmedUsername } });
      } else {
        // Login sukses dan tidak ada 2FA → navigate ke home
        navigate('/');
      }
    } catch (err) {
      // Check if rate limited
      if (err.response?.status === 429) {
        const retryAfter = err.response?.data?.retry_after || 180;
        const expiryTime = Date.now() + (retryAfter * 1000);
        
        // Store expiry time di localStorage
        localStorage.setItem('loginRateLimitExpiry', expiryTime.toString());
        
        setRateLimitRemaining(retryAfter);
        setError(`Too many login attempts. Please try again in ${retryAfter} seconds.`);
      }
      // Check if error adalah email not verified
      else if (err.response?.data?.needs_verification) {
        // Email not verified → need to verify first
        console.log('Email not verified, redirecting to OTP verification');
        
        // Send verification email
        try {
          await axios.post(
            `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-verification`,
            { email: username }
          );
          console.log('Verification email sent');
        } catch (emailErr) {
          console.error('Failed to send verification email:', emailErr);
          setError('Failed to send verification email. Please try again.');
          setLoading(false);
          return;
        }
        
        // Redirect ke OTP verification page dengan email & credentials untuk auto-login setelah verify
        navigate('/verify-otp', { state: { 
          email: username,
          credentials: { username, password }
        } });
      } else {
        // Other errors
        setError(err.response?.data?.error || 'Login failed');
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-white flex items-center justify-center p-4">
      <div className="w-full max-w-md border border-black p-8">
        <h1 className="text-3xl font-bold text-black mb-8 text-center">Login</h1>

        {error && (
          <div className={`border-2 p-4 mb-6 ${rateLimitRemaining > 0 ? 'border-red-600 bg-red-50 text-red-700' : 'border-black bg-white text-black'}`}>
            <p className="font-bold mb-2">{error}</p>
            {rateLimitRemaining > 0 && (
              <p className="text-sm font-bold">
                ⏱ Try again in: <span className="text-lg font-mono">{Math.floor(rateLimitRemaining / 60)}:{String(rateLimitRemaining % 60).padStart(2, '0')}</span>
              </p>
            )}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label className="block text-black font-bold mb-2">Username or Email</label>
            <input
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              required
              className="w-full border border-black p-2 bg-white text-black"
              placeholder="Enter username or email"
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
                placeholder="Enter your password"
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                className="absolute right-2 top-1/2 transform -translate-y-1/2 px-2 py-1 text-black font-bold border border-black hover:bg-black hover:text-white"
              >
                {showPassword ? 'Hide' : 'Show'}
              </button>
            </div>
          </div>

          <button
            type="submit"
            disabled={loading || rateLimitRemaining > 0}
            className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? 'Logging in...' : rateLimitRemaining > 0 ? `Locked (${rateLimitRemaining}s)` : 'Login'}
          </button>
        </form>

        <div className="mt-6 border-t border-black pt-6">
          <p className="text-center text-black mb-4">Or continue with:</p>
          <button
            onClick={() => googleLogin()}
            disabled={loading || rateLimitRemaining > 0}
            className="w-full bg-white border border-black text-black font-bold py-2 px-4 mb-3 hover:bg-black hover:text-white transition disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Google Login
          </button>
          <Link
            to="/web3-auth"
            onClick={(e) => rateLimitRemaining > 0 && e.preventDefault()}
            className={`w-full block text-center bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition ${rateLimitRemaining > 0 ? 'opacity-50 cursor-not-allowed' : ''}`}
          >
            Web3 Auth
          </Link>
        </div>

        <div className="mt-6 text-center">
          <p className="text-black">
            Don't have an account?{' '}
            <Link to="/register" className="font-bold border-b border-black hover:bg-black hover:text-white">
              Register
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
};

export default LoginPage;
