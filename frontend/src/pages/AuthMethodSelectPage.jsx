import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import axios from 'axios';

const AuthMethodSelectPage = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [resendCooldown, setResendCooldown] = useState(0);
  const [codeExpiry, setCodeExpiry] = useState(null); // Start with null until backend responds
  const [isExpirySynced, setIsExpirySynced] = useState(false); // Flag to track if backend sync is done
  const [otpSent, setOtpSent] = useState(false);

  // Get state from location
  const state = location.state || {};
  let { email, username, password, isRegistration, temp_token, mfa_methods, user, can_skip_mfa, isLogin: explicitIsLogin } = state;

  // Debug logging
  console.log('AuthMethodSelectPage - location.state:', location.state);
  console.log('AuthMethodSelectPage - email:', email, 'temp_token:', temp_token, 'user:', user);

  // Normalize email for consistency
  if (email) {
    email = email.toLowerCase().trim()
  }

  // For login flow (has temp_token or explicit isLogin flag)
  const isLogin = !!temp_token || explicitIsLogin || !!user;
  
  // Cooldown timer untuk resend
  useEffect(() => {
    if (resendCooldown > 0) {
      const timer = setTimeout(() => {
        setResendCooldown(resendCooldown - 1);
      }, 1000);
      return () => clearTimeout(timer);
    }
  }, [resendCooldown]);

  // Code expiry timer - only countdown after backend sync
  useEffect(() => {
    if (codeExpiry === null || codeExpiry <= 0 || !isExpirySynced) return;

    const timer = setTimeout(() => {
      setCodeExpiry(codeExpiry - 1);
    }, 1000);
    return () => clearTimeout(timer);
  }, [codeExpiry, isExpirySynced]);

  // Handle expiry error
  useEffect(() => {
    if (codeExpiry === 0 && isExpirySynced && otpSent) {
      setError('Verification code has expired. Please request a new one.');
    }
  }, [codeExpiry, isExpirySynced, otpSent]);

  // Periodic sync dengan backend setiap 30 detik untuk keep timer accurate
  useEffect(() => {
    if (!email || !otpSent || !isExpirySynced || codeExpiry === 0) return;

    const syncInterval = setInterval(() => {
      checkCodeExpiry();
    }, 30000);

    return () => clearInterval(syncInterval);
  }, [email, otpSent, isExpirySynced, codeExpiry]);
  
  const checkCodeExpiry = async () => {
    try {
      const response = await axios.post(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/check-expiry`,
        { email }
      );
      
      if (response.data.has_code && response.data.expires_in_seconds) {
        setCodeExpiry(response.data.expires_in_seconds);
        setIsExpirySynced(true);
      } else {
        // No active code found
        setCodeExpiry(0);
        setIsExpirySynced(true);
      }
    } catch (err) {
      console.error('Failed to check code expiry:', err);
      // Fallback - assume no code
      setCodeExpiry(0);
      setIsExpirySynced(true);
    }
  };
  
  // For login flow, we need temp_token (email is in user object)
  // For registration flow, we need email
  const hasRequiredData = isLogin ? !!temp_token : !!email;
  
  if (!hasRequiredData) {
    return (
      <div className="min-h-screen bg-white flex items-center justify-center p-4">
        <div className="w-full max-w-md border border-black p-8 text-center">
          <h1 className="text-3xl font-bold text-black mb-4">Session Expired</h1>
          <p className="text-black mb-6">
            {isLogin ? 'Please login again.' : 'Please register again.'}
          </p>
          <button
            onClick={() => navigate(isLogin ? '/login' : '/register')}
            className="px-6 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
          >
            Back to {isLogin ? 'Login' : 'Register'}
          </button>
        </div>
      </div>
    );
  }

  const handleOTPVerification = () => {
    setLoading(true);
    setError('');
    
    if (isLogin) {
      // Login with OTP - go directly to OTP verify
      navigate('/verify-otp', {
        state: {
          email: user?.email,
          temp_token,
          mfa_methods,
          method: 'email',
          isLogin: true
        }
      });
    } else {
      // Registration - go to OTP verify page, user will click "Send Code" to get OTP
      navigate('/verify-otp', {
        state: {
          email,
          credentials: { username, password },
          isRegistration: true
        }
      });
    }
    setLoading(false);
  };

  const handleTwoFactorSetup = () => {
    setLoading(true);
    
    if (isLogin) {
      // Login with 2FA
      navigate('/2fa-verify', {
        state: {
          temp_token,
          mfa_methods,
          user,
          can_skip_mfa,
          isLogin: true
        }
      });
    } else {
      // Registration - setup 2FA
      navigate('/2fa-setup', {
        state: {
          email,
          username,
          password,
          isRegistration: true
        }
      });
    }
  };

  return (
    <div className="min-h-screen bg-white flex items-center justify-center p-4">
      <div className="w-full max-w-md border border-black p-8">
        <h1 className="text-3xl font-bold text-black mb-2 text-center">
          {isLogin ? 'Verify Your Identity' : 'Verify Your Email'}
        </h1>
        <p className="text-gray-600 text-center mb-8">
          {isLogin 
            ? 'Choose how you want to verify your login' 
            : 'Choose how you want to verify your email address'}
        </p>

        {error && (
          <div className="border border-black bg-white p-4 mb-6 text-black">
            {error}
          </div>
        )}

        <div className="space-y-4">
          {/* OTP Option */}
          <button
            onClick={handleOTPVerification}
            disabled={loading}
            className="w-full border border-black p-6 bg-white text-black font-bold hover:bg-black hover:text-white transition disabled:opacity-50"
          >
            <div className="text-xl mb-2">Email OTP</div>
            <div className="text-sm font-normal text-left">
              {isLogin 
                ? 'Receive a one-time code via email to verify this login'
                : 'Receive a one-time code via email. Quick and simple verification.'}
            </div>
          </button>

          {/* 2FA Option */}
          <button
            onClick={handleTwoFactorSetup}
            disabled={loading}
            className="w-full border border-black p-6 bg-white text-black font-bold hover:bg-black hover:text-white transition disabled:opacity-50"
          >
            <div className="text-xl mb-2">Two-Factor Authentication</div>
            <div className="text-sm font-normal text-left">
              {isLogin
                ? 'Use your authenticator app to verify this login'
                : 'Setup authenticator app (Google Authenticator, Authy, etc.) for enhanced security.'}
            </div>
          </button>
        </div>

        <div className="mt-8 text-center">
          <button
            onClick={() => navigate(isLogin ? '/login' : '/register')}
            className="text-black underline hover:text-gray-600"
          >
            Back to {isLogin ? 'Login' : 'Register'}
          </button>
        </div>
      </div>
    </div>
  );
};

export default AuthMethodSelectPage;
