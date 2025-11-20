import Header from './Header'

const Layout = ({ children, className = '' }) => {
  return (
    <div className="min-h-screen bg-white">
      <Header />
      <main className={`container mx-auto px-4 py-8 pt-24 ${className}`}>
        {children}
      </main>
    </div>
  )
}

export default Layout