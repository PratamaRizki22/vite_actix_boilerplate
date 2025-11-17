import { Link } from 'react-router-dom'

const HomePage = () => {
  return (
    <div className="card">
      <h2>Welcome to Wallet Core App</h2>
      <p>This is a boilerplate with React Router setup for future scalability.</p>
      
      <div style={{ marginTop: '20px' }}>
        <h3>Quick Links:</h3>
        <div style={{ display: 'flex', gap: '10px', marginTop: '10px' }}>
          <Link to="/users" className="btn btn-primary" style={{ textDecoration: 'none' }}>
            Manage Users
          </Link>
          <Link to="/posts" className="btn btn-primary" style={{ textDecoration: 'none' }}>
            Manage Posts
          </Link>
        </div>
      </div>

      <div style={{ marginTop: '30px' }}>
        <h4>Features:</h4>
        <ul>
          <li>React Router for navigation</li>
          <li>Axios for API calls</li>
          <li>Custom hooks for state management</li>
          <li>Modular component structure</li>
          <li>Ready for Context API when needed</li>
        </ul>
      </div>
    </div>
  )
}

export default HomePage