# Docker Development Setup

## Prerequisites
- Docker
- Docker Compose

## Quick Start

### Build and Run All Services
```bash
docker-compose -f docker-compose-dev.yml up --build
```

This command will:
1. Build the PostgreSQL database container
2. Build the Rust backend container
3. Build the React frontend container
4. Start all services with proper networking

### Access the Application

- **Frontend**: http://localhost:5173
- **Backend API**: http://localhost:3000
- **Swagger Documentation**: http://localhost:3000/docs
- **PostgreSQL**: localhost:5432

### Default Credentials

**Database:**
- Host: localhost
- Port: 5432
- Database: study_snake_xenon_board
- User: postgres
- Password: password

**JWT:**
- Secret: super_secret_key_change_me (change in production)

## Common Commands

### Start Services (Detached Mode)
```bash
docker-compose -f docker-compose-dev.yml up -d
```

### Stop Services
```bash
docker-compose -f docker-compose-dev.yml down
```

### Stop and Remove Volumes (Clean Database)
```bash
docker-compose -f docker-compose-dev.yml down -v
```

### View Logs
```bash
# All services
docker-compose -f docker-compose-dev.yml logs -f

# Specific service
docker-compose -f docker-compose-dev.yml logs -f backend
docker-compose -f docker-compose-dev.yml logs -f frontend
docker-compose -f docker-compose-dev.yml logs -f postgres
```

### Rebuild Specific Service
```bash
docker-compose -f docker-compose-dev.yml up --build backend
docker-compose -f docker-compose-dev.yml up --build frontend
```

## Architecture

### Services

1. **postgres** - PostgreSQL 16 Alpine
   - Database initialization with schema.sql
   - Health check configured
   - Persistent volume for data

2. **backend** - Rust Axum API
   - Multi-stage build (builder + runtime)
   - Waits for database health check
   - Auto-restart enabled

3. **frontend** - React + Vite + Nginx
   - Multi-stage build (Node builder + Nginx runtime)
   - Optimized for production serving
   - Client-side routing support

### Network

All services communicate through `app-network` bridge network.

### Volumes

- `postgres_data` - Persistent PostgreSQL data

## Troubleshooting

### Database Connection Issues
If backend can't connect to database:
```bash
docker-compose -f docker-compose-dev.yml logs postgres
docker-compose -f docker-compose-dev.yml restart backend
```

### Frontend API Connection Issues
The frontend connects to backend at `http://localhost:3000/api`. If running in Docker, ensure:
- Backend is running and healthy
- CORS is properly configured in backend

### Build Issues

**Backend Build Fails:**
- Check Rust version compatibility
- Ensure all dependencies in Cargo.toml are available

**Frontend Build Fails:**
- Check Node.js version (requires Node 20+)
- Clear npm cache: `docker-compose -f docker-compose-dev.yml build --no-cache frontend`

### Port Conflicts
If ports are already in use:
- Backend: Change `3000:3000` to `3001:3000` in docker-compose-dev.yml
- Frontend: Change `5173:80` to `5174:80` in docker-compose-dev.yml
- PostgreSQL: Change `5432:5432` to `5433:5432` in docker-compose-dev.yml

## Development Workflow

1. Make code changes in your local files
2. Rebuild the affected service:
   ```bash
   docker-compose -f docker-compose-dev.yml up --build backend
   # or
   docker-compose -f docker-compose-dev.yml up --build frontend
   ```
3. Test the changes at http://localhost:5173

## Production Considerations

For production deployment, consider:
- Change JWT_SECRET to a strong random value
- Use environment-specific docker-compose files
- Configure proper database credentials
- Set up SSL/TLS certificates
- Configure proper CORS origins
- Use Docker secrets for sensitive data
- Set RUST_LOG to `info` or `warn` instead of `debug`

## Security Notes

⚠️ **Warning**: The current Docker images have some vulnerabilities. For production:
- Regularly update base images
- Use specific image versions instead of `latest`
- Run security scans with tools like Trivy or Snyk
- Consider using distroless or hardened base images
