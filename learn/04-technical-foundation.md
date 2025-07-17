# Chapter 4: Technical Foundation

## Understanding Modern Software Architecture

To effectively practice vibe coding, you need to understand the technical foundation of modern software development. This chapter covers the essential concepts and technologies you'll encounter.

## Full-Stack Architecture Overview

### The Three-Tier Architecture

**Frontend (Presentation Layer)**:
- What users see and interact with
- Runs in browsers or mobile apps
- Technologies: React, Vue, Angular, React Native

**Backend (Business Logic Layer)**:
- Processes requests and business rules
- Handles authentication and authorization
- Technologies: Node.js, Python, Rust, Java, Go

**Database (Data Layer)**:
- Stores and retrieves data
- Ensures data consistency and integrity
- Technologies: PostgreSQL, MySQL, MongoDB, SQLite

### Modern Architecture Patterns

**Microservices**:
- Break applications into small, independent services
- Each service handles a specific business function
- Benefits: Scalability, maintainability, technology diversity

**API-First Development**:
- Design APIs before implementing features
- Enables frontend and backend to develop in parallel
- Supports multiple client types (web, mobile, IoT)

**Serverless Architecture**:
- Functions run on-demand without managing servers
- Pay only for actual usage
- Examples: AWS Lambda, Vercel Functions, Cloudflare Workers

## Frontend Technologies

### Modern JavaScript/TypeScript

**Why TypeScript?**:
- Adds static typing to JavaScript
- Catches errors at compile time
- Better IDE support and code completion
- Improved code maintainability

**React Fundamentals**:
```typescript
// Functional component with hooks
import React, { useState, useEffect } from 'react';

function GroceryList() {
  const [items, setItems] = useState([]);
  
  useEffect(() => {
    // Fetch items from API
    fetchGroceryItems().then(setItems);
  }, []);
  
  return (
    <div>
      {items.map(item => (
        <div key={item.id}>{item.name}</div>
      ))}
    </div>
  );
}
```

### State Management

**Local State**: useState for component-specific data
**Global State**: Context API, Zustand, or Redux for shared data
**Server State**: React Query or SWR for API data

### UI Libraries

**Utility-First CSS**:
- Tailwind CSS for rapid styling
- Responsive design with mobile-first approach

**Component Libraries**:
- shadcn/ui for customizable components
- Material-UI for Google Material Design
- Ant Design for enterprise applications

## Backend Technologies

### Choosing a Backend Language

**Node.js/TypeScript**:
- Same language as frontend
- Large ecosystem (npm)
- Good for I/O-heavy applications

**Python**:
- Easy to learn and read
- Great for data processing
- Frameworks: Django, FastAPI, Flask

**Rust**:
- Memory safety without garbage collection
- High performance
- Growing ecosystem

**Go**:
- Simple, efficient language
- Excellent for concurrent programming
- Fast compilation

### API Design

**RESTful APIs**:
```
GET    /api/grocery-lists     # Get all lists
POST   /api/grocery-lists     # Create new list
GET    /api/grocery-lists/:id # Get specific list
PUT    /api/grocery-lists/:id # Update list
DELETE /api/grocery-lists/:id # Delete list
```

**GraphQL**:
- Single endpoint for all data
- Clients specify exactly what data they need
- Strongly typed schema

**Real-time Communication**:
- WebSockets for bidirectional communication
- Server-Sent Events for server-to-client updates

### Authentication and Security

**JWT (JSON Web Tokens)**:
```javascript
// Token structure
{
  "header": { "alg": "HS256", "typ": "JWT" },
  "payload": { "user_id": "123", "exp": 1234567890 },
  "signature": "encrypted_signature"
}
```

**OAuth 2.0**:
- Standard for authorization
- Enables "Login with Google/Facebook"
- Secure delegation of access

**Security Best Practices**:
- Input validation and sanitization
- SQL injection prevention
- XSS (Cross-Site Scripting) protection
- CSRF (Cross-Site Request Forgery) protection

## Database Technologies

### Relational Databases

**PostgreSQL**:
- ACID compliance
- Advanced features (JSON, arrays, full-text search)
- Excellent for complex queries

**SQLite**:
- Embedded database
- Perfect for development and small applications
- Zero configuration

**MySQL**:
- Wide adoption
- Good performance
- Strong ecosystem

### NoSQL Databases

**MongoDB**:
- Document-oriented storage
- Flexible schema
- Good for rapid development

**Redis**:
- In-memory key-value store
- Excellent for caching and sessions
- Pub/sub messaging

### Database Design Principles

**Normalization**:
- Eliminate data redundancy
- Ensure data integrity
- Optimize for consistency

**Denormalization**:
- Trade storage for query performance
- Reduce complex joins
- Optimize for read-heavy workloads

