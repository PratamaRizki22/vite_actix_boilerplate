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
  const [currentPassword, setCurrentPassword] = useState('');
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
  const [copied, setCopied] = useState(false);
  const qrCodeRef = useRef(null);

  // Update form when user data loads
  useEffect(() => {
    if (user) {
      console.log('User data loaded:', user);
      setUsername(user.username || '');
      setEmail(user.email || '');
      // Set 2FA status from user data
      setIs2FAEnabled(user.two_factor_enabled || false);
    }
  }, [user]);

  if (!user) {
    return (
      <div className="min-h-screen bg-white flex items-center justify-center p-4">
        <div className="text-center">
          <p className="text-black font-bold mb-4">Please login to view your profile</p>
          <button
            onClick={() => navigate('/login')}
            className="px-6 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
          >
            Go to Login
          </button>
        </div>
      </div>
    );
  }

  // 2FA Setup Functions
  const load2FAQrCode = async () => {
    try {
      setTwoFALoading(true);
      const data = await twoFactorService.setup2FA();
      setQrCode(data.qr_code_url);
      setSecret(data.secret);
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to setup 2FA');
    } finally {
      setTwoFALoading(false);
    }
  };

  const handle2FASetup = async () => {
    setError('');
    setSuccess('');
    setShow2FASetup(true);
    await load2FAQrCode();
  };

  const handleDisable2FA = async () => {
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
          const userData = await userResponse.json();
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
        `${import.meta.env.VITE_API_BASE_URL}/api/users/${user.id}`,
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
    } finally {
      setLinkWalletLoading(false);
    }
  };

  const handleUnlinkWeb3Wallet = async () => {
    if (!window.confirm('Are you sure you want to unlink your wallet?')) {
      return;
    }

    setError('');
    setSuccess('');
    setLinkWalletLoading(true);

    try {
      const token = localStorage.getItem('token');

      // Remove wallet address
      const response = await axios.put(
        `${import.meta.env.VITE_API_BASE_URL}/api/users/${user.id}`,
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

  const handleUpdateProfile = async (e) => {
    e.preventDefault();
    setError('');
    setSuccess('');
    setLoading(true);

    try {
      const token = localStorage.getItem('token');
      const updateData = {};

      // Only add fields that have changed
      if (username && username !== user.username) {
        updateData.username = username;
      }
      if (email && email !== user.email) {
        updateData.email = email;
      }

      if (Object.keys(updateData).length === 0 && !newPassword) {
        setError('No changes made');
        setLoading(false);
        return;
      }

      // If updating password, require current password
      if (newPassword) {
        if (!currentPassword) {
          setError('Current password is required to change password');
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
        updateData.password = newPassword;
      }

      const response = await axios.put(
        `${import.meta.env.VITE_API_BASE_URL}/api/users/${user.id}`,
        updateData,
        {
          headers: {
            Authorization: `Bearer ${token}`,
            'Content-Type': 'application/json',
          },
        }
      );

      setSuccess('Profile updated successfully!');
      
      // Update localStorage with new user data
      localStorage.setItem('user', JSON.stringify(response.data));
      
      // Clear password fields
      setCurrentPassword('');
      setNewPassword('');
      setConfirmPassword('');

      // Refresh page after 1 second to show updated info
      setTimeout(() => {
        window.location.reload();
      }, 1000);
    } catch (err) {
      console.error('Profile update error:', err);
      const errorMsg = err.response?.data?.error || err.message || 'Failed to update profile';
      setError(errorMsg);
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

  return (
    <div className="min-h-screen bg-white pt-24 pb-8">
      <div className="container mx-auto px-4">
        <div className="max-w-2xl mx-auto">
          <h1 className="text-3xl font-bold text-black mb-8">Profile Settings</h1>

          {error && (
            <div className="border-2 border-red-600 bg-red-50 text-red-700 p-4 mb-6 font-bold">
              {error}
            </div>
          )}

          {success && (
            <div className="border-2 border-green-600 bg-green-50 text-green-700 p-4 mb-6 font-bold">
              {success}
            </div>
          )}

          <form onSubmit={handleUpdateProfile} className="space-y-6">
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
            </div>

            {/* Password Change Section */}
            <div className="border border-black p-6">
              <h2 className="text-xl font-bold text-black mb-4">Change Password</h2>
              <p className="text-black text-sm mb-4">Leave blank if you don't want to change your password</p>
              
              <div className="space-y-4">
                <div>
                  <label className="block text-black font-bold mb-2">Current Password</label>
                  <div className="relative">
                    <input
                      type={showPassword ? 'text' : 'password'}
                      value={currentPassword}
                      onChange={(e) => setCurrentPassword(e.target.value)}
                      className="w-full border border-black p-2 pr-12 bg-white text-black"
                      placeholder="Enter current password"
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
                  <input
                    type={showPassword ? 'text' : 'password'}
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    className="w-full border border-black p-2 bg-white text-black"
                    placeholder="Confirm new password"
                  />
                </div>
              </div>
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

            {/* Submit Button */}
            <div className="flex gap-4 justify-end">
              <button
                type="button"
                onClick={() => navigate('/')}
                className="px-6 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={loading}
                className="px-6 py-2 border border-black bg-black text-white font-bold hover:bg-white hover:text-black transition disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {loading ? 'Updating...' : 'Update Profile'}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
};

export default ProfilePage;
