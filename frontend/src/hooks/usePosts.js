import { useState, useEffect } from 'react';
import postService from '../services/postService';

export const usePosts = (userId = null) => {
  const [posts, setPosts] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchPosts = async () => {
    setLoading(true);
    setError(null);
    try {
      // If userId is provided, we ideally want posts by that user.
      // Since we don't have a specific endpoint for that yet, we'll use getFeed (all posts)
      // and filter client-side if needed, or just getFeed for now.
      // If userId is current user, we could use getAllPosts (my posts), but usePosts doesn't know current user ID.
      // For now, let's use getFeed for general list and getAllPosts if specifically requested (maybe via a prop?)
      // Actually, let's just use getFeed() as default, and if userId is passed, we might need to filter.
      // But to match previous intent:
      const data = userId
        ? await postService.getFeed() // Temporary: fetch all and filter? Or just fetch all.
        : await postService.getFeed();

      // If userId is provided, filter by it (client-side for now as backend doesn't support it)
      if (userId) {
        setPosts(Array.isArray(data) ? data.filter(p => p.user_id === parseInt(userId)) : []);
      } else {
        setPosts(Array.isArray(data) ? data : []);
      }
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const createPost = async (postData) => {
    try {
      const newPost = await postService.createPost(postData.title, postData.content);
      setPosts(prev => [newPost, ...prev]);
      return newPost;
    } catch (err) {
      setError(err.message);
      throw err;
    }
  };

  const deletePost = async (id) => {
    try {
      await postService.deletePost(id);
      setPosts(prev => prev.filter(post => post.id !== id));
    } catch (err) {
      setError(err.message);
      throw err;
    }
  };

  const updateCommentCount = (postId, count) => {
    setPosts(prev => prev.map(post =>
      post.id === postId
        ? { ...post, comments_count: count }
        : post
    ));
  };

  useEffect(() => {
    fetchPosts();
  }, [userId]);

  return {
    posts,
    loading,
    error,
    createPost,
    deletePost,
    updateCommentCount,
    refetch: fetchPosts
  };
};