import { usePosts } from '../../hooks/usePosts'
import { useState } from 'react'
import interactionService from '../../services/interactionService'
import Button from '../common/Button'
import Card from '../common/Card'

const PostList = () => {
  const { posts, loading, error, deletePost, updateCommentCount } = usePosts()
  const [likedPosts, setLikedPosts] = useState({})
  const [showComments, setShowComments] = useState({})
  const [commentText, setCommentText] = useState({})
  const [submittingComments, setSubmittingComments] = useState({})
  const [commentsPanelData, setCommentsPanelData] = useState(null)

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

  const handleToggleLike = async (postId) => {
    try {
      const result = await interactionService.toggleLike(postId)
      setLikedPosts(prev => ({
        ...prev,
        [postId]: result.liked
      }))
      // Update likes_count if returned from backend
      if (result.likes_count !== undefined) {
        // Posts state tidak bisa di-update di component ini karena menggunakan hook
        // Tapi state akan otomatis terupdate karena menggunakan usePosts hook
      }
    } catch (err) {
      console.error('Failed to toggle like:', err)
    }
  }

  const handleViewComments = async (postId, postTitle, postUsername) => {
    try {
      const comments = await interactionService.getComments(postId)
      setCommentsPanelData({
        postId,
        postTitle,
        postUsername,
        comments
      })
    } catch (err) {
      console.error('Failed to fetch comments:', err)
    }
  }

  const handleCloseCommentPanel = () => {
    setCommentsPanelData(null)
  }

  const handleCreateComment = async (postId) => {
    const content = commentText[postId]?.trim()
    if (!content) {
      alert('Please enter a comment')
      return
    }

    try {
      setSubmittingComments(prev => ({ ...prev, [postId]: true }))
      await interactionService.createComment(postId, content)
      setCommentText(prev => ({ ...prev, [postId]: '' }))
      // Refresh comments in panel
      const comments = await interactionService.getComments(postId)
      setCommentsPanelData(prev => ({
        ...prev,
        comments
      }))
      // Update comments_count in posts list
      updateCommentCount(postId, comments.length)
    } catch (err) {
      console.error('Failed to create comment:', err)
      alert('Failed to create comment')
    } finally {
      setSubmittingComments(prev => ({ ...prev, [postId]: false }))
    }
  }

  return (
    <div className="flex gap-8">
      {/* Left Side - Posts */}
      <div className="flex-1">
      {posts.length === 0 ? (
        <Card>No posts found</Card>
      ) : (
        posts.map(post => (
          <Card key={post.id}>
            <div className="flex justify-between items-start">
              <div className="flex-1">
                <h3 className="text-lg font-semibold mb-2">{post.title}</h3>
                <p className="text-gray-700 mb-3">{post.content}</p>
                <small className="text-gray-500 block mb-3">
                  By {post.username} | Post ID: {post.id}
                </small>

                {/* Like and Comment Stats */}
                <div className="flex gap-4 mb-4 text-sm">
                  <span className="text-black font-bold">❤️ {post.likes_count || 0} Likes</span>
                  <span className="text-black font-bold">💬 {post.comments_count || 0} Comments</span>
                </div>

                {/* Like and Comment Buttons */}
                <div className="flex gap-2">
                  <Button
                    onClick={() => handleToggleLike(post.id)}
                    className="bg-black hover:bg-gray-800 text-white border border-black py-1 px-3 text-sm"
                  >
                    {likedPosts[post.id] ? '❤️ Liked' : '🤍 Like'}
                  </Button>
                  <Button
                    onClick={() => handleViewComments(post.id, post.title, post.username)}
                    className="bg-white hover:bg-gray-100 text-black border border-black py-1 px-3 text-sm"
                  >
                    💬 View Comments
                  </Button>
                </div>
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

      {/* Right Side - Comments Panel */}
      {commentsPanelData && (
        <div className="w-96 sticky top-8 h-fit">
          <div className="border-4 border-black bg-white p-4">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-bold text-black">Comments</h2>
              <button
                onClick={handleCloseCommentPanel}
                className="text-black font-bold text-xl hover:bg-black hover:text-white w-8 h-8 flex items-center justify-center border border-black"
              >
                ×
              </button>
            </div>

            <div className="mb-4 pb-4 border-b-2 border-black">
              <p className="text-xs font-bold text-black mb-1">Post:</p>
              <p className="text-sm text-black font-bold">{commentsPanelData.postTitle}</p>
              <p className="text-xs text-black mt-1">by {commentsPanelData.postUsername}</p>
            </div>

            {/* Comment Input */}
            <div className="mb-4 pb-4 border-b-2 border-black">
              <textarea
                value={commentText[commentsPanelData.postId] || ''}
                onChange={(e) => setCommentText(prev => ({ ...prev, [commentsPanelData.postId]: e.target.value }))}
                placeholder="Add a comment..."
                className="w-full border-2 border-black p-2 bg-white text-black text-xs mb-2 resize-none font-bold"
                rows="3"
              />
              <button
                onClick={() => handleCreateComment(commentsPanelData.postId)}
                disabled={submittingComments[commentsPanelData.postId]}
                className="w-full bg-black hover:bg-gray-800 text-white border border-black font-bold py-2 px-3 text-xs disabled:opacity-50"
              >
                {submittingComments[commentsPanelData.postId] ? 'Posting...' : 'Post Comment'}
              </button>
            </div>

            {/* Comments List */}
            <div className="max-h-96 overflow-y-auto">
              {commentsPanelData.comments.length === 0 ? (
                <p className="text-black text-xs font-bold">No comments yet</p>
              ) : (
                commentsPanelData.comments.map(comment => (
                  <div key={comment.id} className="mb-3 pb-3 border-b border-gray-300 last:border-b-0">
                    <p className="font-bold text-xs text-black">{comment.username}</p>
                    <p className="text-black text-xs mt-1">{comment.content}</p>
                    <p className="text-gray-600 text-xs mt-1">{new Date(comment.created_at).toLocaleDateString()}</p>
                  </div>
                ))
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default PostList