## Development Tools and Environment

### Version Control

**Git Fundamentals**:
```bash
git init                    # Initialize repository
git add .                   # Stage changes
git commit -m "message"     # Commit changes
git push origin main        # Push to remote
git pull origin main        # Pull from remote
```

**Branching Strategy**:
- Feature branches for new development
- Main branch for production-ready code
- Pull requests for code review

### Package Management

**Frontend**:
- npm/yarn for JavaScript packages
- package.json for dependency management

**Backend**:
- npm (Node.js)
- pip (Python)
- cargo (Rust)
- go mod (Go)

### Development Environment

**Code Editors**:
- VS Code with relevant extensions
- Cursor for AI-powered development
- WebStorm for JavaScript/TypeScript

**Essential Extensions**:
- Language support
- Debugger integration
- Git integration
- Linting and formatting

## API Integration

### HTTP Client Libraries

**Frontend**:
```typescript
// Using axios
import axios from 'axios';

const api = axios.create({
  baseURL: 'http://localhost:8080/api',
  headers: {
    'Authorization': `Bearer ${token}`
  }
});

// Making requests
const groceryLists = await api.get('/grocery-lists');
const newList = await api.post('/grocery-lists', { name: 'Weekly Shopping' });
```

**Backend**:
```rust
// Using reqwest in Rust
use reqwest::Client;

let client = Client::new();
let response = client
    .get("https://api.example.com/data")
    .header("Authorization", "Bearer token")
    .send()
    .await?;
```

### Error Handling

**Frontend Error Handling**:
```typescript
try {
  const response = await api.get('/grocery-lists');
  return response.data;
} catch (error) {
  if (error.response?.status === 401) {
    // Handle unauthorized
    logout();
  } else {
    // Handle other errors
    showErrorMessage(error.message);
  }
}
```

**Backend Error Handling**:
```rust
use actix_web::{HttpResponse, Result};

async fn get_grocery_lists() -> Result<HttpResponse> {
    match fetch_lists_from_db().await {
        Ok(lists) => Ok(HttpResponse::Ok().json(lists)),
        Err(e) => {
            log::error!("Failed to fetch lists: {}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}
```

## Configuration and Environment

### Environment Variables

**Frontend (.env)**:
```
VITE_API_URL=http://localhost:8080
VITE_APP_NAME=GroceryNana
```

**Backend (.env)**:
```
DATABASE_URL=postgresql://user:pass@localhost/db
JWT_SECRET=your-secret-key
PORT=8080
```

### Configuration Management

**Frontend**:
```typescript
// config.ts
export const config = {
  apiUrl: import.meta.env.VITE_API_URL || 'http://localhost:8080',
  appName: import.meta.env.VITE_APP_NAME || 'My App',
};
```

**Backend**:
```rust
// config.rs
use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap(),
        }
    }
}
```

## Performance Considerations

### Frontend Performance

**Code Splitting**:
```typescript
// Lazy loading components
const GroceryList = React.lazy(() => import('./components/GroceryList'));

function App() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <GroceryList />
    </Suspense>
  );
}
```

**Optimization Techniques**:
- Minimize bundle size
- Use CDN for static assets
- Implement caching strategies
- Optimize images and fonts

### Backend Performance

**Database Optimization**:
- Use appropriate indexes
- Optimize queries
- Implement connection pooling
- Consider read replicas

**Caching Strategies**:
- Redis for session storage
- Application-level caching
- HTTP caching headers
- CDN for static content

## Common Patterns and Practices

### Design Patterns

**Repository Pattern**:
```typescript
interface GroceryListRepository {
  findAll(): Promise<GroceryList[]>;
  findById(id: string): Promise<GroceryList | null>;
  create(list: CreateGroceryListDto): Promise<GroceryList>;
  update(id: string, updates: UpdateGroceryListDto): Promise<GroceryList>;
  delete(id: string): Promise<void>;
}
```

**Service Layer**:
```typescript
class GroceryListService {
  constructor(private repository: GroceryListRepository) {}
  
  async createList(userId: string, data: CreateGroceryListDto): Promise<GroceryList> {
    // Business logic here
    const list = await this.repository.create({ ...data, userId });
    // Send notifications, log events, etc.
    return list;
  }
}
```

### Error Handling Patterns

**Result/Option Types**:
```rust
use std::result::Result;

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

## What's Next?

Now that you understand the technical foundation, you're ready to learn about development workflows and best practices. In the next chapter, we'll explore Git workflows, branching strategies, and continuous integration.

---

**Next**: [Chapter 5: Development Workflow](05-development-workflow.md)
**Previous**: [Chapter 3: Working with LLMs](03-working-with-llms.md)