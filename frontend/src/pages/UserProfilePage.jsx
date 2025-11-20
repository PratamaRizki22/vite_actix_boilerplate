import { useState, useEffect } from 'react'
import { useParams, useLocation, Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../context/AuthContext'
import userService from '../services/userService'
import postService from '../services/postService'

const UserProfilePage = () => {
  const { userId } = useParams()
  const location = useLocation()
  const navigate = useNavigate()
  const { isAuthenticated, user } = useAuth()
  const [userData, setUserData] = useState(null)
  const [userPosts, setUserPosts] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  // Get userId from state or params
  const targetUserId = userId || location.state?.userId

  useEffect(() => {
    if (isAuthenticated && targetUserId) {
      fetchUserProfile()
    }
  }, [isAuthenticated, targetUserId])

  const fetchUserProfile = async () => {
    try {
      setLoading(true)
      setError('')

      // Fetch user data
      const response = await userService.getAllUsers()
      const foundUser = response.find(u => u.id === parseInt(targetUserId))
      
      if (!foundUser) {
        setError('User not found')
        setLoading(false)
        return
      }

      setUserData(foundUser)

      // Fetch all posts and filter by user
      const posts = await postService.getFeed()
      const filtered = posts.filter(p => p.user_id === parseInt(targetUserId))
      setUserPosts(filtered)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to fetch user profile')
    } finally {
      setLoading(false)
    }
  }

  if (!isAuthenticated) {
    return (
      <div className="container mx-auto px-4 py-8 pt-32">
        <div className="border border-black p-8 text-center">
          <p className="text-black font-bold">Please log in to view user profiles</p>
        </div>
      </div>
    )
  }

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8 pt-32">
        <div className="text-center text-black font-bold">Loading profile...</div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-8 pt-32">
        <div className="border border-black bg-white p-4 mb-6 text-black font-bold">
          {error}
        </div>
        <Link to="/users">
          <button className="bg-black border border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition">
            Back to Users
          </button>
        </Link>
      </div>
    )
  }

  if (!userData) {
    return (
      <div className="container mx-auto px-4 py-8 pt-32">
        <div className="text-center text-black font-bold">User not found</div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8 pt-32">
      <div className="max-w-3xl mx-auto">
        {/* Back Button */}
        <div className="mb-6">
          <Link to="/users">
            <button className="bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition">
              ← Back to Users
            </button>
          </Link>
        </div>

        {/* User Info */}
        <div className="border border-black p-8 mb-8 bg-white">
          <h1 className="text-3xl font-bold text-black mb-4">{userData.username}</h1>
          <div className="flex gap-4">
            <div>
              <p className="text-sm text-black font-bold">User ID</p>
              <p className="text-black">{userData.id}</p>
            </div>
            <div>
              <p className="text-sm text-black font-bold">Role</p>
              <p className="text-black">{userData.role}</p>
            </div>
          </div>
        </div>

        {/* User Posts */}
        <div className="border border-black p-8">
          <h2 className="text-2xl font-bold text-black mb-6">
            Posts by {userData.username}
          </h2>

          {userPosts.length === 0 ? (
            <div className="text-center text-black font-bold border border-black p-8 bg-white">
              No posts yet
            </div>
          ) : (
            <div className="space-y-4">
              {userPosts.map((post) => (
                <div key={post.id} className="border border-black p-6 bg-white">
                  <h3 className="text-xl font-bold text-black mb-2">{post.title}</h3>
                  <p className="text-black mb-4 line-clamp-3">{post.content}</p>
                  <div className="text-xs text-black mb-4 font-bold">
                    <p>Posted: {new Date(post.created_at).toLocaleDateString()}</p>
                  </div>
                  <Link to={`/posts`}>
                    <button className="bg-white border border-black text-black font-bold px-3 py-1 hover:bg-black hover:text-white transition">
                      View →
                    </button>
                  </Link>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default UserProfilePage
