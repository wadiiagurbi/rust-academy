import axios from 'axios'

const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || '/api',
})

export function setAuthToken(token) {
  if (token) {
    apiClient.defaults.headers.common.Authorization = `Bearer ${token}`
  } else {
    delete apiClient.defaults.headers.common.Authorization
  }
}

// On 401 the session is no longer valid: clear auth state and send the user to
// login with a redirect back to where they were. The router/auth store are
// imported dynamically to avoid a static import cycle (auth.js imports this
// module). Auth endpoints are excluded so failed logins don't trigger a
// logout/redirect loop.
apiClient.interceptors.response.use(
  (response) => response,
  async (error) => {
    const status = error.response?.status
    const url = error.config?.url || ''
    const isAuthEndpoint = url.includes('/auth/login') || url.includes('/auth/register')

    if (401 === status && !isAuthEndpoint) {
      try {
        const { useAuthStore } = await import('../stores/auth')
        useAuthStore().logout()
      } catch {
        // Store unavailable — continue to redirect below.
      }

      try {
        const { default: router } = await import('../router')
        const current = router.currentRoute.value
        if ('login' !== current.name) {
          await router.push({ name: 'login', query: { redirect: current.fullPath } })
        }
      } catch {
        // Router unavailable (e.g. outside app context) — fall back to a hard
        // redirect preserving the current path for post-login return.
        if (typeof window !== 'undefined' && !window.location.pathname.startsWith('/login')) {
          const redirect = encodeURIComponent(window.location.pathname + window.location.search)
          window.location.href = `/login?redirect=${redirect}`
        }
      }
    }

    return Promise.reject(error)
  },
)

export default apiClient
