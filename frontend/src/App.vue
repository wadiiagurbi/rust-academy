<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { useDisplay, useTheme } from 'vuetify'
import { useRoute, useRouter } from 'vue-router'
import { useThemeStore } from './stores/theme'
import { useAuthStore } from './stores/auth'
import { useCoursesStore } from './stores/courses'
import AppSidebar from './components/AppSidebar.vue'

const vuetifyTheme = useTheme()
const display = useDisplay()
const themeStore = useThemeStore()
const authStore = useAuthStore()
const coursesStore = useCoursesStore()
const router = useRouter()
const route = useRoute()

// Drawer: persistent 300px panel on desktop, temporary overlay on mobile.
const drawerOpen = ref(true)
const isMobile = computed(() => display.smAndDown.value)

watch(isMobile, (mobile) => {
  drawerOpen.value = !mobile
}, { immediate: true })

// Close the temporary drawer after navigation on mobile.
watch(() => route.path, () => {
  if (isMobile.value) {
    drawerOpen.value = false
  }
})

const completedCount = computed(() => coursesStore.completedLessonSlugs.length)
const totalCount = computed(() => coursesStore.totalLessons)
const completionPercentage = computed(() => coursesStore.completionPercentage)

onMounted(async () => {
  themeStore.restore()
  vuetifyTheme.change(themeStore.isDark ? 'dark' : 'light')
  await authStore.init()
  coursesStore.load()
})

// Keep completions in sync with auth state: refresh on login (a different user
// may have different completions), clear on logout so the next viewer doesn't
// see the previous user's progress.
watch(
  () => authStore.isAuthenticated,
  (isAuthenticated) => {
    if (isAuthenticated) {
      coursesStore.load(true)
    } else {
      coursesStore.resetProgress()
    }
  },
)

function toggleTheme() {
  themeStore.toggle()
  vuetifyTheme.change(themeStore.isDark ? 'dark' : 'light')
}

function toggleDrawer() {
  drawerOpen.value = !drawerOpen.value
}

function logout() {
  authStore.logout()
  coursesStore.resetProgress()
  router.push({ name: 'dashboard' })
}
</script>

<template>
  <v-app>
    <a href="#main-content" class="skip-link">Skip to main content</a>

    <v-app-bar flat border height="64">
      <v-app-bar-nav-icon
        aria-label="Toggle course navigation"
        :aria-expanded="drawerOpen ? 'true' : 'false'"
        @click="toggleDrawer"
      />
      <v-app-bar-title>
        <router-link :to="{ name: 'dashboard' }" class="app-title-link">
          <span class="app-logo rgb-glow" aria-hidden="true">
            <svg viewBox="0 0 48 32" width="20" height="20">
              <g stroke="currentColor" stroke-width="2.75" stroke-linecap="round" fill="none">
                <path d="M11 19 L2 13" />
                <path d="M11 23 L1 23" />
                <path d="M11 26 L3 31" />
                <path d="M37 19 L46 13" />
                <path d="M37 23 L47 23" />
                <path d="M37 26 L45 31" />
              </g>
              <circle cx="6" cy="8" r="5" fill="currentColor" />
              <circle cx="42" cy="8" r="5" fill="currentColor" />
              <ellipse cx="24" cy="20" rx="16" ry="11" fill="currentColor" />
              <circle cx="18" cy="15" r="2.5" fill="#fff" />
              <circle cx="30" cy="15" r="2.5" fill="#fff" />
              <circle cx="18" cy="15" r="1.15" fill="#1a1a1a" />
              <circle cx="30" cy="15" r="1.15" fill="#1a1a1a" />
            </svg>
          </span>
          Ferris Academy
        </router-link>
      </v-app-bar-title>
      <v-spacer />
      <v-btn icon="mdi-magnify" :to="{ name: 'search' }" aria-label="Search lessons" />
      <v-btn
        :icon="themeStore.isDark ? 'mdi-weather-sunny' : 'mdi-weather-night'"
        :aria-label="themeStore.isDark ? 'Switch to light mode' : 'Switch to dark mode'"
        @click="toggleTheme"
      />

      <template v-if="authStore.isAuthenticated">
        <span v-if="authStore.user?.displayName" class="text-body-2 text-medium-emphasis mr-2">{{ authStore.user.displayName }}</span>
        <v-btn variant="text" @click="logout">Log out</v-btn>
      </template>
      <template v-else>
        <v-btn variant="text" :to="{ name: 'login' }">Log in</v-btn>
        <v-btn color="primary" variant="flat" :to="{ name: 'register' }" class="mr-2">Sign up</v-btn>
      </template>
    </v-app-bar>

    <v-navigation-drawer
      v-model="drawerOpen"
      width="300"
      :temporary="isMobile"
      aria-label="Course navigation"
    >
      <AppSidebar />
    </v-navigation-drawer>

    <v-main>
      <div id="main-content" tabindex="-1" class="main-content">
        <router-view />
      </div>
    </v-main>

    <v-footer app border class="app-footer">
      <span class="text-caption text-medium-emphasis">Ferris Academy</span>
      <v-spacer />
      <span
        v-if="totalCount > 0"
        class="text-caption text-medium-emphasis"
        aria-live="polite"
      >
        {{ completedCount }} / {{ totalCount }} lessons &middot; {{ completionPercentage }}%
      </span>
      <span v-else-if="coursesStore.error" class="text-caption text-medium-emphasis">Could not load course progress</span>
      <span v-else class="text-caption text-medium-emphasis">Loading course progress&hellip;</span>
    </v-footer>
  </v-app>
