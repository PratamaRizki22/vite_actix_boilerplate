import React from 'react';
import { Navigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';

const ProtectedRoute = ({ children, requiredRole = null }) => {
  const { user, loading } = useAuth();

  if (loading) {
    return (
      <div className="min-h-screen bg-white flex items-center justify-center p-4">
        <div className="text-center">
          <p className="text-black font-bold">Loading...</p>
        </div>
      </div>
    );
  }

  // If not authenticated at all, redirect to login
  if (!user) {
    return <Navigate to="/login" replace />;
  }

  // If role is required and doesn't match, show access denied
  if (requiredRole && user.role !== requiredRole) {
    return (
      <div className="min-h-screen bg-white flex items-center justify-center p-4">
        <div className="text-center border border-black p-8">
          <h1 className="text-2xl font-bold text-black mb-4">Access Denied</h1>
          <p className="text-black mb-4">You do not have permission to access this page.</p>
          <p className="text-black text-sm mb-6">This page requires {requiredRole} role.</p>
          <a href="/" className="px-6 py-2 border border-black bg-white text-black font-bold hover:bg-black hover:text-white transition inline-block">
            Go to Home
          </a>
        </div>
      </div>
    );
  }

  return children;
};

export default ProtectedRoute;
