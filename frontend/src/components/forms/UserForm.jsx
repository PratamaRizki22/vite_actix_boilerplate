import { useState } from 'react'
import { useUsers } from '../../hooks/useUsers'
import Input from '../common/Input'
import Button from '../common/Button'
import { validateEmail, validateRequired } from '../../utils/validation'

const UserForm = ({ onSuccess }) => {
  const [formData, setFormData] = useState({ username: '', email: '' })
  const [errors, setErrors] = useState({})
  const { createUser, loading } = useUsers()

  const validateForm = () => {
    const newErrors = {}
    
    if (!validateRequired(formData.username)) {
      newErrors.username = 'Username is required'
    }
    
    if (!validateRequired(formData.email)) {
      newErrors.email = 'Email is required'
    } else if (!validateEmail(formData.email)) {
      newErrors.email = 'Please enter a valid email'
    }
    
    setErrors(newErrors)
    return Object.keys(newErrors).length === 0
  }

  const handleSubmit = async (e) => {
    e.preventDefault()
    
    if (!validateForm()) {
      return
    }
    
    try {
      await createUser(formData)
      setFormData({ username: '', email: '' })
      setErrors({})
      onSuccess?.()
    } catch (error) {
      console.error('Failed to create user:', error)
    }
  }

  const handleChange = (e) => {
    const { name, value } = e.target
    setFormData(prev => ({
      ...prev,
      [name]: value
    }))
    // Clear error when user starts typing
    if (errors[name]) {
      setErrors(prev => ({ ...prev, [name]: '' }))
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <h3 className="text-xl font-semibold text-gray-900">Create New User</h3>
      
      <Input
        label="Username:"
        name="username"
        value={formData.username}
        onChange={handleChange}
        error={errors.username}
        required
        placeholder="Enter username"
      />
      
      <Input
        label="Email:"
        name="email"
        type="email"
        value={formData.email}
        onChange={handleChange}
        error={errors.email}
        required
        placeholder="Enter email address"
      />
      
      <Button 
        type="submit" 
        disabled={loading}
      >
        {loading ? 'Creating...' : 'Create User'}
      </Button>
    </form>
  )
}

export default UserForm