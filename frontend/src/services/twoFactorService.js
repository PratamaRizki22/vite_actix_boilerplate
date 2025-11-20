import axios from 'axios'

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL

const twoFactorService = {
  /**
   * Setup 2FA - Get QR code and secret
   * @returns {Promise<{secret: string, qr_code_url: string}>}
   */
  setup2FA: async () => {
    try {
      const token = localStorage.getItem('token')
      const response = await axios.post(
        `${API_BASE_URL}/api/auth/setup-2fa`,
        {},
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      )
      return response.data
    } catch (error) {
      console.error('2FA setup failed:', error)
      throw error
    }
  },

  /**
   * Verify 2FA code
   * @param {string} code - 6-digit TOTP code
   * @returns {Promise<{success: boolean, message: string}>}
   */
  verify2FA: async (code) => {
    try {
      const token = localStorage.getItem('token')
      const response = await axios.post(
        `${API_BASE_URL}/api/auth/verify-2fa`,
        { code },
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      )
      return response.data
    } catch (error) {
      console.error('2FA verification failed:', error)
      throw error
    }
  },

  /**
   * Disable 2FA
   * @returns {Promise<{success: boolean, message: string}>}
   */
  disable2FA: async () => {
    try {
      const token = localStorage.getItem('token')
      const response = await axios.post(
        `${API_BASE_URL}/api/auth/disable-2fa`,
        {},
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      )
      return response.data
    } catch (error) {
      console.error('2FA disable failed:', error)
      throw error
    }
  },
}

export default twoFactorService
