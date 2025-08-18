# Auth Server with MySQL & Password Encryption

A Rust-based authentication server with:
- MySQL and password hashing for secure storage
- JWT-based access control
- Role-based routes (user/admin)...

## Quick Start

### 1. Clone & Setup
```bash
git clone 
cd auth-server
```
### 2. Set up .env
See `sample.env`

### 3. Start MySQL with Docker
```bash
# Start MySQL and Auth-Server containers
docker-compose up -d
```

## API Endpoints

### Authentication
- `POST /register` - Create new user account (requires admin token)
```json
{
  "email": "testuser@test.com",
  "pw": "securepassword"
  "role": "user"
}
```
- `POST /login` - Login and get JWT token
```json
{
  "email": "testuser@test.com",
  "pw": "securepassword"
}
```

### Protected Routes
- `GET /user` - User-only endpoint (requires valid token)
```bash
Authorization: Bearer <JWT_TOKEN>
```
- `GET /admin` - Admin-only endpoint (requires admin token)

## Environment Variables

Create a `.env` file or set these environment variables:
- `JWT_SECRET`: Secret key for JWT signing
- `SERVER_HOST`: Server host
- `SERVER_PORT`: Server port (Default 3030)

- `MYSQL_ROOT_PASSWORD`: root sql password
- `MYSQL_DATABASE`: Name of the db
- `MYSQL_USER`: User of the db
- `MYSQL_PASSWORD`: User password
- `DB_PORT`: Db port (default: 3306)


## Next Steps 
- Token refresh mechanism
- Improved error handling & logging
- Password reset & email verification
