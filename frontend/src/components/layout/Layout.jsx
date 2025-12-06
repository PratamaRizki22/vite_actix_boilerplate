import Header from './Header'
import BannedBanner from '../BannedBanner'

const Layout = ({ children, className = '' }) => {
  return (
    <div className="min-h-screen bg-white">
      <Header />
      <BannedBanner />
      <main className={`container mx-auto px-4 py-8 pt-24 ${className}`}>
        {children}
      </main>
    </div>
  )
}

export default Layout