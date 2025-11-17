import api from './api';

export const postService = {
  getAll: async () => {
    try {
      const response = await api.get('/api/posts');
      return response.data;
    } catch (error) {
      console.error('Error fetching posts:', error.response?.data || error.message);
      throw error;
    }
  },

  getByUser: async (userId) => {
    try {
      const response = await api.get(`/api/posts/user/${userId}`);
      return response.data;
    } catch (error) {
      console.error(`Error fetching posts for user ${userId}:`, error.response?.data || error.message);
      throw error;
    }
  },

  create: async (postData) => {
    try {
      console.log('Sending post data to backend:', postData);
      const response = await api.post('/api/posts', postData);
      console.log('Post created successfully:', response.data);
      return response.data;
    } catch (error) {
      // Handle plain text error response from backend
      let errorMessage = 'Failed to create post';
      
      if (error.response) {
        // Jika backend mengembalikan plain text
        if (typeof error.response.data === 'string') {
          errorMessage = error.response.data;
        } 
        // Jika backend mengembalikan JSON error
        else if (error.response.data && typeof error.response.data === 'object') {
          errorMessage = error.response.data.message || JSON.stringify(error.response.data);
        }
        // Jika status code tertentu
        else if (error.response.status === 400) {
          errorMessage = 'Bad Request: Invalid data sent to server';
        } else if (error.response.status === 500) {
          errorMessage = 'Server error: Please try again later';
        }
      } else if (error.request) {
        errorMessage = 'No response from server. Check if backend is running.';
      } else {
        errorMessage = error.message;
      }
      
      console.error('Create post error:', {
        status: error.response?.status,
        data: error.response?.data,
        message: errorMessage
      });
      
      throw new Error(errorMessage);
    }
  },

  delete: async (id) => {
    try {
      const response = await api.delete(`/api/posts/${id}`);
      return response.data;
    } catch (error) {
      console.error(`Error deleting post ${id}:`, error.response?.data || error.message);
      throw error;
    }
  }
};