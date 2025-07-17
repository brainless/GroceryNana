# Chapter 7: Infrastructure and Deployment

## From Code to Production

Getting your vibe coding projects from development to production requires understanding modern infrastructure and deployment practices. This chapter covers cloud services, deployment strategies, and production monitoring.

## Cloud Infrastructure Overview

### Infrastructure as Code (IaC)

**Why IaC?**
- Version controlled infrastructure
- Reproducible deployments
- Automated provisioning
- Consistent environments

**Popular Tools**:
- **Terraform**: Multi-cloud provisioning
- **AWS CloudFormation**: AWS-specific
- **Pulumi**: Infrastructure as code with familiar languages
- **Docker Compose**: Container orchestration

### Cloud Providers Comparison

**AWS (Amazon Web Services)**:
- Most comprehensive service offering
- Complex pricing and learning curve
- Great for enterprise applications

**Google Cloud Platform (GCP)**:
- Strong in AI/ML services
- Competitive pricing
- Good developer experience

**DigitalOcean**:
- Simple, developer-friendly
- Predictable pricing
- Great for startups and small projects

**Vercel/Netlify**:
- Specialized for frontend deployment
- Excellent developer experience
- Integrated with Git workflows

## Containerization with Docker

### Dockerfile Best Practices

**Frontend Dockerfile**:
```dockerfile
# Multi-stage build for React app
FROM node:18-alpine AS builder

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

# Production stage
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

**Backend Dockerfile (Rust)**:
```dockerfile
# Build stage
FROM rust:1.70 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/grocerynana-backend /usr/local/bin/app
EXPOSE 8080
CMD ["app"]
```

### Docker Compose for Development

**Complete Development Environment**:
```yaml
version: '3.8'

services:
  frontend:
    build: ./webapp
    ports:
      - "3000:80"
    depends_on:
      - backend
    environment:
      - VITE_API_URL=http://localhost:8080

  backend:
    build: ./backend
    ports:
      - "8080:8080"
    depends_on:
      - database
      - redis
    environment:
      - DATABASE_URL=postgresql://user:password@database:5432/grocerynana
      - REDIS_URL=redis://redis:6379
    volumes:
      - ./backend:/app
      - /app/target

  database:
    image: postgres:15
    environment:
      - POSTGRES_DB=grocerynana
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

volumes:
  postgres_data:
```

## Deployment Strategies

### Static Site Deployment

**Vercel Deployment**:
```json
{
  "builds": [
    {
      "src": "webapp/package.json",
      "use": "@vercel/static-build",
      "config": {
        "distDir": "dist"
      }
    }
  ],
  "routes": [
    {
      "src": "/api/(.*)",
      "dest": "https://api.grocerynana.com/$1"
    },
    {
      "src": "/(.*)",
      "dest": "/index.html"
    }
  ]
}
```

**Netlify Configuration**:
```toml
# netlify.toml
[build]
  command = "npm run build"
  publish = "dist"

[[redirects]]
  from = "/api/*"
  to = "https://api.grocerynana.com/:splat"
  status = 200

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

### Container Deployment

**DigitalOcean App Platform**:
```yaml
# .do/app.yaml
name: grocerynana
services:
- name: frontend
  source_dir: webapp
  github:
    repo: your-username/grocerynana
    branch: main
  run_command: serve -s dist -l 80
  environment_slug: node-js
  instance_count: 1
  instance_size_slug: basic-xxs
  routes:
  - path: /

- name: backend
  source_dir: backend
  github:
    repo: your-username/grocerynana
    branch: main
  run_command: ./target/release/grocerynana-backend
  environment_slug: docker
  instance_count: 1
  instance_size_slug: basic-xxs
  routes:
  - path: /api
  envs:
  - key: DATABASE_URL
    value: ${database.DATABASE_URL}

databases:
- name: database
  engine: PG
  version: "15"
  size: db-s-dev-database
```

### Kubernetes Deployment

