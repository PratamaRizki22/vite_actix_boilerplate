import { useState } from 'react'
import PostList from '../components/lists/PostList'
import PostForm from '../components/forms/PostForm'
import Button from '../components/common/Button'
import Card from '../components/common/Card'
import PageContainer from '../components/layout/PageContainer'

const PostsPage = () => {
  const [showForm, setShowForm] = useState(false)

  return (
    <PageContainer
      title="Post Management"
      action={
        <Button 
          variant={showForm ? 'secondary' : 'primary'}
          onClick={() => setShowForm(!showForm)}
        >
          {showForm ? 'Cancel' : 'Create New Post'}
        </Button>
      }
    >
      {showForm && (
        <Card>
          <PostForm onSuccess={() => setShowForm(false)} />
        </Card>
      )}

      <PostList />
    </PageContainer>
  )
}

export default PostsPage