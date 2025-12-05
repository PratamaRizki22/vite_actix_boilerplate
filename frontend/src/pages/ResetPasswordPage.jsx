import { useState, useEffect } from 'react'
import { useNavigate, useSearchParams, Link } from 'react-router-dom'
import authService from '../services/authService'

const ResetPasswordPage = () => {
    const [searchParams] = useSearchParams()
    const navigate = useNavigate()
    const token = searchParams.get('token')

    const [password, setPassword] = useState('')
    const [confirmPassword, setConfirmPassword] = useState('')
    const [status, setStatus] = useState('idle') // idle, loading, success, error
    const [message, setMessage] = useState('')

    useEffect(() => {
        if (!token) {
            setStatus('error')
            setMessage('Invalid or missing reset token.')
        }
    }, [token])

    const handleSubmit = async (e) => {
        e.preventDefault()

        if (password !== confirmPassword) {
            setStatus('error')
            setMessage('Passwords do not match.')
            return
        }

        if (password.length < 8) {
            setStatus('error')
            setMessage('Password must be at least 8 characters long.')
            return
        }

        setStatus('loading')
        setMessage('')

        try {
            await authService.resetPassword(token, password)
            setStatus('success')
            setMessage('Password has been reset successfully. You can now login with your new password.')
            setTimeout(() => {
                navigate('/login')
            }, 3000)
        } catch (err) {
            setStatus('error')
            setMessage(err.response?.data?.error || 'Failed to reset password. The link may have expired.')
        }
    }

    if (!token) {
        return (
            <div className="min-h-screen flex items-center justify-center bg-white p-4">
                <div className="max-w-md w-full border border-black p-8 text-center">
                    <h2 className="text-2xl font-bold text-red-600 mb-4">Invalid Link</h2>
                    <p className="mb-6">The password reset link is invalid or missing.</p>
                    <Link to="/forgot-password">
                        <button className="bg-black text-white font-bold py-2 px-4 hover:bg-gray-800 transition">
                            Request New Link
                        </button>
                    </Link>
                </div>
            </div>
        )
    }

    return (
        <div className="min-h-screen flex items-center justify-center bg-white p-4">
            <div className="max-w-md w-full border border-black p-8">
                <h2 className="text-3xl font-bold text-black mb-6 text-center">Reset Password</h2>

                {status === 'success' ? (
                    <div className="text-center">
                        <div className="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded relative mb-6" role="alert">
                            <span className="block sm:inline">{message}</span>
                        </div>
                        <p className="text-sm text-gray-600 mb-4">Redirecting to login in 3 seconds...</p>
                        <Link to="/login">
                            <button className="w-full bg-black text-white font-bold py-2 px-4 hover:bg-gray-800 transition">
                                Login Now
                            </button>
                        </Link>
                    </div>
                ) : (
                    <form onSubmit={handleSubmit} className="space-y-6">
                        {status === 'error' && (
                            <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                                <span className="block sm:inline">{message}</span>
                            </div>
                        )}

                        <div>
                            <label htmlFor="password" className="block text-black font-bold mb-2 text-sm">
                                New Password
                            </label>
                            <input
                                id="password"
                                type="password"
                                required
                                minLength={8}
                                value={password}
                                onChange={(e) => setPassword(e.target.value)}
                                className="w-full border border-black p-2 text-black focus:outline-none focus:ring-2 focus:ring-black"
                                placeholder="Enter new password"
                            />
                        </div>

                        <div>
                            <label htmlFor="confirmPassword" className="block text-black font-bold mb-2 text-sm">
                                Confirm New Password
                            </label>
                            <input
                                id="confirmPassword"
                                type="password"
                                required
                                minLength={8}
                                value={confirmPassword}
                                onChange={(e) => setConfirmPassword(e.target.value)}
                                className="w-full border border-black p-2 text-black focus:outline-none focus:ring-2 focus:ring-black"
                                placeholder="Confirm new password"
                            />
                        </div>

                        <button
                            type="submit"
                            disabled={status === 'loading'}
                            className="w-full bg-black text-white font-bold py-2 px-4 hover:bg-gray-800 transition disabled:opacity-50"
                        >
                            {status === 'loading' ? 'Resetting...' : 'Reset Password'}
                        </button>
                    </form>
                )}
            </div>
        </div>
    )
}

export default ResetPasswordPage
