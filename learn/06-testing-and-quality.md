# Chapter 6: Testing and Quality

## Building Reliable Software with LLMs

Testing is crucial for maintaining code quality and ensuring your vibe coding projects work reliably. This chapter covers automated testing, code quality practices, and security considerations.

## Testing Philosophy in Vibe Coding

### Test-Driven Development (TDD) with LLMs

**The TDD Cycle**:
1. **Red**: Write a failing test
2. **Green**: Write minimal code to pass
3. **Refactor**: Improve code while keeping tests passing

**With LLMs**:
```
"Write a test for a function that validates grocery list items. The function should ensure items have a name and quantity > 0."
```

Then:
```
"Now implement the function that makes this test pass."
```

### Testing Pyramid

**Unit Tests (70%)**:
- Test individual functions/components
- Fast and isolated
- High coverage of business logic

**Integration Tests (20%)**:
- Test component interactions
- API endpoint testing
- Database integration

**End-to-End Tests (10%)**:
- Test complete user workflows
- Browser automation
- Critical user journeys

## Frontend Testing

### Component Testing with React Testing Library

**Basic Component Test**:
```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { GroceryListForm } from './GroceryListForm';

describe('GroceryListForm', () => {
  it('should add item when form is submitted', async () => {
    const onSubmit = jest.fn();
    render(<GroceryListForm onSubmit={onSubmit} />);
    
    const nameInput = screen.getByLabelText('Item Name');
    const quantityInput = screen.getByLabelText('Quantity');
    const submitButton = screen.getByRole('button', { name: 'Add Item' });
    
    fireEvent.change(nameInput, { target: { value: 'Apples' } });
    fireEvent.change(quantityInput, { target: { value: '5' } });
    fireEvent.click(submitButton);
    
    expect(onSubmit).toHaveBeenCalledWith({
      name: 'Apples',
      quantity: 5
    });
  });
});
```

### Testing Hooks and State Management

**Custom Hook Testing**:
```typescript
import { renderHook, act } from '@testing-library/react';
import { useGroceryList } from './useGroceryList';

describe('useGroceryList', () => {
  it('should add items to the list', () => {
    const { result } = renderHook(() => useGroceryList());
    
    act(() => {
      result.current.addItem({ name: 'Bread', quantity: 1 });
    });
    
    expect(result.current.items).toHaveLength(1);
    expect(result.current.items[0].name).toBe('Bread');
  });
});
```

### API Integration Testing

**Mock API Calls**:
```typescript
import { rest } from 'msw';
import { setupServer } from 'msw/node';

const server = setupServer(
  rest.get('/api/grocery-lists', (req, res, ctx) => {
    return res(ctx.json([
      { id: '1', name: 'Weekly Shopping', items: [] }
    ]));
  })
);

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());
```

## Backend Testing

### Unit Testing in Rust

**Function Testing**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_grocery_item() {
        let valid_item = GroceryItem {
            name: "Apples".to_string(),
            quantity: 5,
            notes: None,
        };
        
        assert!(validate_grocery_item(&valid_item).is_ok());
    }
    
    #[test]
    fn test_validate_grocery_item_empty_name() {
        let invalid_item = GroceryItem {
            name: "".to_string(),
            quantity: 5,
            notes: None,
        };
        
        assert!(validate_grocery_item(&invalid_item).is_err());
    }
}
```

### Integration Testing

**API Endpoint Testing**:
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use actix_web::{test, App};
    
    #[actix_web::test]
    async fn test_create_grocery_list() {
        let app = test::init_service(
            App::new().route("/api/grocery-lists", web::post().to(create_grocery_list))
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/api/grocery-lists")
            .set_json(&CreateGroceryListRequest {
                name: "Test List".to_string(),
            })
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);
    }
}
```

### Database Testing

**Test Database Setup**:
```rust
use sqlx::SqlitePool;

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn test_create_user() {
    let pool = setup_test_db().await;
    let user_service = UserService::new(pool);
    
    let user = user_service.create_user(CreateUserRequest {
        phone: "+1234567890".to_string(),
    }).await.unwrap();
    
    assert_eq!(user.phone, "+1234567890");
}
```

## End-to-End Testing

### Playwright Setup

**E2E Test Example**:
```typescript
import { test, expect } from '@playwright/test';

test.describe('Grocery List App', () => {
  test('should create and manage grocery list', async ({ page }) => {
    await page.goto('/');
    
    // Register/login
    await page.fill('[data-testid="phone-input"]', '+1234567890');
    await page.click('[data-testid="register-button"]');
    
    // Create grocery list
    await page.fill('[data-testid="list-name"]', 'Weekly Shopping');
    await page.click('[data-testid="create-list-button"]');
    
    // Add items
    await page.fill('[data-testid="item-name"]', 'Apples');
    await page.fill('[data-testid="item-quantity"]', '5');
    await page.click('[data-testid="add-item-button"]');
    
    // Verify item was added
    await expect(page.locator('[data-testid="grocery-item"]')).toContainText('Apples');
  });
});
```

### Test Data Management

