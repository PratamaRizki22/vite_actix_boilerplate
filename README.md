# Wallet Core - Full Stack Application

Full-stack blockchain wallet application with Rust backend and React frontend.

## Quick Start

### Prerequisites
- Docker & Docker Compose
- Git

### Deploy with One Command

```bash
chmod +x deploy.sh
./deploy.sh
```

This will:
- ✅ Build all services (Frontend, Backend, PostgreSQL, Redis)
- ✅ Start all containers
- ✅ Setup networks and volumes
- ✅ Run migrations automatically

### Access the Application

- **Frontend**: http://localhost:5173
- **Backend API**: http://localhost:8080
- **PostgreSQL**: localhost:5432
- **Redis**: localhost:6379

### Default Admin Account

- **Username**: admin
- **Email**: admin@example.com
- **Password**: admin123
- **Role**: admin

Note: Please change the default admin password after first login for security.

## Manual Commands

### Start services
```bash
docker-compose up -d
```

### Stop services
```bash
./stop.sh
# or
docker-compose down
```

### View logs
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f frontend
```

### Rebuild after changes
```bash
docker-compose up -d --build
```

### Stop and remove everything (including data)
```bash
docker-compose down -v
```

## Services

### Backend (Rust + Actix-web)
- Port: 8080
- Features:
  - JWT Authentication
  - Web3 Wallet Integration
  - Redis Session Storage
  - Redis Rate Limiting
  - Redis Caching (Users & Posts)
  - Email Verification
  - 2FA/TOTP Support
  - Password Reset
  - Audit Logging

### Frontend (React + Vite)
- Port: 5173
- Features:
  - Modern UI with Tailwind CSS
  - Web3 Wallet Connection
  - User Authentication
  - Post Management
  - Real-time Updates

### Database
- PostgreSQL 15
- Redis 7

## Configuration

Copy `.env.example` to `.env` and update:

```bash
cp .env.example .env
```

**Important**: Change these in production:
- `JWT_SECRET` - Use strong random string
- `POSTGRES_PASSWORD` - Use strong password
- `REDIS_PASSWORD` - Use strong password
- `CORS_ORIGIN` - Set to your frontend domain

## Security Features

- JWT Token Authentication
- Redis Token Blacklist
- Rate Limiting per IP/Endpoint
- Account Lockout after failed attempts
- Session Management with Redis
- 2FA/TOTP Support
- Password Hashing (bcrypt)
- Security Headers
- CORS Protection
- SQL Injection Prevention (sqlx)

## Redis Usage

1. **Session Storage** - Fast session lookups
2. **Token Blacklist** - Instant token revocation
3. **Rate Limiting** - Per-endpoint request limits
4. **Caching** - User profiles & posts feed

## Development

### Backend Development
```bash
cd backend
cargo watch -x run
```

### Frontend Development
```bash
cd frontend
npm run dev
```

### Run Tests
```bash
cd backend
cargo test
```

## Environment Variables

See `.env.example` for all available configuration options.

## Troubleshooting

### Services not starting?
```bash
docker-compose down -v
docker-compose up -d --build
```

### Check service health
```bash
docker-compose ps
docker-compose logs backend
```

### Reset everything
```bash
docker-compose down -v
rm -rf backend/target
./deploy.sh
```

## License

MIT
