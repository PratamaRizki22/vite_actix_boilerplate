import { Link, useLocation } from 'react-router-dom'
import Button from '../common/Button'

const Header = () => {
  const location = useLocation()

  return (
    <header className="bg-white shadow-sm border-b">
      <div className="container mx-auto px-4">
        <div className="flex justify-between items-center py-4">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">Wallet Core</h1>
            <p className="text-sm text-gray-600">Manage your crypto wallet</p>
          </div>
          
          <nav className="flex gap-2">
            <Link to="/">
              <Button 
                variant={location.pathname === '/' ? 'primary' : 'secondary'}
                size="sm"
              >
                Home
              </Button>
            </Link>
            <Link to="/users">
              <Button 
                variant={location.pathname === '/users' ? 'primary' : 'secondary'}
                size="sm"
              >
                Users
              </Button>
            </Link>
            <Link to="/posts">
              <Button 
                variant={location.pathname === '/posts' ? 'primary' : 'secondary'}
                size="sm"
              >
                Posts
              </Button>
            </Link>
          </nav>
        </div>
      </div>
    </header>
  )
}

export default Header