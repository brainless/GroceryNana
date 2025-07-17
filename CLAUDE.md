# GroceryNana Development Guide

## Project Overview
GroceryNana is a marketplace application connecting grocery buyers with local sellers. Buyers create grocery lists, and verified sellers can send offers to fulfill them. The system includes location-based matching, in-app messaging, and admin verification for sellers.

## Development Workflow
- Create a new branch for each task
- Branch names should start with chore/ or feature/ or fix/
- Please add tests for any new features added, particularly integration tests
- Please run formatters, linters and tests before committing changes
- When finished please commit and push to the new branch
- Please mention GitHub issue if provided

## Tech Stack

### Backend (Rust)
- **Framework**: Actix Web
- **Database**: SQLite with sqlx
- **Type Generation**: ts-rs for TypeScript payload types
- **Authentication**: JWT tokens
- **SMS**: Twilio for phone verification
- **File Storage**: Local filesystem for seller verification documents

### Frontend (React/TypeScript)
- **Framework**: React 18 with TypeScript
- **Package Manager**: pnpm
- **UI Components**: shadcn/ui
- **Routing**: React Router
- **State Management**: Zustand
- **HTTP Client**: Axios
- **Form Handling**: React Hook Form with Zod validation

### Development Commands

#### Backend
```bash
cd backend
cargo run                    # Start development server
cargo test                   # Run tests
cargo clippy                 # Lint code
cargo fmt                    # Format code
sqlx migrate run             # Run database migrations
```

#### Frontend
```bash
cd webapp
pnpm install                 # Install dependencies
pnpm run dev                 # Start development server
pnpm run build               # Build for production
pnpm run lint                # Run ESLint
pnpm run typecheck           # Run TypeScript checks
pnpm test                    # Run tests
```

## Architecture

### Database Schema
- `users` - User accounts (buyers and sellers)
- `phone_verifications` - OTP codes for phone verification
- `grocery_lists` - Buyer grocery lists with location and expiry
- `grocery_items` - Items within grocery lists
- `offers` - Seller offers for grocery lists
- `messages` - In-app messaging between buyers and sellers
- `seller_verifications` - Admin verification data for sellers

### API Endpoints
- `POST /api/auth/register` - User registration
- `POST /api/auth/verify-phone` - Phone verification
- `POST /api/auth/login` - User login
- `GET /api/grocery-lists` - Get user's grocery lists
- `POST /api/grocery-lists` - Create grocery list
- `GET /api/offers` - Get offers for grocery lists
- `POST /api/offers` - Create offer (sellers only)
- `GET /api/messages` - Get messages
- `POST /api/messages` - Send message
- `POST /api/admin/verify-seller` - Admin seller verification

### Type Generation
Use `ts-rs` to generate TypeScript types from Rust structs:
- Add `#[derive(serde::Serialize, serde::Deserialize, ts_rs::TS)]` to payload structs
- Run `cargo test` to generate types in `webapp/src/types/`

### Key Features
1. **Phone Verification**: OTP-based phone number verification for all users
2. **Location-Based Matching**: Buyers specify pickup range, sellers see nearby lists
3. **In-App Messaging**: Direct communication between buyers and sellers per grocery list
4. **Price Negotiation**: Sellers can update offer prices, buyers can accept
5. **Admin Verification**: Manual verification of sellers with business documents
6. **List Expiry**: Grocery lists expire after one week
7. **Single Active List**: Each buyer can have only one active grocery list

### Security Considerations
- JWT tokens for authentication
- Input validation with Zod (frontend) and serde (backend)
- Phone number verification required
- Admin-only seller verification
- File upload validation for seller documents

### Environment Variables
Backend (.env):
```
DATABASE_URL=sqlite:./database.db
JWT_SECRET=your-jwt-secret
TWILIO_ACCOUNT_SID=your-twilio-sid
TWILIO_AUTH_TOKEN=your-twilio-token
ADMIN_EMAIL=your-admin-email
```

Frontend (.env):
```
VITE_API_URL=http://localhost:8080
```
