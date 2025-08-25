# Technical Stack & Architecture

**Last Updated:** [Date]  
**Template:** React Router SPA

## Core Technologies

### Frontend Framework
```json
{
  "react": "^18.2.0",
  "react-dom": "^18.2.0",
  "react-router-dom": "^6.22.0"
}
```

### Build Tool
```json
{
  "vite": "^5.1.0",
  "@vitejs/plugin-react": "^4.2.1"
}
```

### Development Dependencies
```json
{
  "@types/react": "^18.2.55",
  "@types/react-dom": "^18.2.19",
  "@typescript-eslint/eslint-plugin": "^6.21.0",
  "@typescript-eslint/parser": "^6.21.0",
  "eslint": "^8.56.0",
  "eslint-plugin-react-hooks": "^4.6.0",
  "eslint-plugin-react-refresh": "^0.4.5",
  "typescript": "^5.2.2"
}
```

## Project Structure
```
src/
├── components/          # Reusable UI components
│   ├── common/         # Shared components (Button, Input, etc.)
│   └── layout/         # Layout components (Header, Footer, etc.)
├── pages/              # Route-based page components
├── hooks/              # Custom React hooks
├── utils/              # Utility functions
├── services/           # API and external service integrations
├── styles/             # Global styles and theme
├── assets/             # Images, fonts, and static files
├── App.tsx             # Main app component with routing
└── main.tsx            # Application entry point
```

## Routing Architecture

### Route Configuration
```typescript
// Current routes structure
const routes = [
  { path: '/', element: <Home /> },
]
```

### Route Guards
- Public routes: Accessible without authentication
- Protected routes: Require authentication
- Role-based routes: Require specific permissions

## State Management

### Current Approach
- React Context API for global state
- Local component state for UI state
- Custom hooks for shared logic

### State Structure
```typescript
interface AppState {
  user: User | null;
  theme: 'light' | 'dark';
  // Add more global state here
}
```

## Styling Approach

### CSS Modules
- Component-scoped styles
- File naming: `Component.module.css`

### Global Styles
- Reset/normalize CSS
- Typography scale
- Color variables
- Spacing system

## Data Layer

### Cloudflare D1 Database
This project uses Cloudflare D1 for both local development and production, providing a consistent SQLite-based database experience.

#### Prerequisites
- Wrangler CLI (comes with Cloudflare Workers/Pages projects)

#### Quick Setup
```bash
# 1. Edit schema.sql with your table definitions
# 2. Create database with tables in one command
npx wrangler d1 execute my-database --local --file=./schema.sql

# 3. Start development server
npm run dev
```

The database is created automatically when you first execute SQL - no separate initialization needed!

#### Database Configuration
To use D1, add this to your `wrangler.jsonc` if not already present:

```jsonc
{
  // ... other config
  "d1_databases": [
    {
      "binding": "DB",
      "database_name": "my-database", 
      "database_id": "your-database-id-here"
    }
  ]
}
```

The project includes:
- **schema.sql**: Template for your database tables
- **wrangler.jsonc**: Configuration for D1 bindings (add manually if needed)

#### Schema Structure
The project includes an empty `schema.sql` file where you can define your tables:

```sql
-- Example table structure
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  email TEXT UNIQUE NOT NULL,
  name TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### Database Operations

**Best Practice: Always Use SQL Files**
Never use inline commands for schema changes. Always use SQL files for production compatibility:

```bash
# ✅ CORRECT: Use SQL files for everything
wrangler d1 execute my-database --local --file=./schema.sql
wrangler d1 execute my-database --file=./schema.sql  # production

# ✅ CORRECT: Use migration files for changes
wrangler d1 execute my-database --local --file=./migrations/001_add_users.sql
wrangler d1 execute my-database --file=./migrations/001_add_users.sql  # production

# ❌ AVOID: Inline commands (not version controlled, not reproducible)
wrangler d1 execute my-database --local --command="CREATE TABLE..."
```

**Migration File Structure:**
```
/
├── schema.sql              # Initial schema
├── migrations/
│   ├── 001_add_users.sql   # First migration
│   ├── 002_add_posts.sql   # Second migration
│   └── 003_add_indexes.sql # Third migration
└── queries/
    ├── get_stats.sql       # Reusable queries
    └── cleanup.sql         # Maintenance queries
```

**Local Development:**
```bash
# List all tables
wrangler d1 execute my-database --local --file=./schema.sql

# For quick queries (prefer SQL files for anything complex)
wrangler d1 execute my-database --local --command="PRAGMA table_list;"

# Interactive shell (for debugging only)
wrangler d1 execute my-database --local
```

**Production Deployment:**
```bash
# Deploy initial schema
wrangler d1 execute my-database --file=./schema.sql

