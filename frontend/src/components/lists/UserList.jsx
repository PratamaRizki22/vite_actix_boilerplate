import { useUsers } from '../../hooks/useUsers'
import Button from '../common/Button'
import Card from '../common/Card'

const UserList = () => {
  const { users, loading, error, deleteUser } = useUsers()

  if (loading) return <Card>Loading users...</Card>
  if (error) return <Card className="text-red-500">Error: {error}</Card>

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
        <Card>No users found</Card>
      ) : (
        users.map(user => (
          <Card key={user.id} className="flex justify-between items-center">
            <div>
              <h3 className="text-lg font-semibold">{user.username}</h3>
              <p className="text-gray-600">Email: {user.email}</p>
              <small className="text-gray-500">ID: {user.id}</small>
            </div>
            <Button 
              variant="danger"
              onClick={() => handleDelete(user.id)}
            >
              Delete
            </Button>
          </Card>
        ))
      )}
    </div>
  )
}

export default UserList