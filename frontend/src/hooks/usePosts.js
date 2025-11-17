import { useState, useEffect } from 'react';
import { postService } from '../services/postService';

export const usePosts = (userId = null) => {
  const [posts, setPosts] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchPosts = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = userId 
        ? await postService.getByUser(userId)
        : await postService.getAll();
      setPosts(data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const createPost = async (postData) => {
    try {
      const newPost = await postService.create(postData);
      setPosts(prev => [...prev, newPost]);
      return newPost;
    } catch (err) {
      setError(err.message);
      throw err;
    }
  };

  const deletePost = async (id) => {
    try {
      await postService.delete(id);
      setPosts(prev => prev.filter(post => post.id !== id));
    } catch (err) {
      setError(err.message);
      throw err;
    }
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
    refetch: fetchPosts
  };
};