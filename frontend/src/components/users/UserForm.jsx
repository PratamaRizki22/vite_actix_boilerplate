import { useState } from 'react'
import { useUsers } from '../../hooks/useUsers'

const UserForm = ({ onSuccess }) => {
  const [formData, setFormData] = useState({ username: '', email: '' })
  const { createUser, loading } = useUsers()

  const handleSubmit = async (e) => {
    e.preventDefault()
    try {
      await createUser(formData)
      setFormData({ username: '', email: '' })
      onSuccess?.()
    } catch (error) {
      console.error('Failed to create user:', error)
    }
  }

  const handleChange = (e) => {
    setFormData(prev => ({
      ...prev,
      [e.target.name]: e.target.value
    }))
  }

  return (
    <form onSubmit={handleSubmit}>
      <h3>Create New User</h3>
      <div className="form-group">
        <label>Username:</label>
        <input
          type="text"
          name="username"
          className="form-control"
          value={formData.username}
          onChange={handleChange}
          required
          placeholder="Enter username"
        />
      </div>
      <div className="form-group">
        <label>Email:</label>
        <input
          type="email"
          name="email"
          className="form-control"
          value={formData.email}
          onChange={handleChange}
          required
          placeholder="Enter email address"
        />
      </div>
      <button type="submit" className="btn btn-primary" disabled={loading}>
        {loading ? 'Creating...' : 'Create User'}
      </button>
    </form>
  )
}

export default UserForm