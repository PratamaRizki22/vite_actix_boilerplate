import { usePosts } from '../../hooks/usePosts'

const PostList = () => {
  const { posts, loading, error, deletePost } = usePosts()

  if (loading) return <div className="card">Loading posts...</div>
  if (error) return <div className="card" style={{ color: 'red' }}>Error: {error}</div>

  const handleDelete = async (id) => {
    if (window.confirm('Are you sure you want to delete this post?')) {
      try {
        await deletePost(id)
      } catch (err) {
        console.error('Failed to delete post:', err)
      }
    }
  }

  return (
    <div>
      {posts.length === 0 ? (
        <div className="card">No posts found</div>
      ) : (
        posts.map(post => (
          <div key={post.id} className="card">
            <div style={{ 
              display: 'flex', 
              justifyContent: 'space-between', 
              alignItems: 'flex-start' 
            }}>
              <div style={{ flex: 1 }}>
                <h3 style={{ marginBottom: '10px' }}>{post.title}</h3>
                <p style={{ marginBottom: '10px', lineHeight: '1.5' }}>
                  {post.content}
                </p>
                <small style={{ color: '#666' }}>
                  User ID: {post.user_id} | Post ID: {post.id}
                </small>
              </div>
              <button 
                className="btn btn-danger"
                onClick={() => handleDelete(post.id)}
                style={{ marginLeft: '15px' }}
              >
                Delete
              </button>
            </div>
          </div>
        ))
      )}
    </div>
  )
}

export default PostList