import React from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';

const DashboardPage = () => {
  const navigate = useNavigate();
  const { user } = useAuth();

  return (
    <div className="min-h-screen bg-white">
      <div className="container mx-auto px-4 py-8">
        <h1 className="text-4xl font-bold text-black mb-8">Dashboard</h1>
        
        {user && (
          <div className="border border-black p-6 mb-8 bg-white">
            <h2 className="text-2xl font-bold text-black mb-4">Welcome, {user.username}!</h2>
            <div className="space-y-2 text-black">
              <p><strong>Email:</strong> {user.email}</p>
              <p><strong>Role:</strong> {user.role}</p>
              <p><strong>Account Status:</strong> {user.email_verified ? '✓ Verified' : '⚠ Pending Verification'}</p>
            </div>
          </div>
        )}

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <button
            onClick={() => navigate('/profile')}
            className="border border-black p-6 bg-white text-black font-bold hover:bg-black hover:text-white transition"
          >
            <div className="text-2xl mb-2">Profile</div>
            <div className="text-sm font-normal">View and edit your profile</div>
          </button>

          <button
            onClick={() => navigate('/posts')}
            className="border border-black p-6 bg-white text-black font-bold hover:bg-black hover:text-white transition"
          >
            <div className="text-2xl mb-2">Posts</div>
            <div className="text-sm font-normal">View and manage posts</div>
          </button>

          <button
            onClick={() => navigate('/users')}
            className="border border-black p-6 bg-white text-black font-bold hover:bg-black hover:text-white transition"
          >
            <div className="text-2xl mb-2">Users</div>
            <div className="text-sm font-normal">Browse and search users</div>
          </button>
        </div>
      </div>
    </div>
  );
};

export default DashboardPage;
