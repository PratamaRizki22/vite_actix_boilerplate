import { Link } from 'react-router-dom'

const NotFoundPage = () => {
  return (
    <div className="min-h-[60vh] flex items-center justify-center">
      <div className="border border-black p-8 w-full max-w-md text-center">
        <h1 className="text-4xl font-bold text-black mb-4">404</h1>
        <h2 className="text-2xl font-bold text-black mb-4">Page Not Found</h2>
        <p className="text-black mb-6">The page you're looking for doesn't exist.</p>
        <Link to="/">
          <button className="bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition">
            Go Back Home
          </button>
        </Link>
      </div>
    </div>
  )
}

export default NotFoundPage