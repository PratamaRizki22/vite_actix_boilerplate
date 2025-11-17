import { useState } from 'react'
import PostList from '../components/posts/PostList'
import PostForm from '../components/posts/PostForm'

const PostsPage = () => {
  const [showForm, setShowForm] = useState(false)

  return (
    <div>
      <div style={{ 
        display: 'flex', 
        justifyContent: 'space-between', 
        alignItems: 'center', 
        marginBottom: '20px' 
      }}>
        <h2>Post Management</h2>
        <button 
          className="btn btn-primary"
          onClick={() => setShowForm(!showForm)}
        >
          {showForm ? 'Cancel' : 'Create New Post'}
        </button>
      </div>

      {showForm && (
        <div className="card">
          <PostForm onSuccess={() => setShowForm(false)} />
        </div>
      )}

      <PostList />
    </div>
  )
}

export default PostsPage