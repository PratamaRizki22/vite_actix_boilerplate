import { useState } from 'react'
import UserList from '../components/users/UserList'
import UserForm from '../components/users/UserForm'

const UsersPage = () => {
  const [showForm, setShowForm] = useState(false)

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
        <h2>User Management</h2>
        <button 
          className="btn btn-primary"
          onClick={() => setShowForm(!showForm)}
        >
          {showForm ? 'Cancel' : 'Add New User'}
        </button>
      </div>

      {showForm && (
        <div className="card">
          <UserForm onSuccess={() => setShowForm(false)} />
        </div>
      )}

      <UserList />
    </div>
  )
}

export default UsersPage