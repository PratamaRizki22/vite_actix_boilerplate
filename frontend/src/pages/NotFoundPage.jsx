import { Link } from 'react-router-dom'

const NotFoundPage = () => {
  return (
    <div className="card" style={{ textAlign: 'center' }}>
      <h2>404 - Page Not Found</h2>
      <p>The page you're looking for doesn't exist.</p>
      <Link to="/" className="btn btn-primary" style={{ textDecoration: 'none' }}>
        Go Back Home
      </Link>
    </div>
  )
}

export default NotFoundPage