**Basic Kubernetes Configuration**:
```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: grocerynana-backend
spec:
  replicas: 3
  selector:
    matchLabels:
      app: grocerynana-backend
  template:
    metadata:
      labels:
        app: grocerynana-backend
    spec:
      containers:
      - name: backend
        image: your-registry/grocerynana-backend:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: database-secret
              key: url
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "256Mi"
            cpu: "200m"
---
apiVersion: v1
kind: Service
metadata:
  name: grocerynana-backend-service
spec:
  selector:
    app: grocerynana-backend
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

## CI/CD Pipelines

### GitHub Actions Deployment

**Complete CI/CD Pipeline**:
```yaml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Run tests
      run: |
        cd webapp && npm ci && npm test
        cd backend && cargo test

  build-and-deploy:
    needs: test
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Build and push Docker images
      env:
        DOCKER_REGISTRY: ${{ secrets.DOCKER_REGISTRY }}
        DOCKER_USERNAME: ${{ secrets.DOCKER_USERNAME }}
        DOCKER_PASSWORD: ${{ secrets.DOCKER_PASSWORD }}
      run: |
        echo $DOCKER_PASSWORD | docker login $DOCKER_REGISTRY -u $DOCKER_USERNAME --password-stdin
        
        # Build and push backend
        docker build -t $DOCKER_REGISTRY/grocerynana-backend:${{ github.sha }} ./backend
        docker push $DOCKER_REGISTRY/grocerynana-backend:${{ github.sha }}
        
        # Build and push frontend
        docker build -t $DOCKER_REGISTRY/grocerynana-frontend:${{ github.sha }} ./webapp
        docker push $DOCKER_REGISTRY/grocerynana-frontend:${{ github.sha }}
    
    - name: Deploy to DigitalOcean
      uses: digitalocean/action-doctl@v2
      with:
        token: ${{ secrets.DIGITALOCEAN_ACCESS_TOKEN }}
    
    - name: Update deployment
      run: |
        doctl apps update ${{ secrets.APP_ID }} --spec .do/app.yaml
```

### Database Migrations

**Migration Strategy**:
```yaml
# Migration job
- name: Run database migrations
  run: |
    docker run --rm \
      -e DATABASE_URL=${{ secrets.DATABASE_URL }} \
      $DOCKER_REGISTRY/grocerynana-backend:${{ github.sha }} \
      migrate
```

**Rust Migration Example**:
```rust
// migrations/src/main.rs
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations completed successfully");
    Ok(())
}
```

## Environment Management

### Environment Variables

**Production Environment Setup**:
```bash
# Production .env
DATABASE_URL=postgresql://user:password@prod-db:5432/grocerynana
REDIS_URL=redis://prod-redis:6379
JWT_SECRET=super-secure-production-secret
TWILIO_ACCOUNT_SID=your-production-sid
TWILIO_AUTH_TOKEN=your-production-token
RUST_LOG=info
```

**Environment-Specific Configuration**:
```rust
// config.rs
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub twilio_account_sid: String,
    pub twilio_auth_token: String,
    pub environment: Environment,
}

#[derive(Clone, PartialEq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let environment = match env::var("ENVIRONMENT")?.as_str() {
            "production" => Environment::Production,
            "staging" => Environment::Staging,
            _ => Environment::Development,
        };
        
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            twilio_account_sid: env::var("TWILIO_ACCOUNT_SID")?,
            twilio_auth_token: env::var("TWILIO_AUTH_TOKEN")?,
            environment,
        })
    }
}
```

## Monitoring and Observability

### Application Monitoring

**Health Check Endpoint**:
```rust
use actix_web::{web, HttpResponse, Result};
use serde_json::json;

pub async fn health_check(
    db: web::Data<sqlx::PgPool>,
    redis: web::Data<redis::Client>,
) -> Result<HttpResponse> {
    // Check database connection
    let db_status = match sqlx::query("SELECT 1").fetch_one(db.as_ref()).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };
    
    // Check Redis connection
    let redis_status = match redis.get_connection() {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };
    
    let overall_status = if db_status == "healthy" && redis_status == "healthy" {
        "healthy"
    } else {
        "unhealthy"
    };
    
    Ok(HttpResponse::Ok().json(json!({
        "status": overall_status,
        "database": db_status,
        "redis": redis_status,
        "timestamp": chrono::Utc::now()
    })))
}
```

### Logging and Metrics

**Structured Logging**:
```rust
use tracing::{info, error, instrument};
use tracing_subscriber;

