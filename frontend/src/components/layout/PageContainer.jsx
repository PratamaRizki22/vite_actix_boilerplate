// src/components/layout/PageContainer.jsx
const PageContainer = ({ 
  title, 
  action, 
  children,
  className = '' 
}) => {
  return (
    <div className={className}>
      {(title || action) && (
        <div className="flex justify-between items-center mb-6">
          {title && <h1 className="text-2xl font-bold text-gray-900">{title}</h1>}
          {action && <div>{action}</div>}
        </div>
      )}
      {children}
    </div>
  )
}

export default PageContainer