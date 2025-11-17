import { useState } from 'react'
import UserList from '../components/lists/UserList'
import UserForm from '../components/forms/UserForm'
import Button from '../components/common/Button'
import Card from '../components/common/Card'
import PageContainer from '../components/layout/PageContainer'

const UsersPage = () => {
  const [showForm, setShowForm] = useState(false)

  return (
    <PageContainer
      title="User Management"
      action={
        <Button 
          variant={showForm ? 'secondary' : 'primary'}
          onClick={() => setShowForm(!showForm)}
        >
          {showForm ? 'Cancel' : 'Add New User'}
        </Button>
      }
    >
      {showForm && (
        <Card>
          <UserForm onSuccess={() => setShowForm(false)} />
        </Card>
      )}

      <UserList />
    </PageContainer>
  )
}

export default UsersPage