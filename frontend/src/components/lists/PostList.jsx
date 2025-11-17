import { usePosts } from '../../hooks/usePosts'
import Button from '../common/Button'
import Card from '../common/Card'

const PostList = () => {
  const { posts, loading, error, deletePost } = usePosts()

  if (loading) return <Card>Loading posts...</Card>
  if (error) return <Card className="text-red-500">Error: {error}</Card>

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
        <Card>No posts found</Card>
      ) : (
        posts.map(post => (
          <Card key={post.id}>
            <div className="flex justify-between items-start">
              <div className="flex-1">
                <h3 className="text-lg font-semibold mb-2">{post.title}</h3>
                <p className="text-gray-700 mb-3">{post.content}</p>
                <small className="text-gray-500">
                  User ID: {post.user_id} | Post ID: {post.id}
                </small>
              </div>
              <Button 
                variant="danger"
                onClick={() => handleDelete(post.id)}
                className="ml-4"
              >
                Delete
              </Button>
            </div>
          </Card>
        ))
      )}
    </div>
  )
}

export default PostList