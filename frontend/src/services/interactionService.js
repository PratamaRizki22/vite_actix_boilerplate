import api from './api'

export const interactionService = {
  // Like operations
  toggleLike: async (postId) => {
    const response = await api.post(`/posts/${postId}/like`)
    return response.data
  },

  checkLike: async (postId) => {
    const response = await api.get(`/posts/${postId}/like/check`)
    return response.data
  },

  // Comment operations
  createComment: async (postId, content, parentCommentId = null) => {
    const response = await api.post(`/posts/${postId}/comments`, {
      content,
      parent_comment_id: parentCommentId,
    })
    return response.data
  },

  getComments: async (postId) => {
    const response = await api.get(`/posts/${postId}/comments`)
    return response.data
  },

  updateComment: async (commentId, content) => {
    const response = await api.put(`/posts/comments/${commentId}`, {
      content,
    })
    return response.data
  },

  deleteComment: async (commentId) => {
    const response = await api.delete(`/posts/comments/${commentId}`)
    return response.data
  },
}

export default interactionService
