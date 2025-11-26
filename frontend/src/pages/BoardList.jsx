import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import { useSelector } from 'react-redux';
import client from '../api/client';

const fetchPosts = async () => {
  const response = await client.get('/boards');
  return response.data;
};

const BoardList = () => {
  const { user } = useSelector((state) => state.auth);
  const { data: posts, isLoading, error } = useQuery({
    queryKey: ['posts'],
    queryFn: fetchPosts,
  });

  if (isLoading) return <div className="loading">Loading...</div>;
  if (error) return <div className="error-message">Error loading posts</div>;

  return (
    <div className="board-container">
      <div className="board-header">
        <h1>Community Board</h1>
        {user && (
          <Link to="/create" className="btn-primary">
            New Post
          </Link>
        )}
      </div>
      
      <div className="post-list">
        {posts.length === 0 ? (
          <div className="empty-state">No posts yet. Be the first to share!</div>
        ) : (
          posts.map(post => (
            <Link to={`/posts/${post.id}`} key={post.id} className="post-card">
              <h3>{post.title}</h3>
              <div className="post-meta">
                <span>{new Date(post.created_at).toLocaleDateString()}</span>
              </div>
              <p className="post-preview">
                {post.content.length > 150 
                  ? `${post.content.substring(0, 150)}...` 
                  : post.content}
              </p>
            </Link>
          ))
        )}
      </div>
    </div>
  );
};

export default BoardList;
