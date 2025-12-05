import React, { useState, useEffect, useRef } from 'react';
import { useAuth } from '../context/AuthContext';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';
import { QRCodeSVG } from 'qrcode.react';
import twoFactorService from '../services/twoFactorService';

const ProfilePage = () => {
  const { user, setError: setAuthError } = useAuth();
  const navigate = useNavigate();

  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [isEditingUsername, setIsEditingUsername] = useState(false);
  const [isEditingEmail, setIsEditingEmail] = useState(false);
  const [verificationCode, setVerificationCode] = useState('');
  const [verificationMethod, setVerificationMethod] = useState('email'); // 'email' or 'totp'
  const [showVerificationInput, setShowVerificationInput] = useState(false);
  const [sendingCode, setSendingCode] = useState(false);
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [loading, setLoading] = useState(false);
  const [verifyEmailLoading, setVerifyEmailLoading] = useState(false);
  const [linkWalletLoading, setLinkWalletLoading] = useState(false);
  const [disable2FALoading, setDisable2FALoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [is2FAEnabled, setIs2FAEnabled] = useState(false);
  const [show2FASetup, setShow2FASetup] = useState(false);
  const [twoFALoading, setTwoFALoading] = useState(false);
  const [qrCode, setQrCode] = useState('');
  const [secret, setSecret] = useState('');
  const [recoveryCodes, setRecoveryCodes] = useState([]);
  const [copied, setCopied] = useState(false);
  const [copiedCodes, setCopiedCodes] = useState(false);
  const [showPasswordConfirm, setShowPasswordConfirm] = useState(false);
  const [showUnlinkWalletConfirm, setShowUnlinkWalletConfirm] = useState(false);
  const [showDisable2FAConfirm, setShowDisable2FAConfirm] = useState(false);
  const [passwordConfirmValue, setPasswordConfirmValue] = useState('');
  const [pendingUpdateData, setPendingUpdateData] = useState(null);
  const [passwordChangeStep, setPasswordChangeStep] = useState(0); // 0: Initial, 1: Verification, 2: New Password
  const [tempToken, setTempToken] = useState('');
  const [countdown, setCountdown] = useState(0);
  const [show2FAConfirm, setShow2FAConfirm] = useState(false);
  const [twoFACode, setTwoFACode] = useState('');
  const qrCodeRef = useRef(null);

  // Countdown timer effect
  useEffect(() => {
    if (countdown > 0) {
      const timer = setTimeout(() => setCountdown(countdown - 1), 1000);
      return () => clearTimeout(timer);
    }
  }, [countdown]);

  // Initialize state from user object and fetch fresh data
  useEffect(() => {
    const initData = async () => {
      if (user) {
        setUsername(user.username || '');
        setEmail(user.email || '');
        setIs2FAEnabled(user.two_factor_enabled || user.totp_enabled || false);
      }

      // Fetch fresh user data to ensure 2FA status is accurate
      try {
        const token = localStorage.getItem('token');
        if (token) {
          const response = await axios.get(`${import.meta.env.VITE_API_BASE_URL}/api/auth/me`, {
            headers: { Authorization: `Bearer ${token}` }
          });
          if (response.data.user) {
            const userData = response.data.user;
            setUsername(userData.username || '');
            setEmail(userData.email || '');
            setIs2FAEnabled(userData.two_factor_enabled || userData.totp_enabled || false);
          }
        }
      } catch (err) {
        console.error('Failed to fetch fresh profile data:', err);
      }
    };

    initData();
  }, [user]);

  // Update verification method based on 2FA status
  useEffect(() => {
    if (is2FAEnabled) {
      setVerificationMethod('totp');
      setShowVerificationInput(true); // TOTP always ready
    } else {
      setVerificationMethod('email');
      setShowVerificationInput(false);
    }
  }, [is2FAEnabled]);

  const handleSendVerificationCode = async () => {
    setError('');
    setSuccess('');
    setSendingCode(true);

    try {
      await axios.post(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-verification`,
        { email: user.email }
      );

      setSuccess('Verification code sent to your email.');
      setShowVerificationInput(true);
      setCountdown(60);
    } catch (err) {
      console.error('Send verification error:', err);
      const errorMsg = err.response?.data?.error || err.message || 'Failed to send verification code';
      setError(errorMsg);
    } finally {
      setSendingCode(false);
    }
  };

  const handleNextStep = async () => {
    setError('');
    setSuccess('');
    setLoading(true);

    try {
      const token = localStorage.getItem('token');
      const response = await axios.post(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/password/verify-code`,
        {
          verification_code: verificationCode,
          verification_method: verificationMethod
        },
        {
          headers: {
            Authorization: `Bearer ${token}`,
            'Content-Type': 'application/json',
          },
        }
      );

      if (response.data.success) {
        setTempToken(response.data.temp_token);
        setPasswordChangeStep(2);
        setSuccess('Code verified successfully');
        setTimeout(() => setSuccess(''), 2000);
      }
    } catch (err) {
      console.error('Verification error:', err);
      const errorMsg = err.response?.data?.error || err.message || 'Failed to verify code';
      setError(errorMsg);
    } finally {
      setLoading(false);
    }
  };

  const handleConfirmUpdateWith2FA = async () => {
    if (!twoFACode) {
      setError('Please enter your 2FA code');
      return;
    }

    setLoading(true);
    setError('');

    try {
      const token = localStorage.getItem('token');
      const updateData = {
        ...pendingUpdateData,
        verification_code: twoFACode
      };

      const response = await axios.put(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/profile`,
        updateData,
        {
          headers: {
            Authorization: `Bearer ${token}`,
            'Content-Type': 'application/json',
          },
        }
      );

      // Update localStorage with new user data
      localStorage.setItem('user', JSON.stringify(response.data));

      // If email was updated, handle verification flow
      if (updateData.email) {
        setSuccess('Profile updated! Please verify your new email.');
        // Send verification email
        try {
          await axios.post(`${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-verification`, { email: updateData.email });
        } catch (emailErr) {
          console.error('Failed to send verification email:', emailErr);
          setError('Profile updated but failed to send verification email. Please request it manually.');
        }

        // Redirect to OTP page
        setTimeout(() => {
          navigate('/verify-otp', { state: { email: updateData.email, isRegistration: true } });
        }, 1500);
        return;
      }

      setSuccess('Profile updated successfully!');
      setIsEditingUsername(false);
      setIsEditingEmail(false);
      setShow2FAConfirm(false);
      setTwoFACode('');
      setPendingUpdateData(null);
    } catch (err) {
      console.error('Update profile 2FA error:', err);
      const errorMsg = err.response?.data?.error || err.message || 'Failed to update profile';
      setError(errorMsg);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdateProfile = async (e) => {
    if (e) e.preventDefault();
    setError('');
    setSuccess('');
    setLoading(true);

    try {
      const token = localStorage.getItem('token');
      const updateData = {};

      // Only add fields that have changed
      if (username && username !== user.username) {
        if (!is2FAEnabled) {
          setError('Two-factor authentication (2FA) must be enabled to update your username.');
          setLoading(false);
          return;
        }
        updateData.username = username;
      }
      if (email && email !== user.email) {
        if (!is2FAEnabled) {
          setError('Two-factor authentication (2FA) must be enabled to update your email.');
          setLoading(false);
          return;
        }
        updateData.email = email;
      }

      if (Object.keys(updateData).length === 0) {
        setError('No profile changes made');
        setLoading(false);
        return;
      }

      // If there are updates, do them
      try {
        const response = await axios.put(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/profile`,
          updateData,
          {
            headers: {
              Authorization: `Bearer ${token}`,
              'Content-Type': 'application/json',
            },
          }
        );

        // Update localStorage with new user data
        localStorage.setItem('user', JSON.stringify(response.data));

        // If email was updated, handle verification flow
        if (updateData.email) {
          setSuccess('Profile updated! Please verify your new email.');
          // Send verification email
          try {
            await axios.post(`${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-verification`, { email: updateData.email });
          } catch (emailErr) {
            console.error('Failed to send verification email:', emailErr);
            setError('Profile updated but failed to send verification email. Please request it manually.');
          }

          // Redirect to OTP page
          setTimeout(() => {
            navigate('/verify-otp', { state: { email: updateData.email, isRegistration: true } });
          }, 1500);
          return;
        }

        setSuccess('Profile updated successfully!');
        setIsEditingUsername(false);
        setIsEditingEmail(false);
      } catch (err) {
        // Check if 2FA confirmation is required
        if (err.response?.data?.require_2fa) {
          setPendingUpdateData(updateData);
          setShow2FAConfirm(true);
          setLoading(false);
          return;
        }
        // Check if password confirmation is required (legacy)
        if (err.response?.data?.require_password) {
          setPendingUpdateData(updateData);
          setShowPasswordConfirm(true);
          setLoading(false);
          return;
        }
        throw err;
      }

      setLoading(false);
    } catch (err) {
      console.error('Update profile error:', err);
      const errorMsg = err.response?.data?.error || err.message || 'Failed to update profile';
      setError(errorMsg);
      setLoading(false);
    }
  };

  const handleUpdatePassword = async (e) => {
    if (e) e.preventDefault();
    setError('');
    setSuccess('');
    setLoading(true);

    try {
      const token = localStorage.getItem('token');

      if (!newPassword) {
        setError('Please enter a new password');
        setLoading(false);
        return;
      }

      if (!verificationCode) {
        setError('Verification code is required');
        setLoading(false);
        return;
      }

      if (newPassword !== confirmPassword) {
        setError('Passwords do not match');
        setLoading(false);
        return;
      }
      if (newPassword.length < 8) {
        setError('Password must be at least 8 characters');
        setLoading(false);
        return;
      }
      if (!/[A-Z]/.test(newPassword) || !/[a-z]/.test(newPassword) || !/[0-9]/.test(newPassword)) {
        setError('Password must contain uppercase, lowercase, and number');
        setLoading(false);
        return;
      }

      // Call dedicated password change endpoint
      try {
        await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/password/change`,
          {
            verification_code: verificationCode,
            verification_method: verificationMethod,
            temp_token: tempToken,
            new_password: newPassword
          },
          {
            headers: {
              Authorization: `Bearer ${token}`,
              'Content-Type': 'application/json',
            },
          }
        );

        setSuccess('Password updated successfully!');
        // Clear password fields
        setVerificationCode('');
        setTempToken('');
        setNewPassword('');
        setConfirmPassword('');
        setPasswordChangeStep(0);
        if (verificationMethod === 'email') {
          setShowVerificationInput(false);
        }
      } catch (passwordErr) {
        console.error('Password change error:', passwordErr);
        const errorMsg = passwordErr.response?.data?.error || passwordErr.message || 'Failed to change password';
        setError(errorMsg);
      }

      setLoading(false);
    } catch (err) {
      console.error('Update password error:', err);
      setError('Failed to update password');
      setLoading(false);
    }
  };

  const handleConfirmPasswordUpdate = async () => {
    if (!passwordConfirmValue) {
      setError('Password is required');
      return;
    }

    setLoading(true);
    setShowPasswordConfirm(false);

    try {
      const token = localStorage.getItem('token');
      const updateData = {
        ...pendingUpdateData,
        current_password: passwordConfirmValue
      };

      const response = await axios.put(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/profile`,
        updateData,
        {
          headers: {
            Authorization: `Bearer ${token}`,
            'Content-Type': 'application/json',
          },
        }
      );

      // Update localStorage with new user data
      localStorage.setItem('user', JSON.stringify(response.data));

      // If email was updated, handle verification flow
      if (updateData.email) {
        setSuccess('Profile updated! Please verify your new email.');
        // Send verification email
        try {
          await axios.post(`${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-verification`, { email: updateData.email });
        } catch (emailErr) {
          console.error('Failed to send verification email:', emailErr);
        }

        // Redirect to OTP page
        setTimeout(() => {
          navigate('/verify-otp', { state: { email: updateData.email, isRegistration: true } });
        }, 1500);
      } else {
        setSuccess('Profile updated successfully!');
      }

      setPendingUpdateData(null);
      setPasswordConfirmValue('');
    } catch (err) {
      console.error('Update profile error:', err);
      const errorMsg = err.response?.data?.error || err.message || 'Failed to update profile';
      setError(errorMsg);
      // Re-open modal if password was wrong
      if (errorMsg.toLowerCase().includes('password')) {
        setShowPasswordConfirm(true);
      }
    } finally {
      setLoading(false);
    }
  };

  const handleSendVerificationEmail = async () => {
    setError('');
    setSuccess('');
    setVerifyEmailLoading(true);

    try {
      const response = await axios.post(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-verification`,
        { email: user.email }
      );

      setSuccess('Verification email sent! Check your inbox.');

      // Navigate to OTP verify page
      setTimeout(() => {
        navigate('/verify-otp', {
          state: {
            email: user.email,
            isFromProfilePage: true
          }
        });
      }, 2000);
    } catch (err) {
      console.error('Send verification error:', err);
      const errorMsg = err.response?.data?.error || err.message || 'Failed to send verification email';
      setError(errorMsg);
    } finally {
      setVerifyEmailLoading(false);
    }
  };

  // 2FA Setup Functions
  const load2FAQrCode = async () => {
    try {
      setTwoFALoading(true);
      const data = await twoFactorService.setup2FA();
      setQrCode(data.qr_code_url);
      setSecret(data.secret);
      setRecoveryCodes(data.recovery_codes || []);
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to setup 2FA');
    } finally {
      setTwoFALoading(false);
    }
  };

  const copyRecoveryCodesToClipboard = () => {
    const codesText = recoveryCodes.join('\n');
    navigator.clipboard.writeText(codesText);
    setCopiedCodes(true);
    setTimeout(() => setCopiedCodes(false), 2000);
  };

  const downloadRecoveryCodes = () => {
    const username = user?.username || 'user';
    const codesText = `USH Account Recovery Codes\n\nUsername: ${username}\nGenerated: ${new Date().toLocaleString()}\n\n${recoveryCodes.join('\n')}\n\nIMPORTANT:\n- Save these codes in a safe place\n- Each code can only be used once\n- Use these if you lose access to your authenticator app`;
    const blob = new Blob([codesText], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `ush-recovery-codes-${username}.txt`;
    link.click();
    URL.revokeObjectURL(url);
  };

  const handle2FASetup = async () => {
    setError('');
    setSuccess('');
    setShow2FASetup(true);
    await load2FAQrCode();
  };

  const handleDisable2FA = () => {
    setShowDisable2FAConfirm(true);
  };

  const confirmDisable2FA = async () => {
    setShowDisable2FAConfirm(false);
    setError('');
    setSuccess('');
    setDisable2FALoading(true);

    try {
      const result = await twoFactorService.disable2FA();
      if (result.success) {
        setSuccess('2FA has been successfully disabled');
        setIs2FAEnabled(false);

        // Refresh user data
        const token = localStorage.getItem('token');
        const userResponse = await fetch(`${import.meta.env.VITE_API_BASE_URL}/api/auth/me`, {
          headers: {
            Authorization: `Bearer ${token}`
          }
        });
        if (userResponse.ok) {
          const data = await userResponse.json();
          const userData = data.user;
          localStorage.setItem('user', JSON.stringify(userData));
        }

        // Auto-dismiss success message after 3 seconds
        setTimeout(() => setSuccess(''), 3000);
      } else {
        setError(result.message || 'Failed to disable 2FA');
      }
    } catch (err) {
      console.error('Disable 2FA error:', err);
      setError(err.response?.data?.error || 'Failed to disable 2FA. Please try again.');
    } finally {
      setDisable2FALoading(false);
    }
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(secret);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleLinkWeb3Wallet = async () => {
    if (!window.ethereum) {
      setError('MetaMask not installed. Please install MetaMask to link wallet.');
      return;
    }

    setError('');
    setSuccess('');
    setLinkWalletLoading(true);

    try {
      // Request account access
      const accounts = await window.ethereum.request({
        method: 'eth_requestAccounts',
      });

      if (!accounts || accounts.length === 0) {
        setError('No wallet account selected');
        return;
      }

      const walletAddress = accounts[0];
      const token = localStorage.getItem('token');

      // Update user profile with wallet address
      const response = await axios.put(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/profile`,
        { wallet_address: walletAddress },
        {
          headers: {
            Authorization: `Bearer ${token}`,
            'Content-Type': 'application/json',
          },
        }
      );

      setSuccess(`Wallet linked successfully! ${walletAddress.substring(0, 6)}...${walletAddress.substring(walletAddress.length - 4)}`);

      // Update localStorage
      localStorage.setItem('user', JSON.stringify(response.data));

      setTimeout(() => {
        window.location.reload();
      }, 2000);
    } catch (err) {
      console.error('Link wallet error:', err);
      if (err.message.includes('User rejected')) {
        setError('Wallet connection cancelled');
      } else {
        const errorMsg = err.response?.data?.error || err.message || 'Failed to link wallet';
        setError(errorMsg);
      }
      setLinkWalletLoading(false);
    }
  };

  const handleUnlinkWeb3Wallet = () => {
    setShowUnlinkWalletConfirm(true);
  };

  const confirmUnlinkWeb3Wallet = async () => {
    setShowUnlinkWalletConfirm(false);
    setError('');
    setSuccess('');
    setLinkWalletLoading(true);

    try {
      const token = localStorage.getItem('token');

      // Remove wallet address
      const response = await axios.put(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/profile`,
        { wallet_address: null },
        {
          headers: {
            Authorization: `Bearer ${token}`,
            'Content-Type': 'application/json',
          },
        }
      );

      setSuccess('Wallet unlinked successfully!');

      // Update localStorage
      localStorage.setItem('user', JSON.stringify(response.data));

      setTimeout(() => {
        window.location.reload();
      }, 2000);
    } catch (err) {
      console.error('Unlink wallet error:', err);
      const errorMsg = err.response?.data?.error || err.message || 'Failed to unlink wallet';
      setError(errorMsg);
    } finally {
      setLinkWalletLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-white pt-24 pb-8">
      {/* Toast Notifications - Fixed at top */}
      {error && (
        <div
          className="fixed top-4 left-1/2 transform -translate-x-1/2 z-50 max-w-md w-full mx-4 border-2 border-red-600 bg-red-50 text-red-700 p-4 font-bold shadow-lg transition-all duration-300 ease-in-out"
          style={{
            animation: 'slideDown 0.3s ease-out'
          }}
        >
          <div className="flex items-center justify-between">
            <span>{error}</span>
            <button
              onClick={() => setError('')}
              className="ml-4 text-red-700 hover:text-red-900 font-bold"
            >
              ✕
            </button>
          </div>
        </div>
      )}

      {success && (
        <div
          className="fixed top-4 left-1/2 transform -translate-x-1/2 z-50 max-w-md w-full mx-4 border-2 border-green-600 bg-green-50 text-green-700 p-4 font-bold shadow-lg transition-all duration-300 ease-in-out"
          style={{
            animation: 'slideDown 0.3s ease-out'
          }}
        >
          <div className="flex items-center justify-between">
            <span>{success}</span>
            <button
              onClick={() => setSuccess('')}
              className="ml-4 text-green-700 hover:text-green-900 font-bold"
            >
              ✕
            </button>
          </div>
        </div>
      )}

      <div className="container mx-auto px-4">
        <div className="max-w-2xl mx-auto">
          <h1 className="text-3xl font-bold text-black mb-8">Profile Settings</h1>

          <form className="space-y-6">
            {/* User Info Section */}
            <div className="border border-black p-6">
              <h2 className="text-xl font-bold text-black mb-4">User Information</h2>

              <div className="space-y-4">
                <div>
                  <label className="block text-black font-bold mb-2">Username</label>
                  <div className="flex gap-2 items-center">
                    {isEditingUsername ? (
                      <>
                        <input
                          type="text"
                          value={username}
                          onChange={(e) => setUsername(e.target.value)}
                          className="flex-1 border border-black p-2 bg-white text-black"
                          placeholder="Enter new username"
                        />
                        <button
                          type="button"
                          onClick={() => setIsEditingUsername(false)}
                          className="px-3 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
                        >
                          Cancel
                        </button>
                      </>
                    ) : (
                      <>
                        <input
                          type="text"
                          value={username}
                          disabled
                          className="flex-1 border border-black p-2 bg-gray-100 text-black cursor-not-allowed"
                        />
                        <button
                          type="button"
                          onClick={() => setIsEditingUsername(true)}
                          className="px-3 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
                        >
                          Edit
                        </button>
                      </>
                    )}
                  </div>
                </div>

                <div>
                  <label className="block text-black font-bold mb-2">Email</label>
                  <div className="flex gap-2 items-center">
                    {isEditingEmail ? (
                      <>
                        <input
                          type="email"
                          value={email}
                          onChange={(e) => setEmail(e.target.value)}
                          className="flex-1 border border-black p-2 bg-white text-black"
                          placeholder="Enter new email"
                        />
                        <button
                          type="button"
                          onClick={() => setIsEditingEmail(false)}
                          className="px-3 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
                        >
                          Cancel
                        </button>
                      </>
                    ) : (
                      <>
                        <input
                          type="email"
                          value={email}
                          disabled
                          className="flex-1 border border-black p-2 bg-gray-100 text-black cursor-not-allowed"
                        />
                        <button
                          type="button"
                          onClick={() => setIsEditingEmail(true)}
                          className="px-3 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
                        >
                          Edit
                        </button>
                      </>
                    )}
                  </div>
                </div>

                <div>
                  <label className="block text-black font-bold mb-2">User ID</label>
                  <input
                    type="text"
                    value={user.id}
                    disabled
                    className="w-full border border-black p-2 bg-gray-100 text-black cursor-not-allowed"
                  />
                </div>

                <div>
                  <label className="block text-black font-bold mb-2">Role</label>
                  <input
                    type="text"
                    value={user.role}
                    disabled
                    className="w-full border border-black p-2 bg-gray-100 text-black cursor-not-allowed"
                  />
                </div>

                <div>
                  <label className="block text-black font-bold mb-2">Email Verified</label>
                  <div className="flex gap-2 items-center">
                    <input
                      type="text"
                      value={user.email_verified ? 'Yes ✓' : 'No'}
                      disabled
                      className={`flex-1 border border-black p-2 ${user.email_verified ? 'bg-green-50 text-green-700' : 'bg-red-50 text-red-700'} font-bold cursor-not-allowed`}
                    />
                    {!user.email_verified && (
                      <button
                        type="button"
                        onClick={handleSendVerificationEmail}
                        disabled={verifyEmailLoading}
                        className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap"
                      >
                        {verifyEmailLoading ? 'Sending...' : 'Verify'}
                      </button>
                    )}
                  </div>
                </div>
              </div>

              {/* Update Profile Button */}
              <div className="mt-6">
                <button
                  type="button"
                  onClick={handleUpdateProfile}
                  disabled={loading || (!isEditingUsername && !isEditingEmail)}
                  className="w-full px-6 py-3 border border-black bg-black text-white font-bold hover:bg-white hover:text-black transition disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-black disabled:hover:text-white"
                >
                  {loading ? 'Updating...' : 'Update Profile'}
                </button>
              </div>
            </div>

            {/* Password Change Section */}
            <div className="border border-black p-6">
              <h2 className="text-xl font-bold text-black mb-4">Change Password</h2>

              {passwordChangeStep === 0 && (
                <div>
                  <p className="text-black text-sm mb-4">Secure your account by updating your password regularly.</p>
                  <button
                    type="button"
                    onClick={() => setPasswordChangeStep(1)}
                    className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
                  >
                    Start Password Change
                  </button>
                </div>
              )}

              {passwordChangeStep === 1 && (
                <div className="space-y-4">
                  <div className="flex justify-between items-center mb-2">
                    <h3 className="font-bold text-lg">Step 1: Verification</h3>
                    <span className="text-xs font-bold bg-gray-200 px-2 py-1 rounded">1/2</span>
                  </div>

                  {/* Verification Method Selection */}
                  <div>
                    <label className="block text-black font-bold mb-2">Verification Method</label>
                    <div className="flex gap-4">
                      <label className="flex items-center gap-2 cursor-pointer">
                        <input
                          type="radio"
                          name="verificationMethod"
                          value="email"
                          checked={verificationMethod === 'email'}
                          onChange={() => {
                            setVerificationMethod('email');
                            setShowVerificationInput(false);
                          }}
                          className="accent-black"
                        />
                        <span className="text-black">Email OTP</span>
                      </label>
                      {is2FAEnabled && (
                        <label className="flex items-center gap-2 cursor-pointer">
                          <input
                            type="radio"
                            name="verificationMethod"
                            value="totp"
                            checked={verificationMethod === 'totp'}
                            onChange={() => {
                              setVerificationMethod('totp');
                              setShowVerificationInput(true);
                            }}
                            className="accent-black"
                          />
                          <span className="text-black">Authenticator App (2FA)</span>
                        </label>
                      )}
                    </div>
                  </div>

                  {/* Verification Code Input */}
                  <div>
                    <label className="block text-black font-bold mb-2">
                      {verificationMethod === 'totp' ? '2FA Code' : 'Email Verification Code'}
                    </label>
                    <div className="flex gap-2">
                      {verificationMethod === 'email' && !showVerificationInput && (
                        <button
                          type="button"
                          onClick={handleSendVerificationCode}
                          disabled={sendingCode}
                          className="bg-black text-white font-bold py-2 px-4 hover:bg-gray-800 transition disabled:opacity-50"
                        >
                          {sendingCode ? 'Sending...' : 'Send Code'}
                        </button>
                      )}
                      {(showVerificationInput || verificationMethod === 'totp') && (
                        <div className="flex-1 flex gap-2">
                          <input
                            type="text"
                            value={verificationCode}
                            onChange={(e) => setVerificationCode(e.target.value)}
                            className="flex-1 border border-black p-2 bg-white text-black"
                            placeholder={verificationMethod === 'totp' ? "Enter 6-digit code" : "Enter email code"}
                          />
                          {verificationMethod === 'email' && (
                            <button
                              type="button"
                              onClick={handleSendVerificationCode}
                              disabled={sendingCode || countdown > 0}
                              className="bg-white text-black border border-black font-bold py-2 px-4 hover:bg-gray-100 transition disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap min-w-[120px]"
                            >
                              {countdown > 0 ? `Resend (${countdown}s)` : (sendingCode ? 'Sending...' : 'Resend Code')}
                            </button>
                          )}
                        </div>
                      )}
                    </div>
                    {verificationMethod === 'email' && showVerificationInput && (
                      <p className="text-xs text-gray-500 mt-1">Code sent to {user.email}</p>
                    )}
                  </div>

                  <div className="flex gap-2 mt-6">
                    <button
                      type="button"
                      onClick={() => setPasswordChangeStep(0)}
                      className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-gray-100 transition"
                    >
                      Cancel
                    </button>
                    <button
                      type="button"
                      onClick={handleNextStep}
                      disabled={!verificationCode || loading}
                      className="px-4 py-2 border border-black bg-black text-white font-bold hover:bg-gray-800 transition disabled:opacity-50"
                    >
                      {loading ? 'Verifying...' : 'Next'}
                    </button>
                  </div>
                </div>
              )}

              {passwordChangeStep === 2 && (
                <div className="space-y-4">
                  <div className="flex justify-between items-center mb-2">
                    <h3 className="font-bold text-lg">Step 2: New Password</h3>
                    <span className="text-xs font-bold bg-gray-200 px-2 py-1 rounded">2/2</span>
                  </div>

                  <div>
                    <label className="block text-black font-bold mb-2">New Password</label>
                    <input
                      type={showPassword ? 'text' : 'password'}
                      value={newPassword}
                      onChange={(e) => setNewPassword(e.target.value)}
                      className="w-full border border-black p-2 bg-white text-black"
                      placeholder="Enter new password (min 8 chars, 1 uppercase, 1 lowercase, 1 number)"
                    />
                  </div>

                  <div>
                    <label className="block text-black font-bold mb-2">Confirm New Password</label>
                    <div className="relative">
                      <input
                        type={showPassword ? 'text' : 'password'}
                        value={confirmPassword}
                        onChange={(e) => setConfirmPassword(e.target.value)}
                        className="w-full border border-black p-2 pr-12 bg-white text-black"
                        placeholder="Confirm new password"
                      />
                      <button
                        type="button"
                        onClick={() => setShowPassword(!showPassword)}
                        className="absolute right-2 top-1/2 transform -translate-y-1/2 px-2 py-1 text-black font-bold text-xs border border-black hover:bg-black hover:text-white"
                      >
                        {showPassword ? 'Hide' : 'Show'}
                      </button>
                    </div>
                  </div>

                  <div className="flex gap-2 mt-6">
                    <button
                      type="button"
                      onClick={() => setPasswordChangeStep(1)}
                      className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-gray-100 transition"
                    >
                      Back
                    </button>
                    <button
                      type="button"
                      onClick={handleUpdatePassword}
                      disabled={loading || !newPassword || !confirmPassword}
                      className="px-4 py-2 border border-black bg-black text-white font-bold hover:bg-gray-800 transition disabled:opacity-50"
                    >
                      {loading ? 'Updating...' : 'Update Password'}
                    </button>
                  </div>
                </div>
              )}
            </div>


            {/* Web3 Wallet Section */}
            <div className="border border-black p-6">
              <h2 className="text-xl font-bold text-black mb-4">Web3 Wallet</h2>

              {user.wallet_address ? (
                <div className="space-y-4">
                  <div>
                    <label className="block text-black font-bold mb-2">Connected Wallet</label>
                    <div className="flex gap-2 items-center">
                      <input
                        type="text"
                        value={user.wallet_address}
                        disabled
                        className="flex-1 border border-black p-2 bg-green-50 text-green-700 font-bold cursor-not-allowed text-xs break-all"
                      />
                      <span className="text-green-600 font-bold">✓</span>
                    </div>
                  </div>
                  <p className="text-black text-sm">Your Web3 wallet is connected and ready to use.</p>
                  <button
                    type="button"
                    onClick={handleUnlinkWeb3Wallet}
                    disabled={linkWalletLoading}
                    className="w-full px-4 py-2 border border-red-600 bg-red-50 text-red-700 font-bold hover:bg-red-600 hover:text-white transition disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {linkWalletLoading ? 'Processing...' : 'Unlink Wallet'}
                  </button>
                </div>
              ) : (
                <div className="space-y-4">
                  <p className="text-black text-sm">No wallet connected yet. Connect your MetaMask wallet to enable Web3 features.</p>
                  <button
                    type="button"
                    onClick={handleLinkWeb3Wallet}
                    disabled={linkWalletLoading}
                    className="w-full px-4 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {linkWalletLoading ? 'Connecting...' : 'Connect MetaMask Wallet'}
                  </button>
                </div>
              )}
            </div>

            {/* Two-Factor Authentication Section */}
            <div className="border border-black p-6">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-xl font-bold text-black">Two-Factor Authentication</h2>
                {is2FAEnabled && (
                  <span className="bg-green-50 text-green-700 px-3 py-1 border border-green-600 font-bold text-sm">
                    ✓ Enabled
                  </span>
                )}
              </div>

              {is2FAEnabled ? (
                <div className="space-y-4">
                  <p className="text-black text-sm">
                    Two-factor authentication is active on your account. You will be required to enter a code from your authenticator app when logging in.
                  </p>
                  <div className="p-4 border border-green-600 bg-green-50 text-green-700 font-bold text-sm rounded">
                    <p>✓ Your account is protected with 2FA</p>
                  </div>
                  <button
                    type="button"
                    onClick={handleDisable2FA}
                    disabled={disable2FALoading}
                    className="w-full px-4 py-2 border border-red-600 bg-red-50 text-red-700 font-bold hover:bg-red-600 hover:text-white transition disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {disable2FALoading ? 'Disabling...' : 'Disable 2FA'}
                  </button>
                </div>
              ) : (
                <>
                  {!show2FASetup ? (
                    <div className="space-y-4">
                      <p className="text-black text-sm">
                        Add an extra layer of security to your account with two-factor authentication (2FA).
                      </p>
                      <button
                        type="button"
                        onClick={handle2FASetup}
                        className="w-full px-4 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
                      >
                        Enable 2FA
                      </button>
                    </div>
                  ) : (
                    <div className="space-y-6">
                      {twoFALoading ? (
                        <div className="text-center text-black">Loading 2FA setup...</div>
                      ) : (
                        <>
                          {/* QR Code Section */}
                          <div>
                            <p className="text-black font-bold mb-4">
                              Step 1: Scan QR Code with Authenticator App
                            </p>
                            <div className="p-6 border border-black text-center bg-white">
                              {qrCode ? (
                                <>
                                  <div ref={qrCodeRef} className="bg-white p-4 border-2 border-black inline-block mb-4">
                                    <QRCodeSVG
                                      value={qrCode}
                                      size={256}
                                      level="H"
                                      includeMargin={true}
                                      renderAs="svg"
                                    />
                                  </div>
                                  <p className="text-sm text-black mb-3">
                                    Scan this QR code with any authenticator app (Google Authenticator, Microsoft Authenticator, Authy, etc.)
                                  </p>
                                  <button
                                    type="button"
                                    onClick={() => {
                                      if (qrCodeRef.current) {
                                        const svg = qrCodeRef.current.querySelector('svg');
                                        if (svg) {
                                          const canvas = document.createElement('canvas');
                                          const ctx = canvas.getContext('2d');
                                          const img = new Image();
                                          img.onload = () => {
                                            canvas.width = img.width;
                                            canvas.height = img.height;
                                            ctx.drawImage(img, 0, 0);
                                            const link = document.createElement('a');
                                            link.href = canvas.toDataURL('image/png');
                                            link.download = 'ush-2fa-qrcode.png';
                                            link.click();
                                          };
                                          const svgData = new XMLSerializer().serializeToString(svg);
                                          img.src = 'data:image/svg+xml;base64,' + btoa(svgData);
                                        }
                                      }
                                    }}
                                    className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition text-sm"
                                  >
                                    Download QR Code
                                  </button>
                                </>
                              ) : (
                                <div className="w-64 h-64 mx-auto flex items-center justify-center bg-gray-100">
                                  <p className="text-black">Loading QR Code...</p>
                                </div>
                              )}
                            </div>
                          </div>

                          {/* Secret Key Section */}
                          <div>
                            <p className="text-black font-bold mb-2">
                              Step 2: Or enter this code manually
                            </p>
                            <div className="flex items-center gap-2 mb-2">
                              <input
                                type="text"
                                value={secret}
                                readOnly
                                className="flex-1 border border-black p-2 bg-white text-black font-mono text-sm"
                              />
                              <button
                                type="button"
                                onClick={copyToClipboard}
                                className="bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition"
                              >
                                {copied ? '✓ Copied' : 'Copy'}
                              </button>
                            </div>
                            <p className="text-sm text-black">
                              Save this secret in a safe place. You'll need it if you lose access to your authenticator app.
                            </p>
                          </div>

                          {/* Recovery Codes Section */}
                          {recoveryCodes.length > 0 && (
                            <div className="p-4 border-2 border-orange-600 bg-orange-50">
                              <p className="text-black font-bold mb-3 flex items-center gap-2">
                                <span className="text-2xl">⚠️</span>
                                Step 3: Save Your Recovery Codes
                              </p>
                              <p className="text-sm text-black mb-3">
                                Save these codes in a safe place. Each code can only be used once. Use these if you lose access to your authenticator app.
                              </p>
                              <div className="grid grid-cols-2 gap-2 mb-3 p-3 bg-white border border-black">
                                {recoveryCodes.map((code, index) => (
                                  <div key={index} className="font-mono text-sm text-black">
                                    {index + 1}. {code}
                                  </div>
                                ))}
                              </div>
                              <div className="flex gap-2">
                                <button
                                  type="button"
                                  onClick={copyRecoveryCodesToClipboard}
                                  className="flex-1 px-4 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition text-sm"
                                >
                                  {copiedCodes ? '✓ Copied' : 'Copy All Codes'}
                                </button>
                                <button
                                  type="button"
                                  onClick={downloadRecoveryCodes}
                                  className="flex-1 px-4 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition text-sm"
                                >
                                  Download as File
                                </button>
                              </div>
                            </div>
                          )}

                          {/* Action Buttons */}
                          <div className="space-y-2">
                            <button
                              type="button"
                              onClick={() => navigate('/2fa-verify', { state: { isSetup: true } })}
                              className="w-full px-4 py-2 border border-black bg-black text-white font-bold hover:bg-white hover:text-black transition"
                            >
                              Verify 2FA Code
                            </button>
                            <button
                              type="button"
                              onClick={() => setShow2FASetup(false)}
                              className="w-full px-4 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
                            >
                              Cancel
                            </button>
                          </div>
                        </>
                      )}
                    </div>
                  )}
                </>
              )}
            </div>

            {/* Account Info Section */}
            <div className="border border-black p-6">
              <h2 className="text-xl font-bold text-black mb-4">Account Information</h2>

              <div className="space-y-3 text-black">
                <div className="flex justify-between">
                  <span className="font-bold">Created:</span>
                  <span>{new Date(user.created_at).toLocaleDateString()}</span>
                </div>
                <div className="flex justify-between">
                  <span className="font-bold">Last Updated:</span>
                  <span>{new Date(user.updated_at).toLocaleDateString()}</span>
                </div>
              </div>
            </div>
          </form >
        </div >
      </div >

      {/* Password Confirmation Modal */}
      {
        showPasswordConfirm && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white p-6 border-2 border-black max-w-md w-full mx-4 shadow-lg">
              <h3 className="text-xl font-bold text-black mb-4">Confirm Changes</h3>
              <p className="text-black mb-4">
                Please enter your current password to confirm these changes.
              </p>
              <input
                type="password"
                value={passwordConfirmValue}
                onChange={(e) => setPasswordConfirmValue(e.target.value)}
                placeholder="Current Password"
                className="w-full border border-black p-2 mb-4"
                autoFocus
              />
              <div className="flex justify-end gap-2">
                <button
                  onClick={() => {
                    setShowPasswordConfirm(false);
                    setPasswordConfirmValue('');
                    setPendingUpdateData(null);
                    setLoading(false);
                  }}
                  className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-gray-100 transition"
                >
                  Cancel
                </button>
                <button
                  onClick={handleConfirmPasswordUpdate}
                  disabled={!passwordConfirmValue || loading}
                  className="px-4 py-2 border border-black bg-black text-white font-bold hover:bg-white hover:text-black transition disabled:opacity-50"
                >
                  {loading ? 'Confirming...' : 'Confirm'}
                </button>
              </div>
            </div>
          </div>
        )
      }

      {/* 2FA Confirmation Modal */}
      {
        show2FAConfirm && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white p-6 border-2 border-black max-w-md w-full mx-4 shadow-lg">
              <h3 className="text-xl font-bold text-black mb-4">2FA Verification</h3>
              <p className="text-black mb-4">
                Please enter the code from your authenticator app to confirm these changes.
              </p>
              {error && (
                <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-2 rounded relative mb-4" role="alert">
                  <span className="block sm:inline">{error}</span>
                </div>
              )}
              <input
                type="text"
                value={twoFACode}
                onChange={(e) => setTwoFACode(e.target.value)}
                placeholder="Enter 6-digit code"
                className="w-full border border-black p-2 mb-4"
                autoFocus
                maxLength={6}
              />
              <div className="flex justify-end gap-2">
                <button
                  onClick={() => {
                    setShow2FAConfirm(false);
                    setTwoFACode('');
                    setPendingUpdateData(null);
                    setLoading(false);
                    setError('');
                  }}
                  className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-gray-100 transition"
                >
                  Cancel
                </button>
                <button
                  onClick={handleConfirmUpdateWith2FA}
                  disabled={!twoFACode || loading}
                  className="px-4 py-2 border border-black bg-black text-white font-bold hover:bg-white hover:text-black transition disabled:opacity-50"
                >
                  {loading ? 'Verifying...' : 'Confirm'}
                </button>
              </div>
            </div>
          </div>
        )
      }

      {/* Unlink Wallet Confirmation Modal */}
      {
        showUnlinkWalletConfirm && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white p-6 border-2 border-black max-w-md w-full mx-4 shadow-lg">
              <h3 className="text-xl font-bold text-black mb-4">Unlink Wallet?</h3>
              <p className="text-black mb-6">
                Are you sure you want to unlink your Web3 wallet? You won't be able to use wallet features until you link it again.
              </p>
              <div className="flex justify-end gap-2">
                <button
                  onClick={() => setShowUnlinkWalletConfirm(false)}
                  className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-gray-100 transition"
                >
                  Cancel
                </button>
                <button
                  onClick={confirmUnlinkWeb3Wallet}
                  className="px-4 py-2 border border-black bg-red-600 text-white font-bold hover:bg-red-700 transition"
                >
                  Unlink
                </button>
              </div>
            </div>
          </div>
        )
      }

      {/* Disable 2FA Confirmation Modal */}
      {
        showDisable2FAConfirm && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white p-6 border-2 border-black max-w-md w-full mx-4 shadow-lg">
              <h3 className="text-xl font-bold text-black mb-4">Disable 2FA?</h3>
              <p className="text-black mb-6">
                Are you sure you want to disable Two-Factor Authentication? This will significantly reduce your account security.
              </p>
              <div className="flex justify-end gap-2">
                <button
                  onClick={() => setShowDisable2FAConfirm(false)}
                  className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-gray-100 transition"
                >
                  Cancel
                </button>
                <button
                  onClick={confirmDisable2FA}
                  className="px-4 py-2 border border-black bg-red-600 text-white font-bold hover:bg-red-700 transition"
                >
                  Disable 2FA
                </button>
              </div>
            </div>
          </div>
        )
      }
    </div >
  );
};

export default ProfilePage;
