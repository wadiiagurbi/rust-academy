<script setup>
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useCoursesStore } from '../stores/courses'

const email = ref('')
const password = ref('')
const displayName = ref('')
const loading = ref(false)
const error = ref(null)

const authStore = useAuthStore()
const coursesStore = useCoursesStore()
const router = useRouter()

const displayNameRules = [
  (v) => !!v || 'Display name is required',
  (v) => (v && v.length <= 64) || 'Display name must be at most 64 characters',
]
const emailRules = [
  (v) => !!v || 'Email is required',
  (v) => /.+@.+\..+/.test(v) || 'Please enter a valid email address',
]
const passwordRules = [
  (v) => !!v || 'Password is required',
  (v) => (v && v.length >= 8) || 'Password must be at least 8 characters',
]

const isFormValid = computed(
  () =>
    (displayName.value || '').trim().length > 0 &&
    (displayName.value || '').length <= 64 &&
    /.+@.+\..+/.test(email.value) &&
    (password.value || '').length >= 8,
)

async function submit() {
  if (!isFormValid.value || loading.value) {
    return
  }
  loading.value = true
  error.value = null
  try {
    await authStore.register(email.value, password.value, displayName.value)
    await coursesStore.load(true)
    router.push({ name: 'dashboard' })
  } catch (submitError) {
    error.value = submitError.response?.data?.error?.message || 'Could not create your account.'
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
            <v-icon icon="mdi-account-plus-outline" color="on-primary" aria-hidden="true" />
          </v-avatar>
        </template>
        <v-card-title>Create your account</v-card-title>
        <v-card-subtitle>Track your progress across the whole course</v-card-subtitle>
      </v-card-item>
      <v-card-text>
        <v-form @submit.prevent="submit">
          <v-text-field v-model="displayName" label="Display name" required autocomplete="name" :rules="displayNameRules" :counter="64" />
          <v-text-field v-model="email" label="Email" type="email" required autocomplete="email" :rules="emailRules" />
          <v-text-field v-model="password" label="Password" type="password" required hint="At least 8 characters" autocomplete="new-password" :rules="passwordRules" />
          <v-alert v-if="error" type="error" variant="tonal" density="compact" class="mb-4" role="alert" aria-live="assertive" :text="error" />
          <v-btn type="submit" color="primary" block :loading="loading" :disabled="loading || !isFormValid">Create account</v-btn>
        </v-form>
        <p class="text-body-2 mt-4 text-center">
          Already have an account?
          <router-link :to="{ name: 'login' }">Log in</router-link>
        </p>
      </v-card-text>
    </v-card>
  </v-container>
</template>
