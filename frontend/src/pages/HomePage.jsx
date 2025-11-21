import { Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../context/AuthContext'
import { useState, useEffect } from 'react'
import postService from '../services/postService'
import interactionService from '../services/interactionService'

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
  const [filterType, setFilterType] = useState('All Time')
  const [showCustomDateModal, setShowCustomDateModal] = useState(false)
  const [customDateStart, setCustomDateStart] = useState('')
  const [customDateEnd, setCustomDateEnd] = useState('')
  const [commentText, setCommentText] = useState({})
  const [submittingComments, setSubmittingComments] = useState({})
  const [selectedPostId, setSelectedPostId] = useState(null)
  const [selectedCommentPostId, setSelectedCommentPostId] = useState(null)
  const [commentsPanelData, setCommentsPanelData] = useState(null)
  const [sortBy, setSortBy] = useState('newest')
  const [showSortMenu, setShowSortMenu] = useState(false)
  const [likedPosts, setLikedPosts] = useState({})
  const [showComments, setShowComments] = useState({})

  useEffect(() => {
    // Set default date range: 1 week ago to today
    const today = new Date()
    const oneWeekAgo = new Date(today)
    oneWeekAgo.setDate(today.getDate() - 7)
    
    setCustomDateStart(oneWeekAgo.toISOString().split('T')[0])
    setCustomDateEnd(today.toISOString().split('T')[0])
  }, [])

  useEffect(() => {
    if (isAuthenticated) {
      fetchFeed()
    }
  }, [isAuthenticated])

  const fetchFeed = async () => {
    try {
      setLoading(true)
      setError('')
      let data = await postService.getFeed()
      
      // Apply sorting
      if (sortBy === 'newest') {
        data.sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
      } else if (sortBy === 'oldest') {
        data.sort((a, b) => new Date(a.created_at) - new Date(b.created_at))
      }
      
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
        const customDateEnd = sessionStorage.getItem('customDateEnd')
        const filterDateObj = new Date(filterDate)
        
        if (customDateEnd) {
          // Range filtering for custom dates
          const endDateObj = new Date(customDateEnd)
          endDateObj.setHours(23, 59, 59, 999) // Include end of day
          
          data = data.filter(post => {
            const postDate = new Date(post.created_at)
            return postDate >= filterDateObj && postDate <= endDateObj
          })
        } else {
          // Single day filtering
          data = data.filter(post => {
            const postDate = new Date(post.created_at)
            return postDate.toDateString() === filterDateObj.toDateString()
          })
        }
      }
      
      // Apply sorting
      if (sortBy === 'newest') {
        data.sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
      } else if (sortBy === 'oldest') {
        data.sort((a, b) => new Date(a.created_at) - new Date(b.created_at))
      }
      
      setPosts(data)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to search posts')
    } finally {
      setLoading(false)
    }
  }

  const handleDateRangeFilter = (option) => {
    const today = new Date()
    let startDate = null
    let label = ''

    switch(option) {
      case 'today':
        startDate = new Date(today)
        label = 'Today'
        break
      case 'week':
        startDate = new Date(today.setDate(today.getDate() - 7))
        label = 'Last 7 Days'
        break
      case 'month':
        startDate = new Date(today.setMonth(today.getMonth() - 1))
        label = 'Last Month'
        break
      case 'year':
        startDate = new Date(today.setFullYear(today.getFullYear() - 1))
        label = 'Last Year'
        break
      case 'all':
        setFilterDate('')
        setFilterType('All Time')
        setShowDateFilter(false)
        setSearching(false)
        return
      default:
        return
    }

    // Format date to YYYY-MM-DD for date input
    const formattedDate = startDate.toISOString().split('T')[0]
    setFilterDate(formattedDate)
    setFilterType(label)
    setSearching(true)
    setShowDateFilter(false)
  }

  const handleClearFilters = () => {
    setSearchPostTerm('')
    setFilterDate('')
    setFilterType('All Time')
    setShowDateFilter(false)
    fetchFeed()
  }

  const handleSortChange = (sort) => {
    setSortBy(sort)
    setShowSortMenu(false)
    // Re-fetch with new sort
    if (searchPostTerm.trim() || filterDate) {
      // Trigger search with existing filters
      const formEvent = new Event('submit')
      handleSearchPosts(formEvent)
    } else {
      fetchFeed()
    }
  }

  const handleCustomDateApply = () => {
    if (!customDateStart || !customDateEnd) {
      alert('Please select both start and end dates')
      return
    }

    if (new Date(customDateStart) > new Date(customDateEnd)) {
      alert('Start date must be before end date')
      return
    }

    // Set filter to start date and store end date for range filtering
    setFilterDate(customDateStart)
    const startObj = new Date(customDateStart)
    const endObj = new Date(customDateEnd)
    const label = `${startObj.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })} - ${endObj.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })}`
    setFilterType(label)
    
    // Filter posts in range
    setSearching(true)
    setShowCustomDateModal(false)
    setShowDateFilter(false)

    // Store end date in localStorage temporarily for filtering
    sessionStorage.setItem('customDateEnd', customDateEnd)
  }

  const handleCloseCustomModal = () => {
    setShowCustomDateModal(false)
    setCustomDateStart('')
    setCustomDateEnd('')
  }

  const handleToggleLike = async (postId) => {
    try {
      const result = await interactionService.toggleLike(postId)
      setLikedPosts(prev => ({
        ...prev,
        [postId]: result.liked
      }))
      // Update likes_count in posts list without full refresh
      setPosts(prev => prev.map(post =>
        post.id === postId
          ? { ...post, likes_count: result.likes_count }
          : post
      ))
    } catch (err) {
      console.error('Failed to toggle like:', err)
    }
  }

  const handleViewComments = async (postId, postTitle, postUsername) => {
    try {
      const comments = await interactionService.getComments(postId)
      setSelectedCommentPostId(postId)
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
    setSelectedCommentPostId(null)
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
      setPosts(prev => prev.map(post => 
        post.id === postId 
          ? { ...post, comments_count: comments.length }
          : post
      ))
    } catch (err) {
      console.error('Failed to create comment:', err)
      alert('Failed to create comment')
    } finally {
      setSubmittingComments(prev => ({ ...prev, [postId]: false }))
    }
  }

  const handleDeleteComment = async (commentId, postId) => {
    if (!window.confirm('Delete this comment?')) return

    try {
      await interactionService.deleteComment(commentId)
      // Refresh comments in panel
      const comments = await interactionService.getComments(postId)
      setCommentsPanelData(prev => ({
        ...prev,
        comments
      }))
      // Update comments_count in posts list
      setPosts(prev => prev.map(post => 
        post.id === postId 
          ? { ...post, comments_count: comments.length }
          : post
      ))
    } catch (err) {
      console.error('Failed to delete comment:', err)
      alert('Failed to delete comment')
    }
  }

  if (!isAuthenticated) {
    // This should not be reached due to LandingPage redirect, but keeping as fallback
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
      <div className="flex gap-8">
        {/* Left Side - Main Content */}
        <div className="flex-1">
        {/* Header with Create Post Button */}
        <div className="mb-8 flex items-center justify-between">
          <div>
            <h1 className="text-4xl font-bold text-black mb-2">Feed</h1>
            <p className="text-lg text-black">Welcome, {user?.username || 'User'}</p>
          </div>
          <Link to="/posts" className="shrink-0">
            <button className="bg-black hover:bg-white border-2 border-black text-white hover:text-black font-bold py-3 px-4 transition text-2xl rounded-full" title="Create new post">
              +
            </button>
          </Link>
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
                className="flex-1 border border-black p-2 bg-white text-black font-bold text-sm"
              />
              <button
                type="submit"
                className="bg-black border border-black text-white font-bold py-2 px-6 hover:bg-white hover:text-black transition text-sm"
              >
                Search
              </button>

              {filterDate && (
                <button
                  type="button"
                  onClick={handleClearFilters}
                  className="border border-black bg-white text-black font-bold py-2 px-3 hover:bg-black hover:text-white transition text-sm"
                >
                  Reset
                </button>
              )}
            </div>

            {/* Date Filter & Sort Options - Same Row */}
            <div className="flex gap-2 items-center">
              {/* Date Filter - Combined Box */}
              <div className="relative">
                <button
                  type="button"
                  onClick={() => setShowDateFilter(!showDateFilter)}
                  className="bg-white text-black font-bold text-sm px-3 py-2 hover:bg-black hover:text-white transition flex items-center gap-2"
                >
                  <span>{filterType}</span>
                  <span>▼</span>
                </button>

                {showDateFilter && (
                  <div className="absolute top-full left-0 mt-1 bg-white z-50 w-40 shadow-lg">
                    <button
                      type="button"
                      onClick={() => handleClearFilters()}
                      className="w-full text-left px-3 py-2 text-black font-bold hover:bg-gray-100 text-xs border-b border-gray-200"
                    >
                      All Time
                    </button>
                    <button
                      type="button"
                      onClick={() => handleDateRangeFilter('today')}
                      className="w-full text-left px-3 py-2 text-black font-bold hover:bg-gray-100 text-xs border-b border-gray-200"
                    >
                      Today
                    </button>
                    <button
                      type="button"
                      onClick={() => handleDateRangeFilter('week')}
                      className="w-full text-left px-3 py-2 text-black font-bold hover:bg-gray-100 text-xs border-b border-gray-200"
                    >
                      Last 7 Days
                    </button>
                    <button
                      type="button"
                      onClick={() => handleDateRangeFilter('month')}
                      className="w-full text-left px-3 py-2 text-black font-bold hover:bg-gray-100 text-xs border-b border-gray-200"
                    >
                      Last Month
                    </button>
                    <button
                      type="button"
                      onClick={() => handleDateRangeFilter('year')}
                      className="w-full text-left px-3 py-2 text-black font-bold hover:bg-gray-100 text-xs border-b border-gray-200"
                    >
                      Last Year
                    </button>
                    <button
                      type="button"
                      onClick={() => {
                        setShowDateFilter(false)
                        setShowCustomDateModal(true)
                      }}
                      className="w-full text-left px-3 py-2 text-blue-600 font-bold hover:bg-gray-100 text-xs"
                    >
                      Custom Date
                    </button>
                  </div>
                )}
              </div>

              {/* Sort Options - Right Side */}
              <div className="relative">
                <button
                  type="button"
                  onClick={() => setShowSortMenu(!showSortMenu)}
                  className="bg-white text-black font-bold py-2 px-3 hover:bg-black hover:text-white transition text-sm flex items-center gap-1"
                >
                  {sortBy === 'newest' ? 'Newest' : 'Oldest'} ▼
                </button>

                {showSortMenu && (
                  <div className="absolute top-full right-0 mt-1 bg-white z-50 w-32 shadow-lg">
                    <button
                      type="button"
                      onClick={() => handleSortChange('newest')}
                      className={`w-full text-left px-3 py-2 font-bold hover:bg-gray-100 text-xs border-b border-gray-200 ${sortBy === 'newest' ? 'bg-black text-white' : 'text-black'}`}
                    >
                      Newest
                    </button>
                    <button
                      type="button"
                      onClick={() => handleSortChange('oldest')}
                      className={`w-full text-left px-3 py-2 font-bold hover:bg-gray-100 text-xs ${sortBy === 'oldest' ? 'bg-black text-white' : 'text-black'}`}
                    >
                      Oldest
                    </button>
                  </div>
                )}
              </div>
            </div>
          </form>

          {/* Custom Date Modal */}
          {showCustomDateModal && (
            <div className="fixed inset-0 z-50 flex items-center justify-center pointer-events-none">
              <div className="bg-white border-4 border-black p-6 w-96 pointer-events-auto">
                <h2 className="text-xl font-bold text-black mb-4">Select Date Range</h2>
                
                <div className="space-y-4">
                  <div>
                    <label className="block text-black font-bold mb-2 text-xs">Start Date</label>
                    <input
                      type="date"
                      value={customDateStart}
                      onChange={(e) => setCustomDateStart(e.target.value)}
                      className="w-full border-2 border-black p-2 bg-white text-black font-bold text-sm"
                    />
                  </div>

                  <div>
                    <label className="block text-black font-bold mb-2 text-xs">End Date</label>
                    <input
                      type="date"
                      value={customDateEnd}
                      onChange={(e) => setCustomDateEnd(e.target.value)}
                      className="w-full border-2 border-black p-2 bg-white text-black font-bold text-sm"
                    />
                  </div>
                </div>

                <div className="flex gap-2 mt-4">
                  <button
                    type="button"
                    onClick={handleCustomDateApply}
                    className="flex-1 bg-black border-2 border-black text-white font-bold py-2 px-4 hover:bg-white hover:text-black transition text-sm"
                  >
                    Apply
                  </button>
                  <button
                    type="button"
                    onClick={handleCloseCustomModal}
                    className="flex-1 bg-white border-2 border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition text-sm"
                  >
                    Cancel
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* Spacing */}
          <div className="mb-4" />

          {/* Admin Management Button */}
          {user?.role === 'admin' && (
            <div className="mb-4">
              <Link to="/users" className="block">
                <button className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition">
                  Users Management
                </button>
              </Link>
            </div>
          )}
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
              <div 
                key={post.id} 
                className={`border border-black p-6 transition-all ${
                  commentsPanelData && commentsPanelData.postId === post.id
                    ? 'bg-yellow-100 border-2 border-black'
                    : commentsPanelData
                    ? 'bg-gray-100 opacity-50'
                    : 'bg-white'
                }`}
              >
                <h3 className="text-xl font-bold text-black mb-2">{post.title}</h3>
                <p className="text-black mb-4 line-clamp-3">{post.content}</p>
                <div className="text-xs text-black mb-4 font-bold">
                  <Link to={`/user/${post.user_id}`} className="hover:underline text-black">
                    By {post.username}
                  </Link>
                  <p>Posted: {new Date(post.created_at).toLocaleDateString()}</p>
                </div>

                {/* Like and Comment Stats */}
                <div className="flex gap-4 mb-4 text-sm">
                  <span className="text-black font-bold">❤️ {post.likes_count || 0} Likes</span>
                  <span className="text-black font-bold">💬 {post.comments_count || 0} Comments</span>
                </div>

                {/* Like and Comment Buttons */}
                <div className="flex gap-2 mb-4">
                  <button
                    onClick={() => handleToggleLike(post.id)}
                    className="bg-black hover:bg-gray-800 text-white border border-black font-bold py-1 px-3 text-sm transition"
                  >
                    {likedPosts[post.id] ? '❤️ Liked' : '🤍 Like'}
                  </button>
                  <button
                    onClick={() => handleViewComments(post.id, post.title, post.username)}
                    className="bg-white hover:bg-gray-100 text-black border border-black font-bold py-1 px-3 text-sm transition"
                  >
                    💬 View Comments
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
        </div>

      {/* Right Side - Comments Panel */}
      {commentsPanelData && (
        <div className="w-96">
          <div className="sticky top-32 border-4 border-black bg-white p-4 h-fit">
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
                  <div key={comment.id} className="mb-3 pb-3 border-b border-gray-300 last:border-b-0 group">
                    <div className="flex items-start justify-between gap-2">
                      <p className="font-bold text-xs text-black">{comment.username}</p>
                      {comment.user_id === user?.id && (
                        <button
                          onClick={() => handleDeleteComment(comment.id, commentsPanelData.postId)}
                          className="text-red-600 hover:text-red-800 font-bold text-xs whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity"
                          title="Delete comment"
                        >
                          Delete
                        </button>
                      )}
                    </div>
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
    </div>
  )
}

export default HomePage