**Test Fixtures**:
```typescript
export const testUsers = {
  buyer: {
    phone: '+1234567890',
    verified: true,
    role: 'buyer'
  },
  seller: {
    phone: '+0987654321',
    verified: true,
    role: 'seller'
  }
};

export const testGroceryLists = [
  {
    id: '1',
    name: 'Weekly Shopping',
    items: [
      { name: 'Apples', quantity: 5 },
      { name: 'Bread', quantity: 1 }
    ]
  }
];
```

## Code Quality and Static Analysis

### TypeScript Configuration

**Strict TypeScript Setup**:
```json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "noImplicitReturns": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "exactOptionalPropertyTypes": true
  }
}
```

### ESLint Configuration

**Comprehensive ESLint Setup**:
```javascript
module.exports = {
  extends: [
    'eslint:recommended',
    '@typescript-eslint/recommended',
    'plugin:react/recommended',
    'plugin:react-hooks/recommended',
    'prettier'
  ],
  rules: {
    'no-unused-vars': 'error',
    'no-console': 'warn',
    'prefer-const': 'error',
    'no-var': 'error',
    'eqeqeq': 'error',
    'curly': 'error'
  }
};
```

### Rust Quality Tools

**Clippy Configuration**:
```toml
# .clippy.toml
cognitive-complexity-threshold = 30
too-many-lines-threshold = 100
type-complexity-threshold = 250
```

## Security Testing

### Input Validation Testing

**Frontend Validation**:
```typescript
import { z } from 'zod';

const GroceryItemSchema = z.object({
  name: z.string().min(1).max(100),
  quantity: z.number().min(1).max(1000),
  notes: z.string().optional()
});

describe('Input Validation', () => {
  it('should reject invalid grocery items', () => {
    const invalidItem = {
      name: '',
      quantity: -1,
      notes: 'x'.repeat(1000)
    };
    
    expect(() => GroceryItemSchema.parse(invalidItem)).toThrow();
  });
});
```

**Backend Security Testing**:
```rust
#[actix_web::test]
async fn test_sql_injection_protection() {
    let app = create_test_app().await;
    
    let malicious_input = "'; DROP TABLE users; --";
    let req = test::TestRequest::post()
        .uri("/api/grocery-lists")
        .set_json(&CreateGroceryListRequest {
            name: malicious_input.to_string(),
        })
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    // Should handle malicious input safely
    assert!(resp.status().is_success() || resp.status().is_client_error());
    
    // Verify database integrity
    let lists = get_all_grocery_lists(&pool).await.unwrap();
    assert!(lists.iter().any(|l| l.name == malicious_input));
}
```

## Performance Testing

### Load Testing

**Artillery Configuration**:
```yaml
# artillery.yml
config:
  target: 'http://localhost:8080'
  phases:
    - duration: 60
      arrivalRate: 10
      name: "Warm up"
    - duration: 120
      arrivalRate: 50
      name: "Load test"

scenarios:
  - name: "Create grocery list"
    flow:
      - post:
          url: "/api/grocery-lists"
          json:
            name: "Test List"
      - think: 1
```

### Performance Monitoring

**Frontend Performance**:
```typescript
// Web Vitals monitoring
import { getCLS, getFID, getFCP, getLCP, getTTFB } from 'web-vitals';

function sendToAnalytics(metric: any) {
  // Send to your analytics service
  console.log(metric);
}

getCLS(sendToAnalytics);
getFID(sendToAnalytics);
getFCP(sendToAnalytics);
getLCP(sendToAnalytics);
getTTFB(sendToAnalytics);
```

## Test Automation and CI/CD

### GitHub Actions Testing

**Complete Test Pipeline**:
```yaml
name: Test Suite

on: [push, pull_request]

jobs:
  frontend-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
    - name: Install dependencies
      run: cd webapp && npm ci
    - name: Run unit tests
      run: cd webapp && npm test
    - name: Run E2E tests
      run: cd webapp && npm run test:e2e

  backend-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run tests
      run: cd backend && cargo test
    - name: Run security audit
      run: cd backend && cargo audit

  integration-tests:
    runs-on: ubuntu-latest
    needs: [frontend-tests, backend-tests]
    steps:
    - uses: actions/checkout@v3
    - name: Start services
      run: docker-compose up -d
    - name: Run integration tests
      run: npm run test:integration
```

## Best Practices Summary

### Testing Strategy
- ✅ Write tests before or alongside code
- ✅ Test behavior, not implementation
- ✅ Use descriptive test names
- ✅ Keep tests independent and fast
- ✅ Test edge cases and error conditions

### Code Quality
- ✅ Use static analysis tools
- ✅ Enforce consistent formatting
- ✅ Set up pre-commit hooks
- ✅ Monitor code coverage
- ✅ Regular security audits

### Security
- ✅ Validate all inputs
- ✅ Use parameterized queries
- ✅ Implement rate limiting
- ✅ Test authentication/authorization
- ✅ Regular dependency updates

## What's Next?

Now that you understand testing and quality practices, you're ready to learn about infrastructure and deployment. In the next chapter, we'll explore cloud services, deployment strategies, and production monitoring.

---

**Next**: [Chapter 7: Infrastructure and Deployment](07-infrastructure-deployment.md)
**Previous**: [Chapter 5: Development Workflow](05-development-workflow.md)