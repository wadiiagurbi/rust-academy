<script setup>
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useCoursesStore } from '../stores/courses'

const email = ref('')
const password = ref('')
const loading = ref(false)
const error = ref(null)

const authStore = useAuthStore()
const coursesStore = useCoursesStore()
const router = useRouter()
const route = useRoute()

const emailRules = [
  (v) => !!v || 'Email is required',
  (v) => /.+@.+\..+/.test(v) || 'Please enter a valid email address',
]
const passwordRules = [
  (v) => !!v || 'Password is required',
  (v) => (v && v.length >= 8) || 'Password must be at least 8 characters',
]

const isFormValid = computed(() => /.+@.+\..+/.test(email.value) && (password.value || '').length >= 8)

function safeRedirect(raw) {
  const candidate = Array.isArray(raw) ? raw[0] : raw
  if (typeof candidate === 'string' && candidate.startsWith('/') && !candidate.startsWith('//')) {
    return candidate
  }
  return null
}

async function submit() {
  if (!isFormValid.value || loading.value) {
    return
  }
  loading.value = true
  error.value = null
  try {
    await authStore.login(email.value, password.value)
    await coursesStore.load(true)
    const redirect = safeRedirect(route.query.redirect)
    if (redirect) {
      router.push(redirect)
    } else {
      router.push({ name: 'dashboard' })
    }
  } catch (submitError) {
    error.value = submitError.response?.data?.error?.message || 'Invalid email or password.'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <v-container class="py-10" style="max-width: 480px">
    <v-card elevation="2">
      <v-card-item>
        <template #prepend>
          <v-avatar color="primary" size="40" class="rgb-glow">
            <v-icon icon="mdi-login-variant" color="on-primary" aria-hidden="true" />
          </v-avatar>
        </template>
        <v-card-title>Log in</v-card-title>
        <v-card-subtitle>Continue your Rust learning journey</v-card-subtitle>
      </v-card-item>
      <v-card-text>
        <v-form @submit.prevent="submit">
          <v-text-field v-model="email" label="Email" type="email" required autocomplete="email" :rules="emailRules" />
          <v-text-field v-model="password" label="Password" type="password" required autocomplete="current-password" :rules="passwordRules" />
          <v-alert v-if="error" type="error" variant="tonal" density="compact" class="mb-4" role="alert" aria-live="assertive" :text="error" />
          <v-btn type="submit" color="primary" block :loading="loading" :disabled="loading || !isFormValid">Log in</v-btn>
        </v-form>
        <p class="text-body-2 mt-4 text-center">
          Don't have an account?
          <router-link :to="{ name: 'register' }">Create one</router-link>
        </p>
      </v-card-text>
    </v-card>
  </v-container>
</template>
