import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router-dom'
import { GoogleOAuthProvider } from '@react-oauth/google'
import './styles/index.css'
import router from './router.jsx'
import { setupCustomValidation } from './utils/validation'

const googleClientId = import.meta.env.VITE_GOOGLE_CLIENT_ID

// Initialize custom validation styling
document.addEventListener('DOMContentLoaded', setupCustomValidation)

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <GoogleOAuthProvider clientId={googleClientId}>
      <RouterProvider router={router} />
    </GoogleOAuthProvider>
  </StrictMode>
)
