import { Link, useLocation, useNavigate } from 'react-router-dom'
import { useAuth } from '../../context/AuthContext'
import { useState } from 'react'

const Header = () => {
  const location = useLocation()
  const navigate = useNavigate()
  const { isAuthenticated, user, logout } = useAuth()
  const [showLogoutConfirm, setShowLogoutConfirm] = useState(false)
  const [showUserMenu, setShowUserMenu] = useState(false)
  const [searchUsername, setSearchUsername] = useState('')

  const handleLogoutClick = () => {
    setShowUserMenu(false)
    setShowLogoutConfirm(true)
  }

  const confirmLogout = async () => {
    setShowLogoutConfirm(false)
    await logout()
    navigate('/login')
  }

  const cancelLogout = () => {
    setShowLogoutConfirm(false)
  }

  const handleSearchUsername = (e) => {
    e.preventDefault()
    if (searchUsername.trim()) {
      navigate(`/users?search=${encodeURIComponent(searchUsername)}`)
      setSearchUsername('')
    }
  }

  return (
    <header className="fixed top-0 left-0 right-0 bg-white border-b border-black z-50">
      <div className="container mx-auto px-4">
        {/* Header Top */}
        <div className="flex justify-between items-center py-4">
          <div className="flex items-center gap-3">
            <div className="bg-black border border-black p-2">
              <img src="/ush.webp" alt="UTY software house" className="h-10 w-10" />
            </div>
            <div>
              <h1 className="text-2xl font-bold text-black">UTY software house</h1>
              <p className="text-sm text-black">QA skill test</p>
            </div>
          </div>
          
          {isAuthenticated && (
            <form onSubmit={handleSearchUsername} className="flex gap-2">
              <input
                type="text"
                value={searchUsername}
                onChange={(e) => setSearchUsername(e.target.value)}
                placeholder="Find user..."
                className="border border-black p-2 bg-white text-black font-bold w-56"
              />
              <button
                type="submit"
                className="border border-black bg-white text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition"
              >
                Go
              </button>
            </form>
          )}
          
          <nav className="flex gap-2 items-center">
            {isAuthenticated ? (
              <>
                <Link to="/">
                  <button className={`px-4 py-2 border border-black font-bold ${location.pathname === '/' ? 'bg-black text-white' : 'bg-white text-black hover:bg-black hover:text-white'} transition`}>
                    Home
                  </button>
                </Link>
                {user?.role === 'admin' && (
                  <Link to="/users">
                    <button className={`px-4 py-2 border border-black font-bold ${location.pathname === '/users' ? 'bg-black text-white' : 'bg-white text-black hover:bg-black hover:text-white'} transition`}>
                      Users
                    </button>
                  </Link>
                )}
                <Link to="/posts">
                  <button className={`px-4 py-2 border border-black font-bold ${location.pathname === '/posts' ? 'bg-black text-white' : 'bg-white text-black hover:bg-black hover:text-white'} transition`}>
                    Posts
                  </button>
                </Link>
                <div className="border-l border-black pl-2 ml-2 relative">
                  <button
                    onClick={() => setShowUserMenu(!showUserMenu)}
                    className="px-4 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition flex items-center gap-2"
                  >
                    <span>{user?.username || 'User'}</span>
                    <span className="text-xs">▼</span>
                  </button>
                  
                  {showUserMenu && (
                    <div className="absolute right-0 mt-1 w-48 bg-white border-2 border-black shadow-lg z-50">
                      <div className="p-4 border-b border-black">
                        <p className="text-xs text-gray-600">Logged in as</p>
                        <p className="font-bold text-black">{user?.username || 'User'}</p>
                        {user?.email && <p className="text-xs text-gray-600">{user.email}</p>}
                      </div>
                      <Link to="/profile" onClick={() => setShowUserMenu(false)}>
                        <button className="w-full text-left px-4 py-3 border-t border-black bg-white text-black font-bold hover:bg-black hover:text-white transition">
                          Profile Settings
                        </button>
                      </Link>
                      <button
                        onClick={handleLogoutClick}
                        className="w-full text-left px-4 py-3 border-t border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
                      >
                        Logout
                      </button>
                    </div>
                  )}
                </div>
              </>
            ) : (
              <>
                <Link to="/login">
                  <button className={`px-4 py-2 border border-black font-bold ${location.pathname === '/login' ? 'bg-black text-white' : 'bg-white text-black hover:bg-black hover:text-white'} transition`}>
                    Login
                  </button>
                </Link>
                <Link to="/register">
                  <button className={`px-4 py-2 border border-black font-bold ${location.pathname === '/register' ? 'bg-black text-white' : 'bg-white text-black hover:bg-black hover:text-white'} transition`}>
                    Register
                  </button>
                </Link>
              </>
            )}
          </nav>
        </div>
      </div>

      {showLogoutConfirm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white border-2 border-black p-8 rounded-lg shadow-lg max-w-sm w-full mx-4">
            <h2 className="text-2xl font-bold text-black mb-4">Confirm Logout</h2>
            <p className="text-black mb-6">Are you sure you want to logout?</p>
            <div className="flex gap-4 justify-end">
              <button
                onClick={cancelLogout}
                className="px-6 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition"
              >
                Cancel
              </button>
              <button
                onClick={confirmLogout}
                className="px-6 py-2 border border-black bg-black text-white font-bold hover:bg-white hover:text-black transition"
              >
                Logout
              </button>
            </div>
          </div>
        </div>
      )}
    </header>
  )
}

export default Header