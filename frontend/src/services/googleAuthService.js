import axios from 'axios'

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL

const googleAuthService = {
  /**
   * Verify Google token with backend and get MFA options
   * @param {string} googleToken - ID token from Google OAuth
   * @returns {Promise<{temp_token: string, mfa_methods: string[], user: object}>}
   */
  verifyGoogleToken: async (googleToken) => {
    try {
      const response = await axios.post(`${API_BASE_URL}/api/auth/google/callback`, {
        token: googleToken,
      })
      
      // Return MFA options - MFA verification is now required
      return {
        temp_token: response.data.temp_token,
        mfa_methods: response.data.mfa_methods,
        user: response.data.user,
      }
    } catch (error) {
      console.error('Google token verification failed:', error)
      throw error
    }
  },
}

export default googleAuthService
