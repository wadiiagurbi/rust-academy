import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const routes = [
  {
    path: '/',
    name: 'dashboard',
    component: () => import('../views/DashboardView.vue'),
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('../views/LoginView.vue'),
    meta: { guestOnly: true },
  },
  {
    path: '/register',
    name: 'register',
    component: () => import('../views/RegisterView.vue'),
    meta: { guestOnly: true },
  },
  {
    path: '/search',
    name: 'search',
    component: () => import('../views/SearchView.vue'),
  },
  {
    path: '/lessons/:slug',
    name: 'lesson',
    component: () => import('../views/LessonView.vue'),
    props: true,
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'not-found',
    component: () => import('../views/NotFoundView.vue'),
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    }
    if (to.hash) {
      return { el: to.hash, top: 84 }
    }
    return { top: 0 }
  },
})

router.beforeEach(async (to) => {
  const authStore = useAuthStore()

  if (!authStore.ready) {
    try {
      await authStore.init()
    } catch {
      // init() already handles failure internally; continue with guards below.
    }
  }

  if (to.meta.guestOnly && authStore.isAuthenticated) {
    return { name: 'dashboard' }
  }

  return true
})

// A lazy-loaded route chunk can fail to fetch (dev server restart, flaky
// network, or a stale chunk hash after a new deploy). Vue Router leaves that
// error uncaught, which aborts the initial navigation and renders nothing —
// a permanently blank page. Reload once to recover; guard with sessionStorage
// so a persistently broken chunk doesn't reload-loop forever.
router.onError((error, to) => {
  const isChunkLoadError = /Failed to fetch dynamically imported module|error loading dynamically imported module|Importing a module script failed/i.test(error.message)
  const reloadKey = 'academy-chunk-reload'

  try {
    if (isChunkLoadError && !sessionStorage.getItem(reloadKey)) {
      sessionStorage.setItem(reloadKey, '1')
      window.location.href = to.fullPath
    }
  } catch {
    // sessionStorage unavailable — cannot guard against a reload loop, so
    // leave the page as-is rather than risk reloading forever.
  }
})

router.afterEach(() => {
  try {
    sessionStorage.removeItem('academy-chunk-reload')
  } catch {
    // sessionStorage unavailable — nothing to clean up.
  }
})

export default router
