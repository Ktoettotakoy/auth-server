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

### 3. Test the Endpoints

#### Register New User
```bash
curl -X POST http://localhost:3030/register \
  -H "Content-Type: application/json" \
  -d '{"email": "u1@user.com", "pw": "pw1", "role": "user"}'

curl -X POST http://localhost:3030/register \
  -H "Content-Type: application/json" \
  -d '{"email": "u2@admin.com", "pw": "pw2", "role": "admin"}'
```

#### Login (User)
```bash
curl -X POST http://localhost:3030/login \
  -H "Content-Type: application/json" \
  -d '{"email": "u1@user.com", "pw": "pw1"}'
```

#### Login (Admin)
```bash
curl -X POST http://localhost:3030/login \
  -H "Content-Type: application/json" \
  -d '{"email": "u2@admin.com", "pw": "pw2"}'
```

#### Access User Endpoint
```bash
# Use token from login response
curl -H "Authorization: Bearer <YOUR_TOKEN>" \
  http://localhost:3030/user
```

#### Access Admin Endpoint
```bash
# Use admin token from login response
curl -H "Authorization: Bearer <YOUR_ADMIN_TOKEN>" \
  http://localhost:3030/admin
```

## API Endpoints

### Authentication
- `POST /register` - Create new user account
- `POST /login` - Login and get JWT token

### Protected Routes
- `GET /user` - User-only endpoint (requires valid token)
- `GET /admin` - Admin-only endpoint (requires admin token)

## Security Features

### Password Hashing
- Uses bcrypt with default cost factor (12)
- Each password gets unique salt automatically
- Passwords are never stored in plaintext

### Password Security
```rust
// Passwords are hashed before storage
let hash = bcrypt::hash("password123", DEFAULT_COST)?;

// During login, passwords are verified against hash
let is_valid = bcrypt::verify("password123", &stored_hash)?;
```

## Environment Variables

Create a `.env` file (see .env example) or set these environment variables:
- `DATABASE_URL`: MySQL connection string
- `JWT_SECRET`: Secret key for JWT signing
- `SERVER_HOST`: Server host (default: 127.0.0.1)
- `SERVER_PORT`: Server port (default: 3030)


## Next Steps (Phase 2)
- Token refresh mechanism
