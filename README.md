# Auth Server with MySQL & Password Encryption

## Features Added
- ✅ **Bcrypt Password Hashing**: All passwords are securely hashed
- ✅ **User Registration**: New endpoint to create users
- ✅ **Password Verification**: Secure login with hash comparison
- ✅ **Duplicate Prevention**: Email uniqueness enforced
- ✅ **Containerised**: run `docker-compose up -d` and you're good to go

## Quick Start

### 0. Prereqs
I had to run this command for making it possible to build auth_server container. It generated an .sqlx folder which is essential.
```bash
cargo sqlx prepare --workspace
```

Don't forget to add .env (see below)

### 1. Start MySQL with Docker
```bash
# Start MySQL and Auth-Server containers
docker-compose up -d
```
### 2. View logs
```bash
# Wait for MySQL to be ready (check logs)
docker-compose logs -f mysql
docker-compose logs -f auth_server
```

## API Endpoints

### Authentication
- `POST /register` - Create new user account
- `POST /login` - Login and get JWT token

### Protected Routes
- `GET /user` - User-only endpoint (requires valid token)
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


## Next Steps (Phase 2)
- Token refresh mechanism
- Other improvements
