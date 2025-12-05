import api from './api';

const authService = {
  // User Registration
  register: async (username, email, password) => {
    const response = await api.post('/auth/register', {
      first_name: username,
      email,
      password,
    });
    return response.data;
  },

  // Email/Password Login
  login: async (username, password) => {
    const response = await api.post('/auth/login', {
      username,
      password,
    });

    // Check if MFA is required
    if (response.data.requires_mfa) {
      // Return MFA requirement info instead of completing login
      return {
        requires_mfa: true,
        mfa_methods: response.data.mfa_methods,
        temp_token: response.data.temp_token,
        user: response.data.user,
      };
    }

    // If no MFA required, complete login normally
    if (response.data.token) {
      localStorage.setItem('token', response.data.token);
      if (response.data.user) {
        localStorage.setItem('user', JSON.stringify(response.data.user));
      }
      window.dispatchEvent(new Event('auth-update'));
    }
    return response.data;
  },

  // Verify MFA Code
  verifyMFA: async (tempToken, method, code) => {
    const response = await api.post('/auth/verify-mfa', {
      temp_token: tempToken,
      method,
      code,
    });

    if (response.data.token) {
      localStorage.setItem('token', response.data.token);
      if (response.data.user) {
        localStorage.setItem('user', JSON.stringify(response.data.user));
      }
      window.dispatchEvent(new Event('auth-update'));
    }
    return response.data;
  },

  // Google OAuth Login
  googleLogin: async (token) => {
    const response = await api.post('/auth/google/callback', {
      token,
    });

    // Check if MFA is required
    if (response.data.requires_mfa) {
      return {
        requires_mfa: true,
        mfa_methods: response.data.mfa_methods,
        temp_token: response.data.temp_token,
        user: response.data.user,
      };
    }

    // If no MFA required, complete login normally
    if (response.data.token) {
      localStorage.setItem('token', response.data.token);
      if (response.data.user) {
        localStorage.setItem('user', JSON.stringify(response.data.user));
      }
      window.dispatchEvent(new Event('auth-update'));
    }
    return response.data;
  },

  // Get Web3 Challenge
  getWeb3Challenge: async (address) => {
    const response = await api.post('/auth/web3/challenge', {
      address,
    });
    return response.data;
  },

  // Verify Web3 Signature and Authenticate
  verifyWeb3Signature: async (address, challenge, signature) => {
    const response = await api.post('/auth/web3/verify', {
      address,
      challenge,
      signature,
    });
    if (response.data.token) {
      localStorage.setItem('token', response.data.token);
      // Store user object if available
      if (response.data.user) {
        localStorage.setItem('user', JSON.stringify(response.data.user));
      }
      // Dispatch custom event to notify AuthContext of the update
      window.dispatchEvent(new Event('auth-update'));
    }
    return response.data;
  },

  // Refresh JWT Token
  refreshToken: async (refreshToken) => {
    const response = await api.post('/auth/refresh', {}, {
      headers: {
        Authorization: `Bearer ${refreshToken}`,
      },
    });
    if (response.data.token) {
      localStorage.setItem('token', response.data.token);
    }
    return response.data;
  },

  // Logout (Revoke Token)
  logout: async () => {
    try {
      await api.post('/auth/logout');
    } finally {
      localStorage.removeItem('token');
    }
  },

  // Get current user from localStorage
  getCurrentUser: () => {
    const userStr = localStorage.getItem('user');
    if (!userStr) return null;
    try {
      return JSON.parse(userStr);
    } catch {
      return null;
    }
  },

  // Check if user is authenticated
  isAuthenticated: () => {
    const token = localStorage.getItem('token');
    return !!token;
  },

  // Get stored token
  getToken: () => localStorage.getItem('token'),

  // Request Password Reset
  requestPasswordReset: async (email) => {
    const response = await api.post('/auth/password/request-reset', { email });
    return response.data;
  },

  // Reset Password
  resetPassword: async (token, newPassword) => {
    const response = await api.post('/auth/password/reset', {
      token,
      new_password: newPassword,
    });
    return response.data;
  },
};

export default authService;
