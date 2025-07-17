# Chapter 3: Working with LLMs

## Mastering LLM Collaboration

Working effectively with LLMs is the core skill of vibe coding. This chapter will teach you how to communicate with LLMs to get the best results for your software development projects.

## Understanding LLM Capabilities

### What LLMs Excel At

**Code Generation**:
- Writing boilerplate code
- Implementing algorithms
- Creating API endpoints
- Building UI components

**Problem Solving**:
- Debugging issues
- Suggesting architectures
- Explaining complex concepts
- Optimizing performance

**Documentation**:
- Writing clear explanations
- Creating user guides
- Generating API documentation
- Code comments

### LLM Limitations

**Context Limitations**:
- Limited memory of previous conversations
- Can't access external systems directly
- May lose track of complex state

**Accuracy Considerations**:
- May generate plausible but incorrect code
- Can hallucinate APIs or libraries
- Needs human validation

## Effective Communication Strategies

### The Art of Prompting

**Clear and Specific**:
```
❌ "Make a website"
✅ "Create a React component for a grocery list form with fields for item name, quantity, and notes"
```

**Provide Context**:
```
✅ "I'm building a grocery marketplace app using React and TypeScript. The backend is Rust with Actix Web. Create a component that..."
```

**Include Examples**:
```
✅ "Create a TypeScript interface for a grocery item. It should have properties like:
- name: string
- quantity: number
- notes?: string (optional)
- category: 'produce' | 'dairy' | 'meat' | 'other'
```

### Progressive Refinement

Start broad, then narrow down:

1. **Initial Request**: "Help me design a user authentication system"
2. **Add Requirements**: "It should use phone numbers and SMS verification"
3. **Specify Technology**: "Using Rust, Actix Web, and SQLite"
4. **Implementation Details**: "Show me the database schema and API endpoints"

## Common Patterns and Techniques

### The Explain-Before-Code Pattern

Ask the LLM to explain the approach before implementing:

```
"Before writing the code, explain how you would implement user authentication with phone verification. What are the key steps and considerations?"
```

### The Review-and-Improve Pattern

Have the LLM review its own work:

```
"Review the authentication code you just wrote. Are there any security issues or improvements you'd suggest?"
```

### The Alternative-Approach Pattern

Get multiple perspectives:

```
"Show me two different ways to implement real-time messaging: one using WebSockets and another using Server-Sent Events. What are the pros and cons of each?"
```

## Debugging with LLMs

### Effective Error Sharing

When you encounter errors, provide:

1. **Full Error Message**: Include stack traces and line numbers
2. **Relevant Code**: The code that's causing the issue
3. **Context**: What you were trying to achieve
4. **Environment**: Language version, dependencies, etc.

**Example**:
```
I'm getting this error in my Rust Actix Web application:

Error: "thread 'main' panicked at src/main.rs:53:10: Failed to create database pool"

Here's the relevant code:
[code snippet]

I'm trying to connect to SQLite database for my grocery app. The database file exists and the connection string is correct.
```

### Debugging Workflow

1. **Describe the Problem**: What's not working?
2. **Share the Error**: Include full error messages
3. **Provide Context**: What were you trying to do?
4. **Try Solutions**: Implement suggested fixes
5. **Iterate**: If the first solution doesn't work, try alternatives

## Code Review and Quality

### Getting Better Code

**Ask for Best Practices**:
```
"Write a secure password hashing function in Rust. Follow security best practices."
```

**Request Specific Patterns**:
```
"Implement this using the Repository pattern for better testability."
```

**Consider Performance**:
```
"Optimize this database query for better performance with large datasets."
```

### Code Review Checklist

Ask the LLM to check for:
- ✅ Security vulnerabilities
- ✅ Performance issues
- ✅ Error handling
- ✅ Code style and conventions
- ✅ Testing considerations

## Testing with LLMs

### Test Generation

**Unit Tests**:
```
"Generate unit tests for this user authentication function. Include happy path, error cases, and edge cases."
```

**Integration Tests**:
```
"Create integration tests for the grocery list API endpoints. Test CRUD operations and error handling."
```

**Mock Data**:
```
"Generate realistic mock data for testing a grocery marketplace app. Include users, lists, and offers."
```

### Test-Driven Development

Use LLMs to practice TDD:

1. **Write Tests First**: "Create tests for a function that validates grocery list items"
2. **Implement Code**: "Now implement the function that makes these tests pass"
3. **Refactor**: "Improve the implementation while keeping tests passing"

## Documentation and Learning

### Getting Explanations

**Concept Clarification**:
```
"Explain how JWT tokens work in the context of user authentication. What are the security considerations?"
```

**Technology Deep Dives**:
```
"Compare SQLite vs PostgreSQL for a grocery marketplace app. What are the trade-offs?"
```

### Documentation Generation

**API Documentation**:
```
"Generate OpenAPI documentation for these REST endpoints."
```

**User Guides**:
```
"Create a user guide for the grocery list feature. Include screenshots and step-by-step instructions."
```

## Advanced Techniques

### Context Management

**Session Summaries**:
```
"Summarize what we've built so far in our grocery marketplace app. What are the key components and how do they interact?"
```

**State Tracking**:
```
"Based on our previous conversation, what's the current state of the authentication system? What still needs to be implemented?"
```

### Multi-Step Workflows

**Planning Sessions**:
```
"Let's plan the implementation of real-time messaging. Break it down into steps and help me implement each one."
```

**Iterative Development**:
```
"Let's start with a basic version of the messaging system, then incrementally add features."
```

## Common Pitfalls and Solutions

### Pitfall 1: Too Vague Requests

**Problem**: "Make the app better"
**Solution**: "Improve the grocery list UI by adding drag-and-drop reordering and better visual feedback"

### Pitfall 2: Ignoring Context

**Problem**: Asking for code without explaining the existing architecture
**Solution**: Always provide relevant context about your project structure

### Pitfall 3: Not Validating Results

**Problem**: Accepting generated code without testing
**Solution**: Always test and verify LLM-generated code

### Pitfall 4: Overwhelming Requests

**Problem**: Asking for too much at once
**Solution**: Break complex requests into smaller, manageable pieces

## Best Practices Summary

### Do's
- ✅ Be specific and provide context
- ✅ Ask for explanations before implementations
- ✅ Request code reviews and improvements
- ✅ Test and validate all generated code
- ✅ Iterate and refine based on results

### Don'ts
- ❌ Make vague, open-ended requests
- ❌ Skip testing generated code
- ❌ Ignore security considerations
- ❌ Accept the first solution without questioning
- ❌ Forget to provide project context

## Tools and Environment

### LLM Development Tools
- **Claude Code**: Full development environment
- **GitHub Copilot**: In-editor assistance
- **ChatGPT**: General programming help
- **Cursor**: AI-powered code editor

### Supporting Tools
- **Version Control**: Git for tracking changes
- **Testing Frameworks**: For validating generated code
- **Documentation Tools**: For maintaining project knowledge
- **Debugging Tools**: For troubleshooting issues

## What's Next?

Now that you know how to work effectively with LLMs, you're ready to understand the technical foundation of modern software development. In the next chapter, we'll explore full-stack architecture and the technologies that power modern applications.

---

**Next**: [Chapter 4: Technical Foundation](04-technical-foundation.md)
**Previous**: [Chapter 2: From Idea to Tasks](02-from-idea-to-tasks.md)