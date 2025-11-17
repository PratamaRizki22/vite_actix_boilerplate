import { useState } from 'react'
import { usePosts } from '../../hooks/usePosts'

const PostForm = ({ onSuccess }) => {
  const [formData, setFormData] = useState({ 
    title: '', 
    content: '', 
    user_id: 1 
  })
  const { createPost, loading } = usePosts()

  const handleSubmit = async (e) => {
    e.preventDefault()
    try {
      const dataToSend = {
        ...formData,
        user_id: Number(formData.user_id)  // Convert ke number
      }
      await createPost(dataToSend)
      setFormData({ title: '', content: '', user_id: 1 })
      onSuccess?.()
    } catch (error) {
      console.error('Failed to create post:', error)
    }
  }

  const handleChange = (e) => {
    setFormData(prev => ({
      ...prev,
      [e.target.name]: e.target.name === 'user_id' 
        ? Number(e.target.value)  // Convert ke number langsung
        : e.target.value
    }))
  }

  return (
    <form onSubmit={handleSubmit}>
      <h3>Create New Post</h3>
      
      <div className="form-group">
        <label>Title:</label>
        <input
          type="text"
          name="title"
          className="form-control"
          value={formData.title}
          onChange={handleChange}
          required
          placeholder="Enter post title"
        />
      </div>
      
      <div className="form-group">
        <label>Content:</label>
        <textarea
          name="content"
          className="form-control"
          rows="4"
          value={formData.content}
          onChange={handleChange}
          required
          placeholder="Enter post content"
        />
      </div>
      
      <div className="form-group">
        <label>User ID:</label>
        <input
          type="number"
          name="user_id"
          className="form-control"
          value={formData.user_id}
          onChange={handleChange}
          required
          min="1"
        />
      </div>
      
      <button 
        type="submit" 
        className="btn btn-primary" 
        disabled={loading}
      >
        {loading ? 'Creating...' : 'Create Post'}
      </button>
    </form>
  )
}

export default PostForm