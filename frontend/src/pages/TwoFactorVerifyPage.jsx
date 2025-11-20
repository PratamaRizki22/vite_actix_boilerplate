import React, { useState } from 'react'
import { useNavigate, useLocation } from 'react-router-dom'
import { useAuth } from '../context/AuthContext'
import twoFactorService from '../services/twoFactorService'

const TwoFactorVerifyPage = () => {
  const [code, setCode] = useState('')
  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)
  const navigate = useNavigate()
  const location = useLocation()
  const { setUser } = useAuth()

  // Determine if this is for setup (from profile) or login
  const isSetup = location.pathname === '/2fa-verify' && location.state?.isSetup

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
      const result = await twoFactorService.verify2FA(code)
      if (result.success) {
        // If this is setup from profile, refresh user data and go back to profile
        if (isSetup) {
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
          navigate('/profile')
        } else {
          // Regular login flow
          navigate('/')
        }
      } else {
        setError(result.message || '2FA verification failed')
      }
    } catch (err) {
      setError(err.response?.data?.error || 'Invalid code. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-white flex items-center justify-center p-4">
      <div className="w-full max-w-md border border-black p-8">
        <h1 className="text-3xl font-bold text-black mb-2 text-center">
          Verify Authentication
        </h1>
        <p className="text-center text-black mb-8">
          Enter the 6-digit code from your authenticator app
        </p>

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
              Authentication Code
            </label>
            <input
              type="text"
              inputMode="numeric"
              value={code}
              onChange={(e) => setCode(e.target.value.replace(/\D/g, '').slice(0, 6))}
              maxLength="6"
              placeholder="000000"
              className="w-full border border-black p-3 bg-white text-black text-center text-2xl font-mono tracking-widest"
              disabled={loading}
              autoFocus
            />
            <p className="text-sm text-black mt-2">
              {code.length}/6 digits
            </p>
          </div>

          <button
            type="submit"
            disabled={loading || code.length !== 6}
            className="w-full bg-black border border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition disabled:opacity-50"
          >
            {loading ? 'Verifying...' : 'Verify'}
          </button>
        </form>

        <div className="mt-6 border-t border-black pt-4">
          <p className="text-xs text-black mb-3 font-bold">Troubleshooting:</p>
          <ul className="text-xs text-black space-y-2 mb-4">
            <li>• <strong>Invalid code?</strong> Check if your device clock is correct</li>
            <li>• <strong>App not working?</strong> Reinstall the authenticator app</li>
            <li>• <strong>Codes not matching?</strong> Try the next code after waiting 5 seconds</li>
          </ul>
        </div>

        <div className="mt-6 text-center">
          <button
            onClick={() => navigate(isSetup ? '/profile' : '/login')}
            className="text-black font-bold border-b border-black hover:bg-black hover:text-white px-2 py-1"
          >
            {isSetup ? 'Back to Profile' : 'Back to Login'}
          </button>
        </div>
      </div>
    </div>
  )
}

export default TwoFactorVerifyPage
