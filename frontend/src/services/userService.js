import api from './api';

const userService = {
  // Get all users (admin only)
  getAllUsers: async () => {
    const response = await api.get('/users');
    return response.data;
  },

  // Search users by username - public search (like Instagram)
  searchUsersPublic: async (searchTerm) => {
    const response = await api.get('/users/search-public', {
      params: {
        search: searchTerm
      }
    });
    return response.data;
  },

  // Search users by username with pagination (admin only)
  searchUsers: async (searchTerm, page = 1) => {
    const response = await api.get('/users', {
      params: {
        search: searchTerm,
        page: page
      }
    });
    return response.data;
  },

  // Get single user by ID
  getUserById: async (id) => {
    const response = await api.get(`/users/${id}`);
    return response.data;
  },

  // Update user (only own profile or admin)
  updateUser: async (id, userData) => {
    const response = await api.put(`/users/${id}`, userData);
    return response.data;
  },

  // Delete user
  deleteUser: async (id) => {
    await api.delete(`/users/${id}`);
    return { success: true };
  },

  // Ban/Unban user (admin only)
  banUser: async (id, isBanned, banDays = null) => {
    const response = await api.put(`/users/${id}/ban`, {
      is_banned: isBanned,
      ban_days: banDays
    });
    return response.data;
  },
};

export default userService;