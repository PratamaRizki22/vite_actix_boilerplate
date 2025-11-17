import { useUsers } from '../../hooks/useUsers'

const UserList = () => {
  const { users, loading, error, deleteUser } = useUsers()

  if (loading) return <div className="card">Loading users...</div>
  if (error) return <div className="card" style={{ color: 'red' }}>Error: {error}</div>

  const handleDelete = async (id) => {
    if (window.confirm('Are you sure you want to delete this user?')) {
      try {
        await deleteUser(id)
      } catch (err) {
        console.error('Failed to delete user:', err)
      }
    }
  }

  return (
    <div>
      {users.length === 0 ? (
        <div className="card">No users found</div>
      ) : (
        users.map(user => (
          <div key={user.id} className="card" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <div>
              <h3>{user.username}</h3>
              <p>Email: {user.email}</p>
              <small>ID: {user.id}</small>
            </div>
            <button 
              className="btn btn-danger"
              onClick={() => handleDelete(user.id)}
            >
              Delete
            </button>
          </div>
        ))
      )}
    </div>
  )
}

export default UserList