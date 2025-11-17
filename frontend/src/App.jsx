import { Outlet, Link, useLocation } from 'react-router-dom'
import './styles/index.css'

function App() {
  const location = useLocation()

  return (
    <div className="container">
      <header style={{ padding: '20px 0', borderBottom: '1px solid #ddd', marginBottom: '30px' }}>
        <h1>Wallet Core App</h1>
        <nav style={{ marginTop: '10px' }}>
          <Link 
            to="/"
            className={`btn ${location.pathname === '/' ? 'btn-primary' : ''}`}
            style={{ marginRight: '10px', textDecoration: 'none' }}
          >
            Home
          </Link>
          <Link 
            to="/users"
            className={`btn ${location.pathname === '/users' ? 'btn-primary' : ''}`}
            style={{ marginRight: '10px', textDecoration: 'none' }}
          >
            Users
          </Link>
          <Link 
            to="/posts"
            className={`btn ${location.pathname === '/posts' ? 'btn-primary' : ''}`}
            style={{ textDecoration: 'none' }}
          >
            Posts
          </Link>
        </nav>
      </header>

      <main>
        <Outlet />
      </main>
    </div>
  )
}

export default App