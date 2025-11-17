import { Link } from 'react-router-dom'
import Button from '../components/common/Button'
import Card from '../components/common/Card'

const HomePage = () => {
  return (
    <Card>
      <h2 className="text-2xl font-bold mb-4">Welcome to Wallet Core App</h2>
      <p className="text-gray-600 mb-6">This is a boilerplate with React Router setup for future scalability.</p>
      
      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-3">Quick Links:</h3>
        <div className="flex gap-3">
          <Link to="/users">
            <Button variant="primary">
              Manage Users
            </Button>
          </Link>
          <Link to="/posts">
            <Button variant="primary">
              Manage Posts
            </Button>
          </Link>
        </div>
      </div>

      <div>
        <h4 className="text-md font-semibold mb-2">Features:</h4>
        <ul className="list-disc list-inside space-y-1 text-gray-700">
          <li>React Router for navigation</li>
          <li>Axios for API calls</li>
          <li>Custom hooks for state management</li>
          <li>Modular component structure</li>
          <li>Ready for Context API when needed</li>
        </ul>
      </div>
    </Card>
  )
}

export default HomePage