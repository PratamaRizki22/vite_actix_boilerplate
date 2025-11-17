const Card = ({ children, className = '' }) => {
  return (
    <div className={`bg-white rounded-lg shadow-md p-6 mb-4 ${className}`}>
      {children}
    </div>
  )
}

export default Card