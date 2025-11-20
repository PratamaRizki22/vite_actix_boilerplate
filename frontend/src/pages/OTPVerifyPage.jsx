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
  const [codeExpiry, setCodeExpiry] = useState(180) // 3 menit = 180 detik
  const [email, setEmail] = useState('')
  const [credentials, setCredentials] = useState(null) // { username, password } untuk auto-login setelah register

  const navigate = useNavigate()
  const location = useLocation()
  const { login } = useAuth()

  useEffect(() => {
    // Get email and credentials from location state
    const emailFromState = location.state?.email
    const credentialsFromState = location.state?.credentials
    
    console.log('OTPVerifyPage mount - email:', emailFromState, 'credentials:', credentialsFromState)
    
    if (!emailFromState) {
      console.log('No email found, redirecting to login')
      navigate('/login')
    } else {
      setEmail(emailFromState)
      if (credentialsFromState) {
        console.log('Setting credentials for auto-login:', credentialsFromState)
        setCredentials(credentialsFromState) // credentials dari register atau login
      }
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

  // Code expiry timer (3 menit)
  useEffect(() => {
    if (codeExpiry > 0) {
      const timer = setTimeout(() => {
        setCodeExpiry(codeExpiry - 1)
      }, 1000)
      return () => clearTimeout(timer)
    } else if (codeExpiry === 0) {
      setError('Verification code has expired. Please request a new one.')
      setCode('')
    }
  }, [codeExpiry])

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

    try {
      console.log('Submitting OTP verification:', { email, code, codeLength: code.length })
      const response = await axios.post(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/verify`,
        {
          email,
          code,
        }
      )
      console.log('OTP verification response:', response.data)

      if (response.data.verified) {
        setSuccess('Email verified successfully!')
        
        // After email is verified, auto-login with credentials
        if (credentials) {
          console.log('Email verified, auto-login dengan credentials:', credentials.username)
          setTimeout(async () => {
            try {
              const loginResult = await login(credentials.username, credentials.password)
              console.log('Auto-login berhasil:', loginResult)
              navigate('/')
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
    } catch (err) {
      console.error('OTP verification error:', err.response?.data)
      setError(err.response?.data?.error || 'Verification failed. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  const handleResendCode = async () => {
    setError('')
    setSuccess('')
    setResendLoading(true)

    try {
      await axios.post(
        `${import.meta.env.VITE_API_BASE_URL}/api/auth/email/send-verification`,
        {
          email,
        }
      )

      setSuccess('Verification code sent to your email!')
      setResendCooldown(60) // 60 detik cooldown
      setCodeExpiry(180) // Reset 3 menit countdown
      setCode('')
    } catch (err) {
      console.error('Resend code error:', err.response?.data)
      setError(err.response?.data?.error || 'Failed to resend code. Please try again.')
    } finally {
      setResendLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-white flex items-center justify-center p-4">
      <div className="w-full max-w-md border border-black p-8">
        <h1 className="text-3xl font-bold text-black mb-2 text-center">Verify Email</h1>
        <p className="text-center text-black mb-8">
          Enter the 6-digit code sent to <br />
          <span className="font-bold">{email}</span>
        </p>

        <div className="text-center mb-6">
          <p className="text-sm text-black font-bold">
            Code expires in:{' '}
            <span className={codeExpiry <= 30 ? 'text-red-600' : 'text-black'}>
              {Math.floor(codeExpiry / 60)}:{(codeExpiry % 60).toString().padStart(2, '0')}
            </span>
          </p>
        </div>

        {error && (
          <div className="border border-black bg-white p-4 mb-6 text-black">
            {error}
          </div>
        )}

        {success && (
          <div className="border border-green-500 bg-green-50 p-4 mb-6 text-green-700">
            {success}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label className="block text-black font-bold mb-2">Verification Code</label>
            <input
              type="text"
              inputMode="numeric"
              value={code}
              onChange={(e) => setCode(e.target.value.replace(/\D/g, '').slice(0, 6))}
              maxLength="6"
              placeholder="000000"
              className="w-full border border-black p-3 bg-white text-black text-center text-2xl font-mono tracking-widest"
              disabled={loading || codeExpiry === 0}
            />
            <p className="text-sm text-black mt-2">{code.length}/6 digits</p>
          </div>

          <button
            type="submit"
            disabled={loading || code.length !== 6 || codeExpiry === 0}
            className="w-full bg-black border border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition disabled:opacity-50"
          >
            {loading ? 'Verifying...' : 'Verify Code'}
          </button>
        </form>

        <div className="mt-6 text-center">
          <p className="text-black mb-4">Didn't receive the code?</p>
          <button
            onClick={handleResendCode}
            disabled={resendLoading || resendCooldown > 0}
            className="text-black font-bold border-b border-black hover:bg-black hover:text-white px-2 py-1 transition disabled:opacity-50"
          >
            {resendCooldown > 0
              ? `Resend in ${resendCooldown}s`
              : resendLoading
              ? 'Sending...'
              : 'Resend Code'}
          </button>
        </div>

        <div className="mt-6 text-center">
          <button
            onClick={() => navigate('/login')}
            className="text-black font-bold border-b border-black hover:bg-black hover:text-white px-2 py-1"
          >
            Back to Login
          </button>
        </div>
      </div>
    </div>
  )
}

export default OTPVerifyPage
