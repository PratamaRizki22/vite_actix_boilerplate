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
  const [showBanModal, setShowBanModal] = useState(false)
  const [banUserId, setBanUserId] = useState(null)
  const [banDays, setBanDays] = useState(1)
  const [showDeleteModal, setShowDeleteModal] = useState(false)
  const [deleteUserId, setDeleteUserId] = useState(null)
  const [selectedUsers, setSelectedUsers] = useState([])
  const [filterRole, setFilterRole] = useState('all')
  const [filterBanned, setFilterBanned] = useState('all')
  const [filterSearch, setFilterSearch] = useState('')
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

  const handleDelete = async (id, userRole) => {
    // Prevent deleting admins
    if (userRole === 'admin') {
      setError('Cannot delete admin users')
      return
    }

    // Show modal for confirmation
    setDeleteUserId(id)
    setShowDeleteModal(true)
  }

  const confirmDelete = async () => {
    try {
      setError('')
      await userService.deleteUser(deleteUserId)
      setShowDeleteModal(false)
      setDeleteUserId(null)
      fetchUsers()
    } catch (err) {
      console.error('Delete error:', err)
      setError(err.response?.data?.error || 'Failed to delete user')
    }
  }

  const toggleUserSelection = (userId, userRole) => {
    // Prevent selecting admins
    if (userRole === 'admin') return

    setSelectedUsers(prev =>
      prev.includes(userId)
        ? prev.filter(id => id !== userId)
        : [...prev, userId]
    )
  }

  const toggleSelectAll = () => {
    const selectableUsers = filteredUsers.filter(u => u.role !== 'admin')
    if (selectedUsers.length === selectableUsers.length && selectableUsers.length > 0) {
      setSelectedUsers([])
    } else {
      setSelectedUsers(selectableUsers.map(u => u.id))
    }
  }

  const handleBulkDelete = () => {
    if (selectedUsers.length === 0) {
      setError('Please select users to delete')
      return
    }
    setShowDeleteModal(true)
  }

  const confirmBulkDelete = async () => {
    try {
      setError('')
      // Delete all selected users
      await Promise.all(selectedUsers.map(id => userService.deleteUser(id)))
      setShowDeleteModal(false)
      setSelectedUsers([])
      fetchUsers()
    } catch (err) {
      console.error('Bulk delete error:', err)
      setError(err.response?.data?.error || 'Failed to delete users')
    }
  }

  // Filter users based on filters
  const filteredUsers = users.filter(user => {
    // Filter by role
    if (filterRole !== 'all' && user.role !== filterRole) return false

    // Filter by banned status
    if (filterBanned === 'banned' && !user.is_banned) return false
    if (filterBanned === 'active' && user.is_banned) return false

    // Filter by search
    if (filterSearch) {
      const search = filterSearch.toLowerCase()
      const matchUsername = user.username?.toLowerCase().includes(search)
      const matchEmail = user.email?.toLowerCase().includes(search)
      if (!matchUsername && !matchEmail) return false
    }

    return true
  })

  const handleBanToggle = async (id, currentBanStatus, userRole) => {
    // Prevent banning admins
    if (userRole === 'admin') {
      setError('Cannot ban admin users')
      return
    }

    if (currentBanStatus) {
      // Unban directly
      try {
        setError('')
        await userService.banUser(id, false, null)
        fetchUsers()
      } catch (err) {
        console.error('Unban error:', err)
        setError(err.response?.data?.error || 'Failed to unban user')
      }
    } else {
      // Show modal to select ban duration
      setBanUserId(id)
      setShowBanModal(true)
    }
  }

  const confirmBan = async () => {
    try {
      setError('')
      await userService.banUser(banUserId, true, banDays)
      setShowBanModal(false)
      setBanUserId(null)
      setBanDays(1)
      fetchUsers()
    } catch (err) {
      console.error('Ban error:', err)
      setError(err.response?.data?.error || 'Failed to ban user')
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
          <>
            {/* Filters */}
            <div className="mb-6 grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <label className="block text-black font-bold mb-2">Filter by Role:</label>
                <select
                  value={filterRole}
                  onChange={(e) => setFilterRole(e.target.value)}
                  className="w-full border-2 border-black p-2 bg-white text-black font-bold"
                >
                  <option value="all">All Roles</option>
                  <option value="admin">Admin</option>
                  <option value="user">User</option>
                </select>
              </div>

              <div>
                <label className="block text-black font-bold mb-2">Filter by Status:</label>
                <select
                  value={filterBanned}
                  onChange={(e) => setFilterBanned(e.target.value)}
                  className="w-full border-2 border-black p-2 bg-white text-black font-bold"
                >
                  <option value="all">All Status</option>
                  <option value="active">Active</option>
                  <option value="banned">Banned</option>
                </select>
              </div>

              <div>
                <label className="block text-black font-bold mb-2">Search:</label>
                <input
                  type="text"
                  value={filterSearch}
                  onChange={(e) => setFilterSearch(e.target.value)}
                  placeholder="Search by username or email..."
                  className="w-full border-2 border-black p-2 bg-white text-black"
                />
              </div>
            </div>

            <div className="mb-4 text-black font-bold">
              Showing {filteredUsers.length} of {users.length} users
            </div>

            {/* Bulk Delete Button */}
            {selectedUsers.length > 0 && (
              <div className="mb-4 flex items-center gap-4">
                <button
                  onClick={handleBulkDelete}
                  className="bg-red-500 border-2 border-black text-white font-bold px-4 py-2 hover:bg-red-600 transition"
                >
                  Delete Selected ({selectedUsers.length})
                </button>
                <button
                  onClick={() => setSelectedUsers([])}
                  className="bg-white border-2 border-black text-black font-bold px-4 py-2 hover:bg-black hover:text-white transition"
                >
                  Clear Selection
                </button>
              </div>
            )}

            <div className="overflow-x-auto">
              <table className="w-full border-collapse border border-black bg-white">
                <thead className="bg-black text-white">
                  <tr className="border-b border-black">
                    <th className="text-left p-2 text-white font-bold">
                      <input
                        type="checkbox"
                        checked={selectedUsers.length === filteredUsers.filter(u => u.role !== 'admin').length && filteredUsers.length > 0}
                        onChange={toggleSelectAll}
                        className="w-4 h-4 cursor-pointer"
                      />
                    </th>
                    <th className="text-left p-2 text-white font-bold">ID</th>
                    <th className="text-left p-2 text-white font-bold">Username</th>
                    <th className="text-left p-2 text-white font-bold">Email</th>
                    <th className="text-left p-2 text-white font-bold">Role</th>
                    <th className="text-left p-2 text-white font-bold">Last Active</th>
                    <th className="text-left p-2 text-white font-bold">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {filteredUsers.map((user) => (
                    <tr key={user.id} className="border-b border-black">
                      <td className="p-2">
                        <input
                          type="checkbox"
                          checked={selectedUsers.includes(user.id)}
                          onChange={() => toggleUserSelection(user.id, user.role)}
                          disabled={user.role === 'admin'}
                          className={`w-4 h-4 ${user.role === 'admin' ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'}`}
                        />
                      </td>
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
                      <td className="p-2 text-black">
                        {user.last_login ? new Date(user.last_login).toLocaleString() : 'Never'}
                      </td>
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
                              onClick={() => handleBanToggle(user.id, user.is_banned, user.role)}
                              disabled={user.role === 'admin'}
                              className={`border border-black font-bold px-3 py-1 transition ${user.role === 'admin'
                                ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
                                : user.is_banned
                                  ? 'bg-green-500 text-white hover:bg-green-600'
                                  : 'bg-yellow-500 text-white hover:bg-yellow-600'
                                }`}
                            >
                              {user.is_banned ? 'Unban' : 'Ban'}
                            </button>
                            <button
                              onClick={() => handleDelete(user.id, user.role)}
                              disabled={user.role === 'admin'}
                              className={`border border-black font-bold px-3 py-1 transition ${user.role === 'admin'
                                ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
                                : 'bg-red-500 text-white hover:bg-red-600'
                                }`}
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
          </>
        )}

        {/* Pagination */}
        {
          pagination && (
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
                    className={`border border-black px-3 py-2 font-bold transition ${page === currentPage
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
          )
        }
      </div >

      {/* Ban Duration Modal */}
      {
        showBanModal && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white border-4 border-black p-8 max-w-md w-full">
              <h2 className="text-2xl font-bold text-black mb-4">Select Ban Duration</h2>

              <div className="mb-6">
                <label className="block text-black font-bold mb-2">Ban Duration:</label>
                <select
                  value={banDays}
                  onChange={(e) => setBanDays(e.target.value === 'permanent' ? null : parseInt(e.target.value))}
                  className="w-full border-2 border-black p-2 bg-white text-black font-bold"
                >
                  <option value="1">1 Day</option>
                  <option value="2">2 Days</option>
                  <option value="7">7 Days (1 Week)</option>
                  <option value="30">30 Days (1 Month)</option>
                  <option value="permanent">Permanent</option>
                </select>
              </div>

              <div className="flex gap-4">
                <button
                  onClick={confirmBan}
                  className="flex-1 bg-red-500 border-2 border-black text-white font-bold px-4 py-2 hover:bg-red-600 transition"
                >
                  Confirm Ban
                </button>
                <button
                  onClick={() => {
                    setShowBanModal(false)
                    setBanUserId(null)
                    setBanDays(1)
                  }}
                  className="flex-1 bg-white border-2 border-black text-black font-bold px-4 py-2 hover:bg-black hover:text-white transition"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        )
      }

      {/* Delete Confirmation Modal */}
      {showDeleteModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white border-4 border-black p-8 max-w-md w-full">
            <h2 className="text-2xl font-bold text-black mb-4">⚠️ Confirm Delete</h2>

            <p className="text-black font-bold mb-6">
              {selectedUsers.length > 0
                ? `Are you sure you want to delete ${selectedUsers.length} user(s)? This action cannot be undone.`
                : 'Are you sure you want to delete this user? This action cannot be undone.'
              }
            </p>

            <div className="flex gap-4">
              <button
                onClick={selectedUsers.length > 0 ? confirmBulkDelete : confirmDelete}
                className="flex-1 bg-red-500 border-2 border-black text-white font-bold px-4 py-2 hover:bg-red-600 transition"
              >
                Delete {selectedUsers.length > 0 ? `${selectedUsers.length} Users` : 'User'}
              </button>
              <button
                onClick={() => {
                  setShowDeleteModal(false)
                  setDeleteUserId(null)
                }}
                className="flex-1 bg-white border-2 border-black text-black font-bold px-4 py-2 hover:bg-black hover:text-white transition"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div >
  )
}

export default UsersPage