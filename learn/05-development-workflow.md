# Chapter 5: Development Workflow

## Mastering Git and Development Workflows

A solid development workflow is essential for successful vibe coding. This chapter covers Git workflows, branching strategies, and continuous integration practices that will keep your projects organized and maintainable.

## Git Fundamentals for Vibe Coding

### The Vibe Coding Git Philosophy

In vibe coding, we embrace:
- **Frequent commits**: Commit often to track progress
- **Descriptive messages**: Clear history of what was built
- **Feature branches**: Isolate work and enable collaboration
- **Clean history**: Organize commits for future reference

### Essential Git Commands

**Daily Workflow**:
```bash
# Check current status
git status

# Stage changes
git add .
git add specific-file.ts

# Commit with message
git commit -m "Add user authentication endpoint"

# Push to remote
git push origin feature/user-auth

# Pull latest changes
git pull origin main
```

**Branch Management**:
```bash
# Create and switch to new branch
git checkout -b feature/grocery-lists

# Switch between branches
git checkout main
git checkout feature/grocery-lists

# Delete branch
git branch -d feature/grocery-lists
```

## Branching Strategies

### Feature Branch Workflow

**Main Branch**:
- Always deployable
- Only contains tested, working code
- Protected from direct commits

**Feature Branches**:
- One branch per feature/ticket
- Branched from main
- Merged back via pull request

**Example Workflow**:
```bash
# Start new feature
git checkout main
git pull origin main
git checkout -b feature/user-registration

# Work on feature
# ... make changes ...
git add .
git commit -m "Add user registration form"

# Push feature branch
git push -u origin feature/user-registration

# Create pull request
gh pr create --title "Add user registration" --body "Implements user registration with phone verification"
```

### Branch Naming Conventions

**Feature Branches**:
- `feature/user-authentication`
- `feature/grocery-list-crud`
- `feature/real-time-messaging`

**Bug Fixes**:
- `fix/authentication-redirect`
- `fix/grocery-list-validation`

**Chores/Maintenance**:
- `chore/update-dependencies`
- `chore/improve-documentation`

## Commit Message Best Practices

### Conventional Commits

Follow this format:
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(auth): add phone number verification

Implement SMS-based phone verification using Twilio.
Users must verify their phone number before account activation.

Closes #123
```

```
fix(api): handle empty grocery list validation

Add validation for empty grocery lists to prevent
server errors when users submit lists without items.

Fixes #456
```

## Pull Request Workflow

### Creating Effective Pull Requests

**PR Template**:
```markdown
## Overview
Brief description of what this PR does.

## Changes
- [ ] Added user authentication endpoints
- [ ] Implemented phone verification
- [ ] Added input validation
- [ ] Updated documentation

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing completed

## Screenshots
[If applicable, add screenshots]

## Notes
Any additional context or considerations.
```

### PR Review Process

**Before Creating PR**:
1. Test your changes locally
2. Run linting and formatting
3. Update documentation if needed
4. Write clear PR description

**During Review**:
1. Respond to feedback promptly
2. Make requested changes
3. Ask for clarification if needed
4. Keep discussions focused

**After Approval**:
1. Merge the PR
2. Delete the feature branch
3. Update local main branch

## Continuous Integration (CI)

### GitHub Actions Workflow

**Basic CI Configuration** (`.github/workflows/ci.yml`):
```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        
    - name: Install dependencies
      run: npm ci
      
    - name: Run tests
      run: npm test
      
    - name: Run linting
      run: npm run lint
      
    - name: Run type checking
      run: npm run typecheck
```

**Multi-Language CI** (Frontend + Backend):
```yaml
name: Full Stack CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  frontend:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
    - name: Install dependencies
      run: cd webapp && npm ci
    - name: Run tests
      run: cd webapp && npm test
    - name: Build
      run: cd webapp && npm run build

  backend:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run tests
      run: cd backend && cargo test
    - name: Run clippy
      run: cd backend && cargo clippy
    - name: Check formatting
      run: cd backend && cargo fmt --check
