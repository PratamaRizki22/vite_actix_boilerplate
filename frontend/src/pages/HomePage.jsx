import { Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../context/AuthContext'
import { useState, useEffect } from 'react'
import postService from '../services/postService'

const HomePage = () => {
  const { isAuthenticated, user } = useAuth()
  const navigate = useNavigate()
  const [posts, setPosts] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [searchPostTerm, setSearchPostTerm] = useState('')
  const [filterDate, setFilterDate] = useState('')
  const [showDateFilter, setShowDateFilter] = useState(false)
  const [searching, setSearching] = useState(false)

  useEffect(() => {
    if (isAuthenticated) {
      fetchFeed()
    }
  }, [isAuthenticated])

  const fetchFeed = async () => {
    try {
      setLoading(true)
      setError('')
      const data = await postService.getFeed()
      setPosts(data)
      setSearching(false)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to fetch posts')
    } finally {
      setLoading(false)
    }
  }

  const handleSearchPosts = async (e) => {
    e.preventDefault()
    if (!searchPostTerm.trim() && !filterDate) {
      fetchFeed()
      return
    }

    try {
      setLoading(true)
      setError('')
      setSearching(true)
      
      let data
      if (searchPostTerm.trim()) {
        // Search by text
        data = await postService.searchPosts(searchPostTerm)
      } else {
        // If no search term, get all posts
        data = await postService.getFeed()
      }
      
      // Filter by date if provided
      if (filterDate) {
        const filterDateObj = new Date(filterDate)
        data = data.filter(post => {
          const postDate = new Date(post.created_at)
          return postDate.toDateString() === filterDateObj.toDateString()
        })
      }
      
      setPosts(data)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to search posts')
    } finally {
      setLoading(false)
    }
  }

  const handleClearFilters = () => {
    setSearchPostTerm('')
    setFilterDate('')
    fetchFeed()
  }

  if (!isAuthenticated) {
    return (
      <div className="min-h-[60vh] flex items-center justify-center">
        <div className="border border-black p-8 w-full max-w-md">
          <h1 className="text-3xl font-bold text-black mb-6 text-center">Welcome</h1>
          <p className="text-black mb-6 text-center">Please log in or register to continue.</p>
          
          <div className="space-y-4">
            <button
              onClick={() => navigate('/login')}
              className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition"
            >
              Login
            </button>
            <button
              onClick={() => navigate('/register')}
              className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition"
            >
              Register
            </button>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-4xl font-bold text-black mb-2">Feed</h1>
          <p className="text-lg text-black">Welcome, {user?.username || 'User'}</p>
        </div>

        {/* Search & Actions */}
        <div className="mb-8">
          {/* Search Bar */}
          <form onSubmit={handleSearchPosts} className="space-y-3">
            {/* Search Input Row */}
            <div className="flex gap-2">
              <input
                type="text"
                value={searchPostTerm}
                onChange={(e) => setSearchPostTerm(e.target.value)}
                placeholder="Search posts..."
                className="flex-1 border border-black p-3 bg-white text-black font-bold"
              />
              <button
                type="submit"
                className="bg-black border border-black text-white font-bold py-3 px-8 hover:bg-white hover:text-black transition"
              >
                Search
              </button>
              
              <button
                type="button"
                onClick={() => setShowDateFilter(!showDateFilter)}
                className="border-2 border-black bg-white text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition"
              >
                {showDateFilter ? '▼' : '▶'} Date
              </button>

              {filterDate && (
                <button
                  type="button"
                  onClick={handleClearFilters}
                  className="border border-black bg-white text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition"
                >
                  Reset
                </button>
              )}
            </div>

            {/* Date Picker - Show when opened */}
            {showDateFilter && (
              <div className="flex gap-2">
                <input
                  type="date"
                  value={filterDate}
                  onChange={(e) => {
                    setFilterDate(e.target.value)
                    setSearching(true)
                  }}
                  className="flex-1 border border-black p-2 bg-white text-black font-bold"
                  autoFocus
                />
                <button
                  type="button"
                  onClick={() => setShowDateFilter(false)}
                  className="border border-black bg-white text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition"
                >
                  Close
                </button>
              </div>
            )}

            {filterDate && (
              <div className="text-black font-bold text-sm bg-gray-100 p-2 border border-black">
                Filtering by: {new Date(filterDate).toLocaleDateString('en-US', { 
                  year: 'numeric', 
                  month: 'short', 
                  day: 'numeric' 
                })}
              </div>
            )}
          </form>

          {/* Action Buttons - Create Post & Users */}
          <div className="flex gap-2">
            <Link to="/posts" className="flex-1">
              <button className="w-full bg-black border border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition">
                Create Post
              </button>
            </Link>
            {user?.role === 'admin' && (
              <Link to="/users" className="flex-1">
                <button className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition">
                  Users Management
                </button>
              </Link>
            )}
          </div>
        </div>

        {/* Error */}
        {error && (
          <div className="border border-black bg-white p-4 mb-6 text-black font-bold">
            {error}
          </div>
        )}

        {/* Posts Feed */}
        {loading ? (
          <div className="text-center text-black">Loading posts...</div>
        ) : posts.length === 0 ? (
          <div className="text-center text-black border border-black p-8">
            <p className="font-bold mb-4">No posts yet</p>
            <Link to="/posts">
              <button className="bg-black border border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition">
                Create the first post
              </button>
            </Link>
          </div>
        ) : (
          <div className="space-y-4">
            {posts.map((post) => (
              <div key={post.id} className="border border-black p-6 bg-white">
                <h3 className="text-xl font-bold text-black mb-2">{post.title}</h3>
                <p className="text-black mb-4 line-clamp-3">{post.content}</p>
                <div className="text-xs text-black mb-4 font-bold">
                <p>By {post.username}</p>
                  <p>Posted: {new Date(post.created_at).toLocaleDateString()}</p>
                </div>
                <Link to={`/user/${post.user_id}`}>
                  <button className="bg-white border border-black text-black font-bold px-3 py-1 hover:bg-black hover:text-white transition">
                    View Profile →
                  </button>
                </Link>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}

export default HomePage
