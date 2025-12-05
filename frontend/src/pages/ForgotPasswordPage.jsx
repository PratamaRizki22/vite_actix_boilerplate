import { useState } from 'react'
import { Link } from 'react-router-dom'
import authService from '../services/authService'

const ForgotPasswordPage = () => {
    const [email, setEmail] = useState('')
    const [status, setStatus] = useState('idle') // idle, loading, success, error
    const [message, setMessage] = useState('')

    const handleSubmit = async (e) => {
        e.preventDefault()
        setStatus('loading')
        setMessage('')

        try {
            await authService.requestPasswordReset(email)
            setStatus('success')
            setMessage('If an account exists with this email, you will receive a password reset link shortly.')
        } catch (err) {
            setStatus('error')
            setMessage(err.response?.data?.error || 'Failed to request password reset. Please try again.')
        }
    }

    return (
        <div className="min-h-screen flex items-center justify-center bg-white p-4">
            <div className="max-w-md w-full border border-black p-8">
                <h2 className="text-3xl font-bold text-black mb-6 text-center">Forgot Password</h2>

                {status === 'success' ? (
                    <div className="text-center">
                        <div className="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded relative mb-6" role="alert">
                            <span className="block sm:inline">{message}</span>
                        </div>
                        <Link to="/login">
                            <button className="w-full bg-black text-white font-bold py-2 px-4 hover:bg-gray-800 transition">
                                Back to Login
                            </button>
                        </Link>
                    </div>
                ) : (
                    <form onSubmit={handleSubmit} className="space-y-6">
                        <p className="text-gray-600 text-sm mb-4">
                            Enter your email address and we'll send you a link to reset your password.
                        </p>

                        {status === 'error' && (
                            <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                                <span className="block sm:inline">{message}</span>
                            </div>
                        )}

                        <div>
                            <label htmlFor="email" className="block text-black font-bold mb-2 text-sm">
                                Email Address
                            </label>
                            <input
                                id="email"
                                type="email"
                                required
                                value={email}
                                onChange={(e) => setEmail(e.target.value)}
                                className="w-full border border-black p-2 text-black focus:outline-none focus:ring-2 focus:ring-black"
                                placeholder="Enter your email"
                            />
                        </div>

                        <button
                            type="submit"
                            disabled={status === 'loading'}
                            className="w-full bg-black text-white font-bold py-2 px-4 hover:bg-gray-800 transition disabled:opacity-50"
                        >
                            {status === 'loading' ? 'Sending...' : 'Send Reset Link'}
                        </button>

                        <div className="text-center mt-4">
                            <Link to="/login" className="text-sm text-gray-600 hover:text-black hover:underline">
                                Back to Login
                            </Link>
                        </div>
                    </form>
                )}
            </div>
        </div>
    )
}

export default ForgotPasswordPage