```

## Code Quality Tools

### Linting and Formatting

**Frontend (ESLint + Prettier)**:
```json
// .eslintrc.js
module.exports = {
  extends: [
    'eslint:recommended',
    '@typescript-eslint/recommended',
    'prettier'
  ],
  rules: {
    'no-unused-vars': 'error',
    'no-console': 'warn',
    'prefer-const': 'error'
  }
}
```

**Backend (Rust)**:
```toml
# .clippy.toml
disallowed-types = [
    "std::collections::HashMap",
    "std::collections::BTreeMap",
]
```

### Pre-commit Hooks

**Setup with Husky**:
```json
// package.json
{
  "scripts": {
    "prepare": "husky install"
  },
  "lint-staged": {
    "*.{ts,tsx}": ["eslint --fix", "prettier --write"],
    "*.{js,jsx}": ["eslint --fix", "prettier --write"]
  }
}
```

**Git Hook Configuration**:
```bash
# .husky/pre-commit
#!/usr/bin/env sh
. "$(dirname -- "$0")/_/husky.sh"

npm run lint
npm run typecheck
npm run test
```

## Development Environment Setup

### Local Development Workflow

**Development Scripts**:
```json
// package.json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "test": "vitest",
    "test:watch": "vitest --watch",
    "lint": "eslint src --ext ts,tsx",
    "lint:fix": "eslint src --ext ts,tsx --fix",
    "typecheck": "tsc --noEmit"
  }
}
```

**Hot Reloading Setup**:
```typescript
// vite.config.ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': 'http://localhost:8080'
    }
  }
})
```

### Docker Development Environment

**Multi-Service Setup**:
```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  frontend:
    build: ./webapp
    ports:
      - "3000:3000"
    volumes:
      - ./webapp:/app
      - /app/node_modules
    environment:
      - VITE_API_URL=http://localhost:8080

  backend:
    build: ./backend
    ports:
      - "8080:8080"
    volumes:
      - ./backend:/app
    environment:
      - DATABASE_URL=sqlite:./database.db
      - RUST_LOG=debug

  database:
    image: postgres:15
    environment:
      - POSTGRES_DB=grocerynana
      - POSTGRES_USER=dev
      - POSTGRES_PASSWORD=dev
    ports:
      - "5432:5432"
```

## Release Management

### Semantic Versioning

**Version Format**: `MAJOR.MINOR.PATCH`
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

**Release Process**:
```bash
# Create release branch
git checkout -b release/v1.2.0

# Update version numbers
npm version minor

# Create release PR
gh pr create --title "Release v1.2.0" --body "Release notes..."

# After merge, create tag
git tag v1.2.0
git push origin v1.2.0
```

### Automated Releases

**GitHub Actions Release**:
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Create Release
      uses: actions/create-release@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        tag_name: ${{ github.ref }}
        release_name: Release ${{ github.ref }}
        draft: false
        prerelease: false
```

## Collaboration Best Practices

### Code Review Guidelines

**For Authors**:
- Keep PRs small and focused
- Write clear descriptions
- Respond to feedback constructively
- Test changes thoroughly

**For Reviewers**:
- Review promptly
- Be constructive and specific
- Focus on code quality and maintainability
- Ask questions when unclear

### Communication

**Commit Messages**:
- Use clear, descriptive messages
- Reference issue numbers
- Explain the "why" not just the "what"

**PR Descriptions**:
- Summarize changes clearly
- Include testing instructions
- Note any breaking changes
- Link to related issues

## Common Workflow Patterns

### Feature Development

1. **Planning**: Create GitHub issue
2. **Branch**: Create feature branch
3. **Develop**: Implement feature with frequent commits
4. **Test**: Write and run tests
5. **Review**: Create PR and get feedback
6. **Merge**: Merge to main after approval
7. **Deploy**: Automatic deployment from main

### Bug Fixes

1. **Reproduce**: Confirm the bug
2. **Branch**: Create fix branch
3. **Fix**: Implement minimal fix
4. **Test**: Verify fix works
5. **Regression**: Ensure no side effects
6. **Deploy**: Fast-track for critical bugs

### Hotfixes

1. **Branch**: From main branch
2. **Fix**: Minimal change to resolve issue
3. **Test**: Quick but thorough testing
4. **Review**: Expedited review process
5. **Deploy**: Immediate deployment
6. **Backport**: Apply to relevant branches

## What's Next?

Now that you understand development workflows, you're ready to learn about testing and quality assurance. In the next chapter, we'll explore automated testing, code quality, and security best practices.

---

**Next**: [Chapter 6: Testing and Quality](06-testing-and-quality.md)
**Previous**: [Chapter 4: Technical Foundation](04-technical-foundation.md)