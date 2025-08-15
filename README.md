# Auth Server with MySQL & Password Encryption

## Features Added
- ✅ **Bcrypt Password Hashing**: All passwords are securely hashed
- ✅ **User Registration**: New endpoint to create users
- ✅ **Password Verification**: Secure login with hash comparison
- ✅ **Duplicate Prevention**: Email uniqueness enforced

## Quick Start

### 1. Start MySQL with Docker
```bash
# Start MySQL container
docker-compose up -d

# Wait for MySQL to be ready (check logs)
docker-compose logs -f mysql
```

### 2. Run the Application
```bash
# Install dependencies and run
cargo run
```

## API Endpoints

### Authentication
- `POST /register` - Create new user account
- `POST /login` - Login and get JWT token

### Protected Routes
- `GET /user` - User-only endpoint (requires valid token)
- `GET /admin` - Admin-only endpoint (requires admin token)

## Environment Variables

### root
Create a `.env` file or set these environment variables:
- `JWT_SECRET`: Secret key for JWT signing
- `SERVER_HOST`: Server host (default: 127.0.0.1)
- `SERVER_PORT`: Server port (default: 3030)

### Inside db folder
If you want to use a docker-compose

Create a `.env` file or set these environment variables:
- `MYSQL_ROOT_PASSWORD`: root sql password
- `MYSQL_DATABASE`: Name of the db
- `MYSQL_USER`: User of the db
- `MYSQL_PASSWORD`: User password
- `DB_PORT`: Db port (default: 3030)


## Next Steps (Phase 2)
- Token refresh mechanism
- Other improvements
