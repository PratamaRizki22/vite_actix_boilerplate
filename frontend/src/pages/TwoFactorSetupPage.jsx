import React, { useState, useEffect } from 'react'
import { useNavigate, useLocation } from 'react-router-dom'
import { QRCodeSVG } from 'qrcode.react'
import twoFactorService from '../services/twoFactorService'

const TwoFactorSetupPage = () => {
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [qrCode, setQrCode] = useState('')
  const [secret, setSecret] = useState('')
  const [copied, setCopied] = useState(false)
  const navigate = useNavigate()
  const location = useLocation()

  // Get state from location (for registration flow)
  const { email, username, password, isRegistration, credentials, mandatory } = location.state || {}

  useEffect(() => {
    loadQrCode()
  }, [])

  const loadQrCode = async () => {
    try {
      setLoading(true)
      const data = await twoFactorService.setup2FA()
      setQrCode(data.qr_code_url)
      setSecret(data.secret)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to setup 2FA')
    } finally {
      setLoading(false)
    }
  }

  const copyToClipboard = () => {
    navigator.clipboard.writeText(secret)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const handleContinue = () => {
    if (isRegistration) {
      // For registration, go to 2FA verify with credentials
      navigate('/2fa-verify', {
        state: {
          credentials: { username, password },
          email,
          isRegistration: true
        }
      })
    } else {
      // For existing users or forced setup, go to regular 2FA verify
      // Pass forceSetup flag so verify page knows to handle differently
      navigate('/2fa-verify', {
        state: {
          isSetup: true,
          forceSetup: location.state?.forceSetup,
          user: location.state?.user
        }
      })
    }
  }

  const handleCancel = () => {
    if (isRegistration) {
      navigate('/auth-method-select', {
        state: { email, username, password, isRegistration: true }
      })
    } else {
      navigate('/')
    }
  }

  return (
    <div className="min-h-screen bg-white flex items-center justify-center p-4">
      <div className="w-full max-w-md border border-black p-8">
        <h1 className="text-3xl font-bold text-black mb-2 text-center">
          {mandatory ? 'Setup Two-Factor Authentication' : 'Enable Two-Factor Authentication'}
        </h1>
        <p className="text-center text-black mb-8">
          {mandatory ? (
            <span className="font-bold">2FA is required for all accounts. Please complete setup to continue.</span>
          ) : (
            'Secure your account with 2FA'
          )}
        </p>

        {error && (
          <div className="border border-black bg-white p-4 mb-6 text-black">
            {error}
          </div>
        )}

        {loading ? (
          <div className="text-center text-black">Loading...</div>
        ) : (
          <>
            {/* QR Code Section */}
            <div className="mb-8 p-6 border border-black text-center">
              <p className="text-black font-bold mb-4">
                Step 1: Scan QR Code with Authenticator App
              </p>
              {qrCode ? (
                <QRCodeSVG
                  value={qrCode}
                  size={256}
                  level="H"
                  includeMargin={true}
                  className="mx-auto border border-black"
                />
              ) : (
                <div className="w-64 h-64 mx-auto border border-black flex items-center justify-center bg-gray-100">
                  <p className="text-black">Loading QR Code...</p>
                </div>
              )}
            </div>

            {/* Secret Key Section */}
            <div className="mb-8 p-6 border border-black">
              <p className="text-black font-bold mb-2">
                Step 2: Or enter this code manually
              </p>
              <div className="flex items-center gap-2 mb-4">
                <input
                  type="text"
                  value={secret}
                  readOnly
                  className="flex-1 border border-black p-2 bg-white text-black font-mono text-sm"
                />
                <button
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

            {/* Instructions */}
            <div className="mb-8 p-6 border border-black bg-white">
              <p className="text-black font-bold mb-2">Supported Apps:</p>
              <ul className="list-disc list-inside text-black text-sm space-y-1">
                <li>Google Authenticator</li>
                <li>Microsoft Authenticator</li>
                <li>Authy</li>
                <li>FreeOTP</li>
                <li>1Password</li>
              </ul>
            </div>

            {/* Buttons */}
            <div className="space-y-3">
              <button
                onClick={handleContinue}
                className="w-full bg-black border border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition"
              >
                I've Scanned the Code
              </button>
              {!mandatory && (
                <button
                  onClick={handleCancel}
                  className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition"
                >
                  Back
                </button>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  )
}

export default TwoFactorSetupPage
