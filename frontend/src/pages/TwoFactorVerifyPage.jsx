import React, { useState, useEffect } from 'react'
import { useNavigate, useLocation } from 'react-router-dom'
import { useAuth } from '../context/AuthContext'
import twoFactorService from '../services/twoFactorService'
import axios from 'axios'
import { QRCodeSVG } from 'qrcode.react'

const TwoFactorVerifyPage = () => {
  const [code, setCode] = useState('')
  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)
  const [setupRequired, setSetupRequired] = useState(false)
  const [qrCode, setQrCode] = useState('')
  const [secret, setSecret] = useState('')
  const [copied, setCopied] = useState(false)
  const [checkingSetup, setCheckingSetup] = useState(true)
  const navigate = useNavigate()
  const location = useLocation()
  const { setUser, login } = useAuth()

  // Get state from location
  const { isSetup, isRegistration, credentials, email, temp_token, isLogin, user } = location.state || {}

  // Check if user needs to setup 2FA first (ONLY for registration or profile setup, NOT login)
  useEffect(() => {
    const checkTotpSetup = async () => {
      // SECURITY: Only request QR code for registration or profile setup, NEVER for login
      if ((isRegistration || isSetup) && temp_token) {
        // Check if user has TOTP enabled
        const hasTotp = user?.totp_enabled || false
        
        if (!hasTotp) {
          // User doesn't have 2FA setup, request QR code
          console.log('User needs to setup 2FA - requesting QR code')
          try {
            // Request setup by sending empty code to get QR
            const response = await axios.post(
              `${import.meta.env.VITE_API_BASE_URL}/api/auth/verify-mfa`,
              {
                temp_token,
                method: 'totp',
                code: '' // Empty code to trigger setup
              }
            )
            
            if (response.data.setup_required) {
              setSetupRequired(true)
              setQrCode(response.data.qr_code_url)
              setSecret(response.data.secret)
            }
          } catch (err) {
            console.error('Failed to check 2FA setup:', err)
            setError(err.response?.data?.error || 'Failed to setup 2FA')
          }
        }
      }
      // For login, just show code input - no QR code request
      setCheckingSetup(false)
    }
    
    checkTotpSetup()
  }, [isRegistration, isSetup, temp_token, user])

  const copyToClipboard = () => {
    navigator.clipboard.writeText(secret)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const handleSubmit = async (e) => {
    e.preventDefault()
    setError('')
    setLoading(true)

    if (code.length !== 6 || !/^\d+$/.test(code)) {
      setError('Please enter a valid 6-digit code')
      setLoading(false)
      return
    }

    try {
      // Check if this is login flow with temp_token
      if (isLogin && temp_token) {
        // Login flow - call verify-mfa endpoint
        const response = await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/verify-mfa`,
          {
            temp_token,
            method: 'totp',
            code
          }
        )

        if (response.data.token) {
          // Store token and user data
          localStorage.setItem('token', response.data.token)
          if (response.data.refresh_token) {
            localStorage.setItem('refresh_token', response.data.refresh_token)
          }
          if (response.data.user) {
            localStorage.setItem('user', JSON.stringify(response.data.user))
            setUser(response.data.user)
          }

          // Dispatch auth event and navigate to dashboard
          window.dispatchEvent(new Event('auth-update'))
          navigate('/dashboard')
        } else {
          setError('Login verification failed')
        }
      } else {
        // Setup or registration flow - call setup-2fa/verify-2fa endpoint
        const result = await twoFactorService.verify2FA(code)
        if (result.success) {
          // If this is registration flow, auto-login then go to dashboard
          if (isRegistration && credentials) {
            console.log('2FA verified in registration, auto-login with:', credentials.username)
            setTimeout(async () => {
              try {
                await login(credentials.username, credentials.password)
                console.log('Auto-login berhasil')
                
                // Dispatch event to notify AuthContext
                window.dispatchEvent(new Event('auth-update'))
                
                // Navigate to dashboard
                setTimeout(() => {
                  navigate('/dashboard')
                }, 100)
              } catch (loginErr) {
                console.error('Auto-login failed:', loginErr)
                setError('2FA verified but auto-login failed. Please login manually.')
                setTimeout(() => navigate('/login'), 2000)
              }
            }, 500)
          }
          // If this is setup from profile, refresh user data and go back to profile
          else if (isSetup) {
            const token = localStorage.getItem('token')
            const userResponse = await fetch(`${import.meta.env.VITE_API_BASE_URL}/api/auth/me`, {
              headers: {
                Authorization: `Bearer ${token}`
              }
            })
            if (userResponse.ok) {
              const userData = await userResponse.json()
              localStorage.setItem('user', JSON.stringify(userData))
              setUser(userData)
            }
            // Dispatch auth event to notify ProfilePage
            window.dispatchEvent(new Event('auth-update'))
            navigate('/profile')
          } else {
            // Regular login flow - dispatch auth event and navigate to dashboard
            window.dispatchEvent(new Event('auth-update'))
            navigate('/dashboard')
          }
        } else {
          setError(result.message || '2FA verification failed')
        }
      }
    } catch (err) {
      setError(err.response?.data?.error || 'Invalid code. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  if (checkingSetup) {
    return (
      <div className="min-h-screen bg-white flex items-center justify-center p-4">
        <div className="w-full max-w-md border border-black p-8">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-black mx-auto mb-4"></div>
            <p className="text-black font-bold">Checking 2FA setup...</p>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-white flex items-center justify-center p-4">
      <div className="w-full max-w-md border border-black p-8">
        <h1 className="text-3xl font-bold text-black mb-2 text-center">
          {setupRequired ? 'Setup Two-Factor Authentication' : 'Verify Authentication'}
        </h1>
        <p className="text-center text-black mb-8">
          {setupRequired 
            ? 'Scan the QR code with your authenticator app, then enter the code'
            : 'Enter the 6-digit code from your authenticator app'}
        </p>

        {setupRequired && qrCode && (
          <>
            {/* QR Code Section */}
            <div className="mb-6 p-6 border border-black text-center">
              <p className="text-black font-bold mb-4">
                Step 1: Scan QR Code
              </p>
              <QRCodeSVG
                value={qrCode}
                size={200}
                level="H"
                includeMargin={true}
                className="mx-auto border border-black"
              />
            </div>

            {/* Secret Key Section */}
            <div className="mb-6 p-4 border border-black">
              <p className="text-black font-bold mb-2 text-sm">
                Or enter this code manually:
              </p>
              <div className="flex items-center gap-2">
                <input
                  type="text"
                  value={secret}
                  readOnly
                  className="flex-1 border border-black p-2 bg-white text-black font-mono text-xs"
                />
                <button
                  onClick={copyToClipboard}
                  className="bg-white border border-black text-black font-bold py-2 px-3 text-sm hover:bg-black hover:text-white transition"
                >
                  {copied ? '✓' : 'Copy'}
                </button>
              </div>
            </div>

            <div className="mb-6 p-4 border border-black bg-gray-50">
              <p className="text-black font-bold mb-2 text-sm">Supported Apps:</p>
              <ul className="text-black text-xs space-y-1">
                <li>• Google Authenticator</li>
                <li>• Microsoft Authenticator</li>
                <li>• Authy</li>
              </ul>
            </div>
          </>
        )}

        {error && (
          <div className="border border-red-600 bg-red-50 p-4 mb-6 text-red-700">
            <p className="font-bold mb-2">❌ {error}</p>
            <div className="text-sm space-y-1">
              <p>• Make sure your device time is synchronized</p>
              <p>• The code changes every 30 seconds</p>
              <p>• Try the current code shown in your app</p>
            </div>
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label className="block text-black font-bold mb-2">
              {setupRequired ? 'Step 2: Enter Code from App' : 'Authentication Code or Recovery Code'}
            </label>
            <input
              type="text"
              value={code}
              onChange={(e) => {
                const value = e.target.value.toUpperCase();
                // Allow either 6 digits OR recovery code format (XXXX-XXXX)
                if (/^[0-9]{0,6}$/.test(value) || /^[A-Z0-9-]{0,9}$/.test(value)) {
                  setCode(value);
                }
              }}
              maxLength="9"
              placeholder="000000 or XXXX-XXXX"
              className="w-full border border-black p-3 bg-white text-black text-center text-2xl font-mono tracking-widest"
              disabled={loading}
              autoFocus
            />
            <p className="text-sm text-black mt-2">
              {code.includes('-') ? 'Recovery code format' : `${code.length}/6 digits`}
            </p>
          </div>

          <button
            type="submit"
            disabled={loading || (code.length !== 6 && code.length !== 9)}
            className="w-full bg-black border border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition disabled:opacity-50"
          >
            {loading ? 'Verifying...' : 'Verify'}
          </button>
        </form>

        <div className="mt-6 border-t border-black pt-4">
          <p className="text-xs text-black mb-3 font-bold">Troubleshooting:</p>
          <ul className="text-xs text-black space-y-2 mb-4">
            <li>• <strong>Invalid code?</strong> Check if your device clock is correct</li>
            <li>• <strong>Lost your authenticator app?</strong> Use a recovery code (format: XXXX-XXXX)</li>
            <li>• <strong>Codes not matching?</strong> Try the next code after waiting 5 seconds</li>
          </ul>
          <div className="p-3 bg-orange-50 border border-orange-600 text-xs text-black">
            <strong>Recovery Codes:</strong> If you lost access to your authenticator app, you can use one of the recovery codes you saved during setup. Enter it in the code field above.
          </div>
        </div>

        <div className="mt-6 text-center">
          <button
            onClick={() => {
              if (isSetup) {
                // If setting up 2FA from profile, go back to profile
                navigate('/profile');
              } else {
                // If during login/register, cancel and return to login/register page
                navigate(isRegistration ? '/register' : '/login');
              }
            }}
            className="text-black font-bold border-b border-black hover:bg-black hover:text-white px-2 py-1"
          >
            {isSetup ? 'Back' : 'Cancel'}
          </button>
        </div>
      </div>
    </div>
  )
}

export default TwoFactorVerifyPage
