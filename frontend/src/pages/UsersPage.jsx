import { useState, useEffect } from 'react'
import { useSearchParams, Link } from 'react-router-dom'
import userService from '../services/userService'
import { useAuth } from '../context/AuthContext'

const UsersPage = () => {
  const [users, setUsers] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [editingId, setEditingId] = useState(null)
  const [editUsername, setEditUsername] = useState('')
  const [editEmail, setEditEmail] = useState('')
  const { isAuthenticated } = useAuth()
  const [searchParams, setSearchParams] = useSearchParams()
  const searchTerm = searchParams.get('search')
  const currentPage = parseInt(searchParams.get('page') || '1', 10)
  const [pagination, setPagination] = useState(null)

  useEffect(() => {
    if (isAuthenticated) {
      fetchUsers(currentPage)
    }
  }, [isAuthenticated, searchTerm, currentPage])

  const fetchUsers = async (page = 1) => {
    try {
      setLoading(true)
      setError('')
      let data
      if (searchTerm) {
        const result = await userService.searchUsers(searchTerm, page)
        data = result.data
        setPagination(result.pagination)
      } else {
        data = await userService.getAllUsers()
        setPagination(null)
      }
      setUsers(data)
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to fetch users')
    } finally {
      setLoading(false)
    }
  }

  const handlePageChange = (newPage) => {
    const params = new URLSearchParams(searchParams)
    params.set('page', newPage.toString())
    setSearchParams(params)
  }

  const startEdit = (user) => {
    setEditingId(user.id)
    setEditUsername(user.username)
    setEditEmail(user.email)
  }

  const cancelEdit = () => {
    setEditingId(null)
    setEditUsername('')
    setEditEmail('')
  }

  const handleUpdate = async (id) => {
    try {
      setError('')
      await userService.updateUser(id, {
        username: editUsername,
        email: editEmail,
      })
      setEditingId(null)
      fetchUsers()
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to update user')
    }
  }

  const handleDelete = async (id) => {
    if (!confirm('Are you sure you want to delete this user?')) return

    try {
      setError('')
      await userService.deleteUser(id)
      fetchUsers()
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to delete user')
    }
  }

  if (!isAuthenticated) {
    return (
      <div className="border border-black p-8 text-center">
        <p className="text-black">Please log in to view users</p>
      </div>
    )
  }

  return (
    <div>
      <div className="border border-black p-8 mb-8">
        <h1 className="text-3xl font-bold text-black mb-4">Users Management</h1>
        
        {error && (
          <div className="border border-black bg-white p-4 mb-6 text-black">
            {error}
          </div>
        )}

        {loading ? (
          <p className="text-black">Loading users...</p>
        ) : users.length === 0 ? (
          <p className="text-black">No users found</p>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full border-collapse">
              <thead>
                <tr className="border-b border-black">
                  <th className="text-left p-2 text-black font-bold">ID</th>
                  <th className="text-left p-2 text-black font-bold">Username</th>
                  <th className="text-left p-2 text-black font-bold">Email</th>
                  <th className="text-left p-2 text-black font-bold">Role</th>
                  <th className="text-left p-2 text-black font-bold">Actions</th>
                </tr>
              </thead>
              <tbody>
                {users.map((user) => (
                  <tr key={user.id} className="border-b border-black">
                    <td className="p-2 text-black">{user.id}</td>
                    <td className="p-2 text-black">
                      {editingId === user.id ? (
                        <input
                          type="text"
                          value={editUsername}
                          onChange={(e) => setEditUsername(e.target.value)}
                          className="border border-black p-1 bg-white text-black w-full"
                        />
                      ) : (
                        user.username
                      )}
                    </td>
                    <td className="p-2 text-black">
                      {editingId === user.id ? (
                        <input
                          type="email"
                          value={editEmail}
                          onChange={(e) => setEditEmail(e.target.value)}
                          className="border border-black p-1 bg-white text-black w-full"
                        />
                      ) : (
                        user.email
                      )}
                    </td>
                    <td className="p-2 text-black">{user.role}</td>
                    <td className="p-2 space-x-2">
                      {editingId === user.id ? (
                        <>
                          <button
                            onClick={() => handleUpdate(user.id)}
                            className="bg-white border border-black text-black font-bold px-3 py-1 hover:bg-black hover:text-white transition"
                          >
                            Save
                          </button>
                          <button
                            onClick={cancelEdit}
                            className="bg-white border border-black text-black font-bold px-3 py-1 hover:bg-black hover:text-white transition"
                          >
                            Cancel
                          </button>
                        </>
                      ) : (
                        <>
                          <Link to={`/user/${user.id}`}>
                            <button
                              className="bg-black border border-black text-white font-bold px-3 py-1 hover:bg-white hover:text-black transition"
                            >
                              View
                            </button>
                          </Link>
                          <button
                            onClick={() => startEdit(user)}
                            className="bg-white border border-black text-black font-bold px-3 py-1 hover:bg-black hover:text-white transition"
                          >
                            Edit
                          </button>
                          <button
                            onClick={() => handleDelete(user.id)}
                            className="bg-white border border-black text-black font-bold px-3 py-1 hover:bg-black hover:text-white transition"
                          >
                            Delete
                          </button>
                        </>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
        
        {/* Pagination */}
        {pagination && (
          <div className="mt-6 flex justify-center items-center gap-2">
            <button
              onClick={() => handlePageChange(currentPage - 1)}
              disabled={currentPage === 1}
              className="border border-black bg-white text-black font-bold px-3 py-2 hover:bg-black hover:text-white transition disabled:opacity-50 disabled:cursor-not-allowed"
            >
              ← Previous
            </button>
            
            <div className="flex gap-1">
              {Array.from({ length: pagination.total_pages }, (_, i) => i + 1).map((page) => (
                <button
                  key={page}
                  onClick={() => handlePageChange(page)}
                  className={`border border-black px-3 py-2 font-bold transition ${
                    page === currentPage
                      ? 'bg-black text-white'
                      : 'bg-white text-black hover:bg-black hover:text-white'
                  }`}
                >
                  {page}
                </button>
              ))}
            </div>
            
            <button
              onClick={() => handlePageChange(currentPage + 1)}
              disabled={currentPage === pagination.total_pages}
              className="border border-black bg-white text-black font-bold px-3 py-2 hover:bg-black hover:text-white transition disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Next →
            </button>
            
            <span className="text-black font-bold ml-4">
              Page {currentPage} of {pagination.total_pages} ({pagination.total} total)
            </span>
          </div>
        )}
      </div>
    </div>
  )
}

export default UsersPage