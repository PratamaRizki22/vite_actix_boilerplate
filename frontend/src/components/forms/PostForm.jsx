import { useState } from 'react'
import { usePosts } from '../../hooks/usePosts'
import Input from '../common/Input'
import Button from '../common/Button'
import { validateRequired } from '../../utils/validation'

const PostForm = ({ onSuccess }) => {
  const [formData, setFormData] = useState({ 
    title: '', 
    content: '', 
    user_id: 1 
  })
  const [errors, setErrors] = useState({})
  const { createPost, loading, error } = usePosts()

  const validateForm = () => {
    const newErrors = {}
    
    if (!validateRequired(formData.title)) {
      newErrors.title = 'Title is required'
    }
    
    if (!validateRequired(formData.content)) {
      newErrors.content = 'Content is required'
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
      const dataToSend = {
        ...formData,
        user_id: Number(formData.user_id)
      }
      await createPost(dataToSend)
      setFormData({ title: '', content: '', user_id: 1 })
      setErrors({})
      onSuccess?.()
    } catch (error) {
      console.error('Failed to create post:', error)
    }
  }

  const handleChange = (e) => {
    const { name, value } = e.target
    setFormData(prev => ({
      ...prev,
      [name]: value
    }))
    if (errors[name]) {
      setErrors(prev => ({ ...prev, [name]: '' }))
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <h3 className="text-xl font-semibold text-gray-900">Create New Post</h3>
      
      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          <strong>Error:</strong> {error}
        </div>
      )}
      
      <Input
        label="Title:"
        name="title"
        value={formData.title}
        onChange={handleChange}
        error={errors.title}
        required
        placeholder="Enter post title"
      />
      
      {/* PERBAIKI BAGIAN INI - ganti dengan Tailwind langsung */}
      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-700 mb-1">Content:</label>
        <textarea
          name="content"
          className={`w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-vertical ${
            errors.content ? 'border-red-500 focus:ring-red-500' : ''
          }`}
          rows="4"
          value={formData.content}
          onChange={handleChange}
          required
          placeholder="Enter post content"
        />
        {errors.content && (
          <p className="text-red-500 text-sm mt-1">{errors.content}</p>
        )}
      </div>
      
      <Input
        label="User ID:"
        name="user_id"
        type="number"
        value={formData.user_id}
        onChange={handleChange}
        required
        min="1"
      />
      
      <Button 
        type="submit"
        disabled={loading}
      >
        {loading ? 'Creating...' : 'Create Post'}
      </Button>
    </form>
  )
}

export default PostForm