import { useQuery } from '@tanstack/react-query';
import { useParams, useNavigate } from 'react-router-dom';
import client from '../api/client';

const fetchPost = async (id) => {
  const response = await client.get(`/boards/${id}`);
  return response.data;
};

const BoardDetail = () => {
  const { id } = useParams();
  const navigate = useNavigate();

  const { data: post, isLoading, error } = useQuery({
    queryKey: ['post', id],
    queryFn: () => fetchPost(id),
    retry: false,
  });

  if (isLoading) return <div className="loading">Loading...</div>;
  
  if (error) {
    return (
      <div className="board-container">
        <div className="error-message">Post not found</div>
        <button onClick={() => navigate('/')} className="btn-secondary">Back to Board</button>
      </div>
    );
  }

  return (
    <div className="board-container">
      <button onClick={() => navigate('/')} className="btn-back">
        &larr; Back to Board
      </button>
      
      <article className="post-detail">
        <header className="post-header">
          <h1>{post.title}</h1>
          <div className="post-meta">
            <span>Posted on {new Date(post.created_at).toLocaleDateString()}</span>
          </div>
        </header>
        
        <div className="post-content">
          {post.content.split('\n').map((paragraph, index) => (
            <p key={index}>{paragraph}</p>
          ))}
        </div>
      </article>
    </div>
  );
};

export default BoardDetail;
