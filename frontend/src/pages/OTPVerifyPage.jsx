import React, { useState, useEffect } from 'react'
import { useNavigate, useLocation } from 'react-router-dom'
import { useAuth } from '../context/AuthContext'
import axios from 'axios'

const OTPVerifyPage = () => {
  const [code, setCode] = useState('')
  const [error, setError] = useState('')
  const [success, setSuccess] = useState('')
  const [loading, setLoading] = useState(false)
  const [resendLoading, setResendLoading] = useState(false)
  const [resendCooldown, setResendCooldown] = useState(0)
  const [codeExpiry, setCodeExpiry] = useState(null) // Start with null until we get backend value
  const [isExpirySynced, setIsExpirySynced] = useState(false) // Flag untuk indicate backend sync selesai
  const [hasCodeBeenSent, setHasCodeBeenSent] = useState(false) // Track if code has been sent at least once
  const [lastCodeSentTime, setLastCodeSentTime] = useState(null) // Track when code was last sent
  const [email, setEmail] = useState('')
  const [pageLoading, setPageLoading] = useState(true) // Loading state untuk initial page setup
  const [credentials, setCredentials] = useState(null) // { username, password } untuk auto-login setelah register
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  // For login flow
  const [tempToken, setTempToken] = useState('')
  const [mfaMethods, setMfaMethods] = useState([])
  const [user, setUser] = useState(null)
  const [canSkipMfa, setCanSkipMfa] = useState(false)

  const navigate = useNavigate()
  const location = useLocation()
  const { login } = useAuth()

  // Get state to determine if from login or registration
  const isRegistration = location.state?.isRegistration || false

  useEffect(() => {
    // Get email and credentials from location state
    let emailFromState = location.state?.email
    let credentialsFromState = location.state?.credentials
    let isRegistrationFromStorage = false
    let codeSentFromState = location.state?.codeSent // Flag if code was already sent
    let autoSentFromState = location.state?.autoSent // NEW: Flag if code was auto-sent from AuthMethodSelectPage

    // For login flow
    let tempTokenFromState = location.state?.temp_token
    let mfaMethodsFromState = location.state?.mfa_methods
    let userFromState = location.state?.user
    let canSkipMfaFromState = location.state?.can_skip_mfa

    // If no state from location, try to get from sessionStorage (for page refresh)
    let storedData = null
    if (!emailFromState) {
      const stored = sessionStorage.getItem('otp_verify_state')
      if (stored) {
        try {
          storedData = JSON.parse(stored)
          emailFromState = storedData.email
          // Override location state with stored data
          if (storedData.isRegistration !== undefined) isRegistrationFromStorage = storedData.isRegistration
          if (storedData.tempToken) tempTokenFromState = storedData.tempToken
          if (storedData.mfaMethods) mfaMethodsFromState = storedData.mfaMethods
          if (storedData.user) userFromState = storedData.user
          if (storedData.canSkipMfa !== undefined) canSkipMfaFromState = storedData.canSkipMfa
          if (storedData.codeSent) codeSentFromState = storedData.codeSent
          if (storedData.autoSent) autoSentFromState = storedData.autoSent

          // Restore expires_at timestamp if available
          if (storedData.expiresAt) {
            const expiresAtTime = new Date(storedData.expiresAt).getTime()
            const now = Date.now()
            const remainingMs = expiresAtTime - now
            const remainingSeconds = Math.max(0, Math.floor(remainingMs / 1000))

            if (remainingSeconds > 0) {
              setCodeExpiry(remainingSeconds)
              setIsExpirySynced(true)
              console.log(`Restored code expiry from timestamp: ${remainingSeconds} seconds remaining`)
            }
          }
        } catch (e) {
          console.error('Failed to parse stored OTP state:', e)
        }
      }
    }

    // Normalize email
    if (emailFromState) {
      emailFromState = emailFromState.toLowerCase().trim()
    }

    console.log('OTPVerifyPage mount - email:', emailFromState, 'autoSent:', autoSentFromState)

    if (!emailFromState) {
      console.log('No email found, redirecting to login')
      navigate('/login')
      return
    }

    setEmail(emailFromState)
    if (credentialsFromState) {
      console.log('Setting credentials for auto-login:', credentialsFromState)
      setCredentials(credentialsFromState) // credentials dari register atau login
      setUsername(credentialsFromState.username)
      setPassword(credentialsFromState.password)
    }

    // Store login data
    if (tempTokenFromState) {
      setTempToken(tempTokenFromState)
      setMfaMethods(mfaMethodsFromState || [])
      setUser(userFromState)
      setCanSkipMfa(canSkipMfaFromState || false)
    }

    // If code was already sent before navigation (e.g., from LoginPage for unverified email)
    if (codeSentFromState || autoSentFromState) {
      console.log('Code was sent before navigation, marking as sent')
      setHasCodeBeenSent(true)
      setLastCodeSentTime(Date.now()) // Track when code was sent

      // If auto-sent, set default expiry time immediately to prevent "expired" message
      if (autoSentFromState) {
        // Determine if this is login flow (has tempToken) or registration flow
        const isLoginFlow = tempTokenFromState ? true : false
        const defaultExpiry = isLoginFlow ? 120 : 180 // 2 min for MFA, 3 min for registration
        console.log(`Auto-sent detected, setting default expiry: ${defaultExpiry}s (${isLoginFlow ? 'MFA' : 'Registration'} flow)`)
        setCodeExpiry(defaultExpiry)
        setIsExpirySynced(true)

        // Delay the expiry check to allow backend to process
        setTimeout(() => {
          checkCodeExpiry(emailFromState)
        }, 2000) // Wait 2 seconds before checking actual expiry
      }
    }

    // Store state in sessionStorage for refresh persistence
    const stateToStore = {
      email: emailFromState,
      isRegistration: location.state?.isRegistration || storedData?.isRegistration || false,
      codeSent: codeSentFromState || autoSentFromState || false,
      autoSent: autoSentFromState || false,
      // Store login flow data if available (for persistence across refresh)
      tempToken: tempTokenFromState,
      mfaMethods: mfaMethodsFromState,
      user: userFromState,
      canSkipMfa: canSkipMfaFromState,
      expiresAt: null, // Will be updated when we get the expiry time from backend
    }
    sessionStorage.setItem('otp_verify_state', JSON.stringify(stateToStore))

    // Check remaining time for existing verification code (only if not auto-sent)
    if (!autoSentFromState) {
      checkCodeExpiry(emailFromState)
    }
  }, [location, navigate])

  // Cooldown timer untuk resend
  useEffect(() => {
    if (resendCooldown > 0) {
      const timer = setTimeout(() => {
        setResendCooldown(resendCooldown - 1)
      }, 1000)
      return () => clearTimeout(timer)
    }
  }, [resendCooldown])

  // Code expiry timer (3 menit) with backend sync
  // Only countdown ketika expiry sudah di-sync dari backend
  useEffect(() => {
    if (codeExpiry === null || codeExpiry <= 0 || !isExpirySynced) return

    const timer = setTimeout(() => {
      setCodeExpiry(codeExpiry - 1)
    }, 1000)
    return () => clearTimeout(timer)
  }, [codeExpiry, isExpirySynced])

  // Expire error handling
  useEffect(() => {
    // Only show expired error if a code was previously sent (hasCodeBeenSent) and now expired
    // BUT don't show if code was just sent (within last 5 seconds) to avoid false positive
    if (codeExpiry === 0 && isExpirySynced && hasCodeBeenSent) {
      const now = Date.now()
      const timeSinceLastSent = lastCodeSentTime ? now - lastCodeSentTime : Infinity

      // Only show expired error if code was sent more than 5 seconds ago
      if (timeSinceLastSent > 5000) {
        setError('Verification code has expired. Please request a new one.')
        setCode('')
        // Clear stored state when code expires
        sessionStorage.removeItem('otp_verify_state')
      }
    }
  }, [codeExpiry, isExpirySynced, hasCodeBeenSent, lastCodeSentTime])

  // Auto-dismiss error messages after 5 seconds
  useEffect(() => {
    if (error) {
      const timer = setTimeout(() => {
        setError('')
      }, 5000) // 5 seconds
      return () => clearTimeout(timer)
    }
  }, [error])

  // Auto-dismiss success messages after 5 seconds
  useEffect(() => {
    if (success) {
      const timer = setTimeout(() => {
        setSuccess('')
      }, 5000) // 5 seconds
      return () => clearTimeout(timer)
    }
  }, [success])

  // Periodic sync with backend every 30 seconds to keep timer accurate
  useEffect(() => {
    if (!email || !isExpirySynced || codeExpiry === 0) return

    const syncInterval = setInterval(() => {
      checkCodeExpiry(email)
    }, 30000) // Sync every 30 seconds

    return () => clearInterval(syncInterval)
  }, [email, codeExpiry])

  const handleSubmit = async (e) => {
    e.preventDefault()
    setError('')
    setSuccess('')
    setLoading(true)

    if (code.length !== 6 || !/^\d+$/.test(code)) {
      setError('Please enter a valid 6-digit code')
      setLoading(false)
      return
    }

    if (!isExpirySynced || codeExpiry <= 0) {
      setError('Please click "Send Code" first to receive your verification code')
      setLoading(false)
      return
    }

    try {
      // Check if this is login flow with temp_token
      if (tempToken) {
        // Login flow - call verify-mfa endpoint
        console.log('Login flow: verifying MFA with email OTP')
        const response = await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/verify-mfa`,
          {
            temp_token: tempToken,
            method: 'email',
            code
          }
        )

        if (response.data.token) {
          setSuccess('Login successful!')
          // Clear stored state
          sessionStorage.removeItem('otp_verify_state')

          // Store token and user data
          localStorage.setItem('token', response.data.token)
          if (response.data.refresh_token) {
            localStorage.setItem('refresh_token', response.data.refresh_token)
          }
          if (response.data.user) {
            localStorage.setItem('user', JSON.stringify(response.data.user))
          }

          // Dispatch auth event and navigate to dashboard
          window.dispatchEvent(new Event('auth-update'))
          setTimeout(() => {
            navigate('/dashboard')
          }, 100)
        } else {
          setError('Login verification failed')
        }
      } else {
        // Registration flow - verify email
        const normalizedEmail = email.toLowerCase().trim()
        console.log('Registration flow: verifying email with OTP:', { email: normalizedEmail, code, codeLength: code.length })
        const response = await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/verify`,
          {
            email: normalizedEmail,
            code,
          }
        )
        console.log('OTP verification response:', response.data)

        if (response.data.verified) {
          setSuccess('Email verified successfully!')
          // Clear stored state
          sessionStorage.removeItem('otp_verify_state')

          // Check if should redirect to 2FA setup (mandatory 2FA flow)
          const redirectTo2FA = location.state?.redirectTo2FA

          if (redirectTo2FA && credentials) {
            console.log('Email verified, redirecting to mandatory 2FA setup')
            setTimeout(() => {
              navigate('/2fa-setup', {
                state: {
                  email,
                  username: credentials.username,
                  password: credentials.password,
                  isRegistration: true,
                  mandatory: true
                }
              })
            }, 500)
          }
          // After email is verified, auto-login with credentials (old flow)
          else if (credentials) {
            console.log('Email verified, auto-login dengan credentials:', credentials.username)
            setTimeout(async () => {
              try {
                const loginResult = await login(credentials.username, credentials.password)
                console.log('Auto-login berhasil:', loginResult)

                // Dispatch event to notify AuthContext
                window.dispatchEvent(new Event('auth-update'));

                // Navigate directly to dashboard (not home page which might redirect to login)
                setTimeout(() => {
                  navigate('/dashboard');
                }, 100);
              } catch (loginErr) {
                console.error('Auto-login failed:', loginErr)
                setError('Email verified but auto-login failed. Please login manually.')
                setTimeout(() => navigate('/login'), 2000)
              }
            }, 500)
          } else {
            console.log('Email verified but no credentials, navigasi ke login untuk manual login')
            setTimeout(() => {
              navigate('/login')
            }, 1500)
          }
        } else {
          setError('Invalid verification code. Please try again.')
        }
      }
    } catch (err) {
      console.error('OTP verification error:', err.response?.data)
      const errorMessage = err.response?.data?.message || err.response?.data?.error || 'Verification failed. Please try again.'
      setError(errorMessage)
    } finally {
      setLoading(false)
    }
  }

  const checkCodeExpiry = async (emailToCheck) => {
    try {
      let response
      if (tempToken) {
        // Login flow - check MFA code expiry
        response = await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/check-mfa-expiry`,
          { temp_token: tempToken }
        )
      } else {
        // Registration flow - check verification code expiry
        response = await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/check-expiry`,
          { email: emailToCheck }
        )
      }

      if (response.data.has_code && response.data.expires_in_seconds) {
        setCodeExpiry(response.data.expires_in_seconds)
        setHasCodeBeenSent(true) // Mark that a code exists
        console.log(`Code expires in ${response.data.expires_in_seconds} seconds`)

        // Update stored state with absolute expiry timestamp from backend
        if (response.data.expires_at) {
          const stored = sessionStorage.getItem('otp_verify_state')
          if (stored) {
            try {
              const storedData = JSON.parse(stored)
              storedData.expiresAt = response.data.expires_at
              sessionStorage.setItem('otp_verify_state', JSON.stringify(storedData))
              console.log(`Stored expires_at timestamp: ${response.data.expires_at}`)
            } catch (e) {
              console.error('Failed to update stored expiry timestamp:', e)
            }
          }
        }
        // Mark as synced with backend
        setIsExpirySynced(true)
      } else {
        console.log('No active code found')
        // Only set codeExpiry to 0 if we don't already have a valid default expiry
        // This prevents race condition where auto-sent code hasn't been saved yet
        setCodeExpiry(prev => {
          if (prev && prev > 0) {
            console.log(`Keeping existing expiry: ${prev}s (backend hasn't saved code yet)`)
            return prev // Keep existing default expiry
          }
          return 0 // No code exists yet
        })
        // Mark as synced so UI shows "Click Send Code" instead of "Loading..."
        setIsExpirySynced(true)
      }
    } catch (err) {
      console.error('Failed to check code expiry:', err)
      // Only set to 0 if we don't have a valid default expiry
      setCodeExpiry(prev => prev && prev > 0 ? prev : 0)
      // Mark as synced so UI shows "Click Send Code" instead of "Loading..."
      setIsExpirySynced(true)
    } finally {
      setPageLoading(false) // Set page loading to false after expiry check completes
    }
  }

  const sendOTPCode = async (emailToSend) => {
    try {
      console.log('Sending OTP code, tempToken:', tempToken ? 'present' : 'not present')

      let response
      if (tempToken) {
        // Login flow - use send-mfa-code endpoint
        console.log('Login flow: sending MFA code via temp_token')
        response = await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-mfa-code`,
          { temp_token: tempToken }
        )
      } else {
        // Registration flow - use send-verification endpoint
        console.log('Registration flow: sending verification code to:', emailToSend)
        response = await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-verification`,
          { email: emailToSend }
        )
      }

      console.log('OTP sent successfully')
      setHasCodeBeenSent(true) // Mark that code has been sent
      // Set resend cooldown from backend response
      const cooldownSeconds = response.data.resend_cooldown_seconds || 60
      setResendCooldown(cooldownSeconds)

      // Get expiry time after sending
      try {
        let expiryResponse
        if (tempToken) {
          // Login flow - check MFA expiry
          expiryResponse = await axios.post(
            `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/check-mfa-expiry`,
            { temp_token: tempToken }
          )
        } else {
          // Registration flow - check verification expiry
          expiryResponse = await axios.post(
            `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/check-expiry`,
            { email: emailToSend }
          )
        }

        if (expiryResponse.data.has_code && expiryResponse.data.expires_in_seconds) {
          setCodeExpiry(expiryResponse.data.expires_in_seconds);
        } else {
          // Default: MFA codes expire in 2 minutes, verification codes in 3 minutes
          setCodeExpiry(tempToken ? 120 : 180);
        }
        setIsExpirySynced(true)
      } catch (err) {
        console.error('Failed to get expiry time after sending:', err);
        // Default: MFA codes expire in 2 minutes, verification codes in 3 minutes
        setCodeExpiry(tempToken ? 120 : 180);
        setIsExpirySynced(true)
      }
    } catch (err) {
      console.error('Failed to send OTP:', err)

      // Handle rate limit errors
      if (err.response?.status === 429) {
        const retryAfter = err.response.data.retry_after || 60
        setResendCooldown(retryAfter)
        setError(`Too many requests. Please wait ${retryAfter} seconds before trying again.`)
      } else {
        setError('Failed to send verification code. Please try again.')
      }
    } finally {
      setPageLoading(false) // Set page loading to false after OTP sending completes
    }
  }

  const handleResendCode = async () => {
    setResendLoading(true)
    setError('')
    setSuccess('')

    try {
      let response
      if (tempToken) {
        // Login flow - use send-mfa-code endpoint
        console.log('Resend: Login flow, using MFA code endpoint')
        response = await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-mfa-code`,
          { temp_token: tempToken }
        )
      } else {
        // Registration flow - use send-verification endpoint
        console.log('Resend: Registration flow, using verification endpoint')
        response = await axios.post(
          `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-verification`,
          { email }
        )
      }

      setSuccess('Verification code sent to your email!')
      setHasCodeBeenSent(true) // Mark that code has been sent
      setLastCodeSentTime(Date.now()) // Track when code was sent
      // Set resend cooldown from backend response
      const cooldownSeconds = response.data.resend_cooldown_seconds || 60
      setResendCooldown(cooldownSeconds)

      // Get fresh expiry time from backend
      try {
        let expiryResponse
        if (tempToken) {
          // Login flow - check MFA expiry
          expiryResponse = await axios.post(
            `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/check-mfa-expiry`,
            { temp_token: tempToken }
          )
        } else {
          // Registration flow - check verification expiry
          expiryResponse = await axios.post(
            `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/check-expiry`,
            { email }
          )
        }

        if (expiryResponse.data.has_code && expiryResponse.data.expires_in_seconds) {
          setCodeExpiry(expiryResponse.data.expires_in_seconds);
        } else {
          // Default: MFA codes expire in 2 minutes, verification codes in 3 minutes
          setCodeExpiry(tempToken ? 120 : 180);
        }
        setIsExpirySynced(true)
      } catch (err) {
        console.error('Failed to get new expiry time:', err);
        // Default: MFA codes expire in 2 minutes, verification codes in 3 minutes
        setCodeExpiry(tempToken ? 120 : 180);
        setIsExpirySynced(true)
      }

      setCode('')
    } catch (err) {
      console.error('Resend code error:', err.response?.data)

      // Handle rate limit errors
      if (err.response?.status === 429) {
        const retryAfter = err.response.data.retry_after || 60
        setResendCooldown(retryAfter)
        setError(`Too many requests. Please wait ${retryAfter} seconds before trying again.`)
      } else {
        setError(err.response?.data?.error || 'Failed to resend code. Please try again.')
      }
    } finally {
      setResendLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-white flex items-center justify-center p-4">
      {/* Toast Notifications - Fixed at top */}
      {error && (
        <div
          className={`fixed top-4 left-1/2 transform -translate-x-1/2 z-50 max-w-md w-full mx-4 border p-4 text-black shadow-lg transition-all duration-300 ease-in-out ${error.includes('expired')
            ? 'border-red-500 bg-red-50 animate-pulse'
            : 'border-black bg-white'
            }`}
          style={{
            animation: 'slideDown 0.3s ease-out'
          }}
        >
          <div className="flex items-center justify-between">
            <span>{error}</span>
            <button
              onClick={() => setError('')}
              className="ml-4 text-black hover:text-gray-600 font-bold"
            >
              ✕
            </button>
          </div>
        </div>
      )}

      {success && (
        <div
          className="fixed top-4 left-1/2 transform -translate-x-1/2 z-50 max-w-md w-full mx-4 border border-green-500 bg-green-50 p-4 text-green-700 shadow-lg transition-all duration-300 ease-in-out"
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

      <div className="w-full max-w-md border border-black p-8">
        {pageLoading ? (
          // Loading state for initial page setup
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-black mx-auto mb-4"></div>
            <p className="text-black font-bold">Loading verification page...</p>
          </div>
        ) : (
          <>
            <h1 className="text-3xl font-bold text-black mb-2 text-center">Verify Email</h1>
            <p className="text-center text-black mb-8">
              {hasCodeBeenSent || (isExpirySynced && codeExpiry > 0)
                ? 'Enter the 6-digit verification code sent to'
                : 'Click "Send Code" below to receive your verification code at'} <br />
              <span className="font-bold">{email}</span>
            </p>

            <div className="text-center mb-6">
              <p className="text-sm text-black font-bold">
                {isExpirySynced && codeExpiry > 0 ? (
                  <>
                    Code expires in:{' '}
                    <span className={codeExpiry <= 30 ? 'text-red-600 animate-pulse' : 'text-black'}>
                      {`${Math.floor(codeExpiry / 60)}:${(codeExpiry % 60).toString().padStart(2, '0')}`}
                    </span>
                  </>
                ) : (
                  <span className="text-gray-500">
                    {!isExpirySynced ? 'Loading...' : 'Click "Send Code" to get started'}
                  </span>
                )}
              </p>
            </div>

            <form onSubmit={handleSubmit} className={`space-y-6 ${loading ? 'opacity-75 pointer-events-none' : ''}`}>
              <div>
                <label className="block text-black font-bold mb-2">Verification Code</label>
                <input
                  type="text"
                  inputMode="numeric"
                  value={code}
                  onChange={(e) => setCode(e.target.value.replace(/\D/g, '').slice(0, 6))}
                  maxLength="6"
                  placeholder="000000"
                  className="w-full border border-black p-3 bg-white text-black text-center text-2xl font-mono tracking-widest disabled:bg-gray-100 disabled:cursor-not-allowed transition"
                  disabled={loading || !isExpirySynced || codeExpiry <= 0 || pageLoading}
                />
                <p className="text-sm text-black mt-2">{code.length}/6 digits</p>
              </div>

              <button
                type="submit"
                disabled={loading || code.length !== 6 || !isExpirySynced || codeExpiry <= 0}
                className="w-full bg-black border border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
              >
                {loading && (
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                )}
                {loading ? 'Verifying...' : 'Verify Code'}
              </button>
            </form>

            <div className="mt-6 text-center">
              {hasCodeBeenSent || (isExpirySynced && codeExpiry > 0) ? (
                <p className="text-black mb-4">Didn't receive the code?</p>
              ) : null}
              <button
                onClick={handleResendCode}
                disabled={resendLoading || resendCooldown > 0}
                className="text-black font-bold border-b border-black hover:bg-black hover:text-white px-2 py-1 transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
              >
                {resendLoading && (
                  <div className="animate-spin rounded-full h-3 w-3 border-b border-black mr-1"></div>
                )}
                {resendCooldown > 0
                  ? `Resend in ${resendCooldown}s`
                  : resendLoading
                    ? 'Sending...'
                    : hasCodeBeenSent || (isExpirySynced && codeExpiry > 0)
                      ? 'Resend Code'
                      : 'Send Code'}
              </button>
            </div>

            <div className="mt-6 text-center">
              <button
                onClick={() => {
                  console.log('=== CANCEL BUTTON CLICKED ===');
                  // Clear stored state
                  sessionStorage.removeItem('otp_verify_state');
                  // Return to login/register page
                  navigate(isRegistration ? '/register' : '/login');
                }}
                disabled={pageLoading}
                className="text-black font-bold border-b border-black hover:bg-black hover:text-white px-2 py-1 transition disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Cancel
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  )
}

export default OTPVerifyPage
