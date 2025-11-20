import axios from 'axios'

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL

const googleAuthService = {
  /**
   * Verify Google token with backend and get JWT
   * @param {string} googleToken - ID token from Google OAuth
   * @returns {Promise<{token: string, user: object}>}
   */
  verifyGoogleToken: async (googleToken) => {
    try {
      const response = await axios.post(`${API_BASE_URL}/api/auth/google/callback`, {
        token: googleToken,
      })
      
      if (response.data.access_token) {
        // Store JWT token
        localStorage.setItem('token', response.data.access_token)
        localStorage.setItem('user', JSON.stringify(response.data.user || {}))
        
        // Dispatch custom event to notify AuthContext of the update
        window.dispatchEvent(new Event('auth-update'));
        
        return {
          token: response.data.access_token,
          user: response.data.user,
        }
      }
      
      throw new Error('No token in response')
    } catch (error) {
      console.error('Google token verification failed:', error)
      throw error
    }
  },
}

export default googleAuthService
