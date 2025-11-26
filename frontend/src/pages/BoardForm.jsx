import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useSelector } from 'react-redux';
import client from '../api/client';

const createPost = async (postData) => {
  const response = await client.post('/boards', postData);
  return response.data;
};

const BoardForm = () => {
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [error, setError] = useState('');
  
  const { user } = useSelector((state) => state.auth);
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: createPost,
    onSuccess: () => {
      // Invalidate and refetch
      queryClient.invalidateQueries({ queryKey: ['posts'] });
      navigate('/');
    },
    onError: () => {
      setError('Failed to create post. Please try again.');
    },
  });

  const handleSubmit = (e) => {
    e.preventDefault();
    if (!user) return;
    
    setError('');
    mutation.mutate({
      title,
      content,
      author_id: user.id
    });
  };

  return (
    <div className="board-container">
      <div className="form-card">
        <h2>Create New Post</h2>
        {error && <div className="error-message">{error}</div>}
        
        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label>Title</label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              required
              placeholder="Enter post title"
            />
          </div>
          
          <div className="form-group">
            <label>Content</label>
            <textarea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              required
              rows="10"
              placeholder="Write your thoughts here..."
            />
          </div>
          
          <div className="form-actions">
            <button 
              type="button" 
              onClick={() => navigate('/')} 
              className="btn-secondary"
            >
              Cancel
            </button>
            <button 
              type="submit" 
              className="btn-primary" 
              disabled={mutation.isPending}
            >
              {mutation.isPending ? 'Publishing...' : 'Publish Post'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default BoardForm;
