# AGENTS.md - AI Coding Agent Instructions

## Project Overview

**BetterNewTab** - A browser new tab replacement web application.

- **Frontend (`client/`)**: Vue.js 3 + TypeScript + Vite
- **Backend (`server/`)**: Rust + Axum

## Build/Lint/Test Commands

All commands run from `client/` directory using `bun`:

### Development

```bash
bun start           # Lint + dev server + Tailwind watcher
bun dev             # Vite dev server only
bun dev:all         # Dev server + Tailwind watcher (no lint)
```

### Building

```bash
bun build           # Full production build (lint + tailwind + type-check + build + test)
bun build:all       # Production build without lint/test
bun type-check      # TypeScript type checking only
```

### Linting

```bash
bun lint            # Biome check with auto-fix
```

### Testing

```bash
bun test:unit       # Vitest in watch mode
bun test:run        # Vitest single run

# Run a single test file:
bun vitest run src/tests/stores/user.test.ts

# Run tests matching a pattern:
bun vitest run -t "User Store"

# E2E tests:
bun test:e2e        # Cypress E2E tests
bun test:e2e:dev    # Cypress interactive mode
```

### Backend (Rust)

```bash
cd server
cargo build         # Build
cargo run           # Run dev server
cargo test          # Run tests
cargo clippy        # Linting
```

## Code Style Guidelines

### Formatting (Biome)

- **Indentation**: 2 spaces
- **Quotes**: Double quotes for strings
- **Import organization**: Auto-organized by Biome
- **Files**: Only `.ts` files in `src/` are linted (not `.vue` or `.html`)

### TypeScript Conventions

#### Import Order

1. External packages (vue, pinia, axios, etc.)
2. Internal modules using `@/` alias
3. Type-only imports use `import type`

```typescript
import { defineStore } from "pinia";
import { API } from "@/constants/api";
import api from "@/services/api";
import type { User, UserState } from "@/types/User";
```

#### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Types/Interfaces | PascalCase | `UserState`, `ClerkUser` |
| Functions | camelCase | `fetchUserData`, `setUserId` |
| Constants | camelCase or SCREAMING_SNAKE | `API.GET_USER_DATA` |
| Pinia stores | `use[Name]Store` | `useUserStore` |
| Components | PascalCase | `SearchBar.vue` |
| Composables | `use[Name]` | `useApi` |

#### Type Definitions

- Define types in `src/types/` directory
- Use `type` for object shapes, `interface` for extendable contracts
- Always type function parameters and return values

```typescript
async function fetchUserData(clerk_user: ClerkUser): Promise<boolean> {
  // ...
}
```

### Vue Components

Use Composition API with `<script setup lang="ts">`:

```vue
<template>
  <!-- Template content -->
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useUserStore } from "@/stores/user";
import type { User } from "@/types/User";

const props = defineProps<{
  userId: string;
}>();

const emit = defineEmits<{
  update: [value: string];
}>();
</script>

<style scoped>
/* Component styles */
</style>
```

### Pinia Stores

Use Options API style for stores:

```typescript
export const useUserStore = defineStore("user", {
  state: (): UserState => ({
    userId: null,
    isLoading: false,
    error: null,
  }),

  actions: {
    async fetchUserData(clerk_user: ClerkUser): Promise<boolean> {
      this.isLoading = true;
      try {
        // API call
      } catch (error) {
        this.error = error as string;
        return false;
      } finally {
        this.isLoading = false;
      }
    },
  },
});
```

### Error Handling

#### Frontend Pattern

```typescript
try {
  const response = await api.get<UserDataResponse>(API.GET_USER_DATA);
  if (response.status !== 200) {
    throw new Error(`Failed to fetch, status: ${response.status}`);
  }
  // Handle success
} catch (error) {
  this.error = error as string;
  cache.clear(CacheKeys.USER);
  return false;
} finally {
  this.isLoading = false;
}
```

#### Backend Pattern (Rust)

```rust
let result = some_operation().map_err(|e| {
    tracing::error!("Error: {:?}", e);
    StatusCode::INTERNAL_SERVER_ERROR
})?;
```

### Testing (Vitest)

```typescript
import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

describe("User Store", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it("should have initial state", () => {
    const store = useUserStore();
    expect(store.userId).toBeNull();
  });
});
```

### Rust Conventions

- snake_case for functions and variables
- PascalCase for structs and enums
- Use `tracing` for logging
- Use `anyhow::Result` or `Result<T, StatusCode>` for error handling
- Sentry integration for error monitoring

## Project Structure

```
client/
├── src/
│   ├── assets/css/      # CSS/SCSS files
│   ├── components/      # Vue SFC components
│   ├── composables/     # Vue composables
│   ├── constants/       # API endpoints, config
│   ├── data/            # Static data
│   ├── plugins/         # Vuetify plugin
│   ├── router/          # Vue Router config
│   ├── services/        # API service layer
│   ├── stores/          # Pinia stores
│   ├── tests/           # Test files
│   ├── types/           # TypeScript types
│   ├── utils/           # Utility functions
│   └── views/           # Page components
server/
├── src/
│   ├── main.rs          # Entry + handlers
│   ├── supabase.rs      # Database client
│   ├── stripe_client.rs # Payments
│   └── middleware.rs    # Auth middleware
```

## Key Dependencies

- **Vue 3.5**, **Vue Router 4**, **Pinia 2.3**
- **Vuetify 3.7** for UI components
- **Clerk** for authentication
- **Axios** for HTTP requests
- **Vitest** for unit tests, **Cypress** for E2E
- **Biome** for linting/formatting
