import { Link } from 'react-router-dom'
import Button from '../components/common/Button'
import Card from '../components/common/Card'

const NotFoundPage = () => {
  return (
    <Card className="text-center">
      <h2 className="text-2xl font-bold mb-4">404 - Page Not Found</h2>
      <p className="text-gray-600 mb-6">The page you're looking for doesn't exist.</p>
      <Link to="/">
        <Button variant="primary">
          Go Back Home
        </Button>
      </Link>
    </Card>
  )
}

export default NotFoundPage