</template>

<style>
html {
  scroll-behavior: smooth;
}

/* Ambient RGB glow: soft, off-brand-hued radial gradients painted onto the
   app background. Uses background-attachment so the glow stays put while the
   page scrolls, avoiding any z-index/stacking-context juggling. */
.v-application {
  background-image:
    radial-gradient(46vmax 46vmax at 8% -12%, rgba(206, 66, 43, 0.16), transparent 70%),
    radial-gradient(46vmax 46vmax at 104% 8%, rgba(56, 189, 248, 0.14), transparent 70%),
    radial-gradient(40vmax 40vmax at 15% 112%, rgba(74, 222, 128, 0.12), transparent 70%);
  background-repeat: no-repeat;
  background-attachment: fixed;
}

/* Animated multi-hue "snake" light: a conic-gradient ring masked down to a
   thin band running right along the element's own edge. The band sits
   flush with the element's border-box (inset: 0) rather than bleeding
   outside it, so it can never be cropped by an ancestor's
   `overflow: hidden` (Vuetify cards, buttons and avatars all set this) —
   no per-component overrides needed. */
@property --snake-angle {
  syntax: '<angle>';
  inherits: false;
  initial-value: 0deg;
}

@keyframes snake-spin {
  to {
    --snake-angle: 360deg;
  }
}

.rgb-glow {
  position: relative;
  isolation: isolate;
}

.rgb-glow::before {
  content: '';
  position: absolute;
  inset: 0;
  z-index: 1;
  border-radius: inherit;
  padding: 2.5px;
  background: conic-gradient(
    from var(--snake-angle),
    #ce422b 0deg,
    #fbbf24 72deg,
    #4ade80 144deg,
    #38bdf8 216deg,
    #a78bfa 288deg,
    #ce422b 360deg
  );
  -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
  -webkit-mask-composite: xor;
  mask-composite: exclude;
  animation: snake-spin 3.5s linear infinite;
  pointer-events: none;
}

.app-logo {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  margin-inline-end: 8px;
  border-radius: 999px;
  background: rgba(var(--v-theme-primary), 0.12);
  color: rgb(var(--v-theme-primary));
  vertical-align: -8px;
}

.visually-hidden {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.skip-link {
  position: absolute;
  top: -48px;
  left: 12px;
  z-index: 2000;
  padding: 8px 16px;
  border-radius: 8px;
  background: #CE422B;
  color: #F5F5F7;
  font-size: 0.875rem;
  font-weight: 600;
  text-decoration: none;
  transition: top 0.15s ease;
}

.skip-link:focus-visible {
  top: 8px;
  outline: 2px solid #F5F5F7;
  outline-offset: 2px;
}

.main-content:focus {
  outline: none;
}

.app-title-link {
  color: inherit;
  text-decoration: none;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.app-footer {
  max-height: 40px;
}

@media (prefers-reduced-motion: reduce) {
  html {
    scroll-behavior: auto;
  }

  *,
  *::before,
  *::after {
    animation: none !important;
    transition: none !important;
  }
}
</style>
