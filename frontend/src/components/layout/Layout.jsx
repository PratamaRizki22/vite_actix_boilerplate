// src/components/layout/Layout.jsx
import Header from './Header'

const Layout = ({ children, className = '' }) => {
  return (
    <div className="min-h-screen bg-gray-50">
      <Header />
      <main className={`container mx-auto px-4 py-8 ${className}`}>
        {children}
      </main>
    </div>
  )
}

export default Layout