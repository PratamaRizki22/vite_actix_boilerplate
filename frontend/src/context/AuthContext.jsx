import React, { createContext, useContext, useState, useEffect } from 'react';
import authService from '../services/authService';

const AuthContext = createContext(null);

export const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [token, setToken] = useState(localStorage.getItem('token'));
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  // Initialize auth state on mount
  useEffect(() => {
    const storedToken = localStorage.getItem('token');
    if (storedToken) {
      setToken(storedToken);
      const userData = authService.getCurrentUser();
      if (userData) {
        setUser(userData);
      }
    }
    setLoading(false);
  }, []);

  // Watch for token changes from localStorage and custom auth update events
  useEffect(() => {
    const handleStorageChange = () => {
      const storedToken = localStorage.getItem('token');
      if (storedToken && storedToken !== token) {
        console.log('Token updated via storage event');
        setToken(storedToken);
        const userData = authService.getCurrentUser();
        if (userData) {
          setUser(userData);
        }
      }
    };

    // Listen to custom auth update event (fired when token changes in same tab)
    const handleAuthUpdate = () => {
      const storedToken = localStorage.getItem('token');
      console.log('Token updated via custom event');
      setToken(storedToken);
      const userData = authService.getCurrentUser();
      if (userData) {
        setUser(userData);
      }
    };

    window.addEventListener('storage', handleStorageChange);
    window.addEventListener('auth-update', handleAuthUpdate);
    
    return () => {
      window.removeEventListener('storage', handleStorageChange);
      window.removeEventListener('auth-update', handleAuthUpdate);
    };
  }, [token]);

  const register = async (username, email, password) => {
    try {
      setError(null);
      setLoading(true);
      const response = await authService.register(username, email, password);
      return response;
    } catch (err) {
      const errorMsg = err.response?.data?.error || err.message;
      setError(errorMsg);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const login = async (username, password) => {
    try {
      setError(null);
      setLoading(true);
      const response = await authService.login(username, password);
      // authService sudah simpan token ke localStorage
      // Update state juga
      setToken(response.token);
      setUser(response.user);
      return response;
    } catch (err) {
      const errorMsg = err.response?.data?.error || err.message;
      setError(errorMsg);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const getWeb3Challenge = async (address) => {
    try {
      setError(null);
      const response = await authService.getWeb3Challenge(address);
      return response;
    } catch (err) {
      const errorMsg = err.response?.data?.error || err.message;
      setError(errorMsg);
      throw err;
    }
  };

  const verifyWeb3Signature = async (address, challenge, signature) => {
    try {
      setError(null);
      setLoading(true);
      const response = await authService.verifyWeb3Signature(address, challenge, signature);
      setToken(response.token);
      setUser(response.user);
      return response;
    } catch (err) {
      const errorMsg = err.response?.data?.error || err.message;
      setError(errorMsg);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const logout = async () => {
    try {
      setError(null);
      setLoading(true);
      await authService.logout();
      setUser(null);
      setToken(null);
    } catch (err) {
      const errorMsg = err.response?.data?.error || err.message;
      setError(errorMsg);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const value = {
    user,
    token,
    loading,
    error,
    isAuthenticated: !!token,
    register,
    login,
    logout,
    getWeb3Challenge,
    verifyWeb3Signature,
    setError,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
};
