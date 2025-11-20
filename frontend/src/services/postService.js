import api from './api';

const postService = {
  // Get feed - all posts from all users (with pagination)
  getFeed: async (page = 1) => {
    const response = await api.get('/posts/feed', {
      params: { page }
    });
    // Handle both old format (array) and new format (object with data/pagination)
    return Array.isArray(response.data) ? response.data : response.data.data || [];
  },

  // Get all posts (for user's own posts)
  getAllPosts: async () => {
    const response = await api.get('/posts');
    return response.data;
  },

  // Search posts by title or content
  searchPosts: async (searchTerm) => {
    const response = await api.get('/posts/search', {
      params: {
        search: searchTerm
      }
    });
    return response.data;
  },

  // Get single post by ID
  getPostById: async (id) => {
    const response = await api.get(`/posts/${id}`);
    return response.data;
  },

  // Create new post
  createPost: async (title, content) => {
    const response = await api.post('/posts', {
      title,
      content,
    });
    return response.data;
  },

  // Update post
  updatePost: async (id, title, content) => {
    const response = await api.put(`/posts/${id}`, {
      title,
      content,
    });
    return response.data;
  },

  // Delete post
  deletePost: async (id) => {
    await api.delete(`/posts/${id}`);
    return { success: true };
  },
};

export default postService;
