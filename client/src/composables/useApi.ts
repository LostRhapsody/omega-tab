// composables/useApi.ts
import { ref } from 'vue'

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'

export function useApi() {
  const loading = ref(false)
  const error = ref<Error | null>(null)

  async function api<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    loading.value = true
    error.value = null

    try {
      const response = await fetch(`${API_URL}${endpoint}`, {
        headers: {
          'Content-Type': 'application/json',
          // Add any default headers here
        },
        ...options,
      })

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }

      return await response.json()
    } catch (e) {
      error.value = e as Error
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    api,
    loading,
    error
  }
}

// examples
// In your component:
// const { api } = useApi()

// // GET request
// const users = await api('/users')

// // POST request
// const newUser = await api('/users', {
//   method: 'POST',
//   body: JSON.stringify({ name: 'John' })
// })

// // PUT request
// const updatedUser = await api(`/users/${id}`, {
//   method: 'PUT',
//   body: JSON.stringify({ name: 'John Updated' })
// })