# Deploy migrations in order (critical: same files as local)
wrangler d1 execute my-database --file=./migrations/001_add_users.sql
wrangler d1 execute my-database --file=./migrations/002_add_posts.sql
wrangler d1 execute my-database --file=./migrations/003_add_indexes.sql

# Run production maintenance
wrangler d1 execute my-database --file=./queries/production_stats.sql
```

**Why SQL Files Are Essential:**
- ✅ **Version Control** - Track all database changes in Git
- ✅ **Reproducible** - Same exact schema in local, staging, production
- ✅ **Team Collaboration** - Review database changes in PRs
- ✅ **Rollback Safe** - Can easily revert problematic changes
- ✅ **CI/CD Ready** - Automated deployments use same files
- ✅ **No Human Error** - No typos in production commands

**⚠️ Important: Local vs Cloud D1**
Local D1 (`--local`) and Cloud D1 are separate databases:
- **Local**: Uses SQLite file on your machine
- **Cloud**: Uses Cloudflare D1 in production
- **No automatic sync** - You must deploy schema files to both

#### Using D1 in Your App
```typescript
// In your React Router loaders/actions
export async function loader({ context }: LoaderFunctionArgs) {
  const { env } = context;
  
  // Query users
  const users = await env.DB.prepare("SELECT * FROM users").all();
  
  return json({ users: users.results });
}

export async function action({ request, context }: ActionFunctionArgs) {
  const { env } = context;
  const formData = await request.formData();
  
  // Insert new user
  await env.DB.prepare("INSERT INTO users (email, name) VALUES (?, ?)")
    .bind(formData.get("email"), formData.get("name"))
    .run();
  
  return redirect("/users");
}
```

#### Benefits of D1
- **Consistent**: Same database for local dev and production  
- **Fast**: SQLite performance at the edge
- **Scalable**: Automatically scales with Cloudflare's network
- **Simple**: No connection pooling or server management
- **Cost-effective**: Pay only for reads/writes

## API Integration

### React Router Data Loading
This project uses React Router's built-in data loading instead of external API calls:

```typescript
// app/routes/users.tsx
import type { LoaderFunctionArgs, ActionFunctionArgs } from '@react-router/cloudflare';
import { json, redirect } from '@react-router/cloudflare';

// Load data on route entry
export async function loader({ context }: LoaderFunctionArgs) {
  const { env } = context;
  
  const users = await env.DB.prepare("SELECT * FROM users ORDER BY created_at DESC").all();
  return json({ users: users.results });
}

// Handle form submissions and mutations
export async function action({ request, context }: ActionFunctionArgs) {
  const { env } = context;
  const formData = await request.formData();
  
  switch (formData.get("_action")) {
    case "create":
      await env.DB.prepare("INSERT INTO users (email, name) VALUES (?, ?)")
        .bind(formData.get("email"), formData.get("name"))
        .run();
      return redirect("/users");
    
    case "delete":
      await env.DB.prepare("DELETE FROM users WHERE id = ?")
        .bind(formData.get("id"))
        .run();
      return redirect("/users");
    
    default:
      return json({ error: "Invalid action" }, { status: 400 });
  }
}
```

### External API Integration (if needed)
For third-party APIs or services outside your Cloudflare environment:

```typescript
// lib/api.ts
export const externalApi = {
  async fetchUserProfile(userId: string) {
    const response = await fetch(`https://api.external.com/users/${userId}`, {
      headers: {
        'Authorization': `Bearer ${process.env.EXTERNAL_API_TOKEN}`,
        'Content-Type': 'application/json',
      }
    });
    return response.json();
  },
  
  async sendNotification(email: string, message: string) {
    return fetch('https://api.emailservice.com/send', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, message })
    });
  }
};

// Use in loaders/actions
export async function loader({ params, context }: LoaderFunctionArgs) {
  const [user, profile] = await Promise.all([
    context.env.DB.prepare("SELECT * FROM users WHERE id = ?").bind(params.id).first(),
    externalApi.fetchUserProfile(params.id)
  ]);
  
  return json({ user, profile });
}
```

## Build Configuration

### Vite Config
```typescript
// vite.config.ts
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
      '/api': 'http://localhost:3000'
    }
  },
  build: {
    outDir: 'dist',
    sourcemap: true
  }
})
```

## Testing Setup

### Test Framework
- Vitest for unit tests
- React Testing Library for component tests
- Playwright for E2E tests

### Test Structure
```
src/
├── __tests__/          # Test files
├── setupTests.ts       # Test configuration
└── test-utils.tsx      # Test utilities
```

### Output Structure
```
dist/
├── index.html
├── assets/
│   ├── index-[hash].js
│   └── index-[hash].css
└── favicon.ico
```

### Hosting Requirements
- Static file hosting (Netlify, Vercel, S3, etc.)
- SPA routing support (fallback to index.html)
- HTTPS certificate

---

*This document contains technical implementation details. For product features and design, see PRODUCT_SPEC.md*