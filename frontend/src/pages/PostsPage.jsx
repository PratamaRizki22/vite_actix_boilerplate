import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import postService from '../services/postService'
import { useAuth } from '../context/AuthContext'
import Modal from '../components/common/Modal'

const PostsPage = () => {
  const [posts, setPosts] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [editingId, setEditingId] = useState(null)
  const [formTitle, setFormTitle] = useState('')
  const [formContent, setFormContent] = useState('')

  // Delete Modal State
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
  const [postToDelete, setPostToDelete] = useState(null)

  const { isAuthenticated, user } = useAuth()

  useEffect(() => {
    if (isAuthenticated) {
      fetchPosts()
    }
  }, [isAuthenticated])

  const fetchPosts = async () => {
    try {
      setLoading(true)
      setError('')
      const data = await postService.getAllPosts()
      setPosts(data)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to fetch posts')
    } finally {
      setLoading(false)
    }
  }

  const resetForm = () => {
    setFormTitle('')
    setFormContent('')
    setEditingId(null)
  }

  const handleSubmit = async (e) => {
    e.preventDefault()
    setError('')

    try {
      if (editingId) {
        await postService.updatePost(editingId, formTitle, formContent)
      } else {
        await postService.createPost(formTitle, formContent)
      }
      resetForm()
      fetchPosts()
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to save post')
    }
  }

  const startEdit = (post) => {
    setEditingId(post.id)
    setFormTitle(post.title)
    setFormContent(post.content)
  }

  const handleDeleteClick = (post) => {
    setPostToDelete(post)
    setIsDeleteModalOpen(true)
  }

  const confirmDelete = async () => {
    if (!postToDelete) return

    try {
      setError('')
      await postService.deletePost(postToDelete.id)
      setPosts(prev => prev.filter(p => p.id !== postToDelete.id))
      setIsDeleteModalOpen(false)
      setPostToDelete(null)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to delete post')
      setIsDeleteModalOpen(false)
    }
  }

  if (!isAuthenticated) {
    return (
      <div className="border border-black p-8 text-center">
        <p className="text-black">Please log in to view posts</p>
      </div>
    )
  }

  const canEditPost = (post) => {
    return user && (user.id === post.user_id || user.role === 'admin')
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-6xl mx-auto">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Create/Edit Form - Left Side */}
          <div className="lg:col-span-1">
            <div className="border border-black p-6 sticky top-24">
              <h2 className="text-xl font-bold text-black mb-4">
                {editingId ? 'Edit Post' : 'Create Post'}
              </h2>

              {error && (
                <div className="border border-black bg-white p-3 mb-4 text-black text-sm font-bold">
                  {error}
                </div>
              )}

              <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                  <label className="block text-black font-bold mb-2 text-sm">Title</label>
                  <input
                    type="text"
                    value={formTitle}
                    onChange={(e) => setFormTitle(e.target.value)}
                    required
                    className="w-full border border-black p-2 bg-white text-black text-sm"
                    placeholder="Post title"
                  />
                </div>

                <div>
                  <label className="block text-black font-bold mb-2 text-sm">Content</label>
                  <textarea
                    value={formContent}
                    onChange={(e) => setFormContent(e.target.value)}
                    required
                    rows={8}
                    className="w-full border border-black p-2 bg-white text-black text-sm"
                    placeholder="Post content"
                  />
                </div>

                <div className="flex gap-2">
                  <button
                    type="submit"
                    className="flex-1 bg-black border border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition text-sm"
                  >
                    {editingId ? 'Update' : 'Create'}
                  </button>
                  {editingId && (
                    <button
                      type="button"
                      onClick={resetForm}
                      className="flex-1 bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition text-sm"
                    >
                      Cancel
                    </button>
                  )}
                </div>
              </form>

              <Link to="/">
                <button className="w-full mt-4 bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition text-sm">
                  ← Back to Feed
                </button>
              </Link>
            </div>
          </div>

          {/* Posts List - Right Side */}
          <div className="lg:col-span-2">
            <h2 className="text-2xl font-bold text-black mb-6">My Posts</h2>

            {loading ? (
              <p className="text-black">Loading posts...</p>
            ) : posts.filter((post) => canEditPost(post)).length === 0 ? (
              <div className="text-center text-black border border-black p-8">
                <p className="font-bold">No posts yet</p>
              </div>
            ) : (
              <div className="space-y-4">
                {posts
                  .filter((post) => canEditPost(post))
                  .map((post) => (
                    <div key={post.id} className="border border-black p-4 bg-white">
                      <h3 className="text-lg font-bold text-black mb-2">{post.title}</h3>
                      <p className="text-black mb-3 line-clamp-2">{post.content}</p>
                      <div className="text-xs text-black mb-4 font-bold">
                        <p>Posted: {new Date(post.created_at).toLocaleDateString()}</p>
                      </div>
                      <div className="flex gap-2">
                        <button
                          onClick={() => startEdit(post)}
                          className="bg-white border border-black text-black font-bold px-3 py-1 hover:bg-black hover:text-white transition text-sm"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleDeleteClick(post)}
                          className="bg-white border border-black text-black font-bold px-3 py-1 hover:bg-black hover:text-white transition text-sm"
                        >
                          Delete
                        </button>
                      </div>
                    </div>
                  ))}
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Delete Confirmation Modal */}
      <Modal isOpen={isDeleteModalOpen} onClose={() => setIsDeleteModalOpen(false)}>
        <div className="text-center">
          <h3 className="text-xl font-bold text-black mb-4">Delete Post</h3>
          <p className="text-black mb-6">Are you sure you want to delete this post? This action cannot be undone.</p>
          <div className="flex gap-4 justify-center">
            <button
              onClick={() => setIsDeleteModalOpen(false)}
              className="bg-white border border-black text-black font-bold py-2 px-6 hover:bg-gray-100 transition"
            >
              Cancel
            </button>
            <button
              onClick={confirmDelete}
              className="bg-red-600 border border-red-600 text-white font-bold py-2 px-6 hover:bg-red-700 transition"
            >
              Delete
            </button>
          </div>
        </div>
      </Modal>
    </div>
  )
}

export default PostsPage