#[instrument]
pub async fn create_grocery_list(
    user_id: String,
    request: CreateGroceryListRequest,
) -> Result<GroceryList, AppError> {
    info!("Creating grocery list for user {}", user_id);
    
    match grocery_list_service.create(user_id, request).await {
        Ok(list) => {
            info!("Successfully created grocery list {}", list.id);
            Ok(list)
        }
        Err(e) => {
            error!("Failed to create grocery list: {}", e);
            Err(e)
        }
    }
}
```

**Metrics Collection**:
```rust
use prometheus::{Counter, Histogram, register_counter, register_histogram};

lazy_static::lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: Counter = register_counter!(
        "http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();
    
    static ref HTTP_REQUEST_DURATION: Histogram = register_histogram!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds"
    ).unwrap();
}

// In your middleware
HTTP_REQUESTS_TOTAL.inc();
let timer = HTTP_REQUEST_DURATION.start_timer();
// ... process request
timer.observe_duration();
```

## Security in Production

### SSL/TLS Configuration

**Nginx SSL Configuration**:
```nginx
server {
    listen 443 ssl http2;
    server_name grocerynana.com;
    
    ssl_certificate /etc/letsencrypt/live/grocerynana.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/grocerynana.com/privkey.pem;
    
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;
    
    location / {
        proxy_pass http://frontend:80;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
    
    location /api/ {
        proxy_pass http://backend:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Security Headers

**Security Middleware**:
```rust
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .header("X-Frame-Options", "DENY")
        .header("X-Content-Type-Options", "nosniff")
        .header("X-XSS-Protection", "1; mode=block")
        .header("Strict-Transport-Security", "max-age=31536000; includeSubDomains")
        .header("Content-Security-Policy", "default-src 'self'")
}
```

## Backup and Disaster Recovery

### Database Backups

**Automated Backup Script**:
```bash
#!/bin/bash
# backup.sh

DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/backups"
DB_NAME="grocerynana"

# Create backup
pg_dump $DATABASE_URL > $BACKUP_DIR/backup_$DATE.sql

# Upload to S3
aws s3 cp $BACKUP_DIR/backup_$DATE.sql s3://grocerynana-backups/

# Clean up old backups (keep last 7 days)
find $BACKUP_DIR -name "backup_*.sql" -mtime +7 -delete
```

**Backup Verification**:
```bash
# Test backup restoration
pg_dump $DATABASE_URL > test_backup.sql
dropdb test_db
createdb test_db
psql test_db < test_backup.sql
```

## Performance Optimization

### CDN Configuration

**Cloudflare Setup**:
```javascript
// Cloudflare Worker for API caching
addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  const cache = caches.default;
  const cacheKey = new Request(request.url, request);
  
  // Check cache first
  let response = await cache.match(cacheKey);
  
  if (!response) {
    // Not in cache, fetch from origin
    response = await fetch(request);
    
    // Cache GET requests for 5 minutes
    if (request.method === 'GET') {
      const modifiedResponse = new Response(response.body, {
        status: response.status,
        statusText: response.statusText,
        headers: {
          ...response.headers,
          'Cache-Control': 'public, max-age=300',
        },
      });
      
      event.waitUntil(cache.put(cacheKey, modifiedResponse.clone()));
      return modifiedResponse;
    }
  }
  
  return response;
}
```

### Database Optimization

**Connection Pooling**:
```rust
use sqlx::postgres::PgPoolOptions;

pub async fn create_db_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(100)
        .min_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
}
```

## What's Next?

You now have a solid understanding of infrastructure and deployment. In the final chapter, we'll put everything together by walking through the complete development of the GroceryNana application, from initial idea to production deployment.

---

**Next**: [Chapter 8: Real-world Application](08-real-world-application.md)
**Previous**: [Chapter 6: Testing and Quality](06-testing-and-quality.md)