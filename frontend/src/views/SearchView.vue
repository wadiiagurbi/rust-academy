<script setup>
import { onUnmounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import apiClient from '../api/client'
import DifficultyBadge from '../components/DifficultyBadge.vue'

const route = useRoute()
const router = useRouter()

function normalizeQueryParam(value) {
  const raw = Array.isArray(value) ? value[0] : value
  return (raw || '').trim()
}

const query = ref(normalizeQueryParam(route.query.q))
const results = ref([])
const loading = ref(false)
const searched = ref(false)
const error = ref(null)

let searchSeq = 0
let searchAbort = null
let debounceTimer = null

async function runSearch() {
  // query is always a string (v-model on a text field); no Array check needed.
  const trimmed = (query.value || '').trim()
  query.value = trimmed

  if (debounceTimer) {
    clearTimeout(debounceTimer)
    debounceTimer = null
  }

  if (!trimmed) {
    if (searchAbort) {
      searchAbort.abort()
      searchAbort = null
    }
    searchSeq += 1
    results.value = []
    searched.value = false
    error.value = null
    loading.value = false
    router.replace({ name: 'search', query: {} })
    return
  }

  loading.value = true
  searched.value = true
  error.value = null
  router.replace({ name: 'search', query: { q: trimmed } })

  if (searchAbort) {
    searchAbort.abort()
  }
  searchAbort = new AbortController()
  const seq = ++searchSeq
  const signal = searchAbort.signal

  try {
    const { data } = await apiClient.get('/search', { params: { q: trimmed }, signal })
    // Ignore stale responses: only the latest search may update state.
    if (seq !== searchSeq || signal.aborted) {
      return
    }
    results.value = Array.isArray(data.results) ? data.results : []
  } catch (searchError) {
    if (seq !== searchSeq) {
      return
    }
    // Aborted superseded requests are not errors.
    if (searchError.code === 'ERR_CANCELED' || searchError.name === 'CanceledError' || signal.aborted) {
      return
    }
    error.value = searchError.response?.data?.error?.message || 'Could not run the search. Please try again.'
    results.value = []
  } finally {
    if (seq === searchSeq) {
      loading.value = false
    }
  }
}

function scheduleSearch() {
  if (debounceTimer) {
    clearTimeout(debounceTimer)
  }
  debounceTimer = setTimeout(() => {
    debounceTimer = null
    runSearch()
  }, 300)
}

function clearSearch() {
  if (debounceTimer) {
    clearTimeout(debounceTimer)
    debounceTimer = null
  }
  if (searchAbort) {
    searchAbort.abort()
    searchAbort = null
  }
  searchSeq += 1
  query.value = ''
  results.value = []
  searched.value = false
  error.value = null
  loading.value = false
  router.replace({ name: 'search', query: {} })
}

// Keep the field in sync with ?q= for back/forward navigation.
watch(
  () => route.query.q,
  (newQ) => {
    const normalized = normalizeQueryParam(newQ)
    if (normalized === query.value) {
      return
    }
    query.value = normalized
    if (normalized) {
      runSearch()
    } else {
      if (searchAbort) {
        searchAbort.abort()
        searchAbort = null
      }
      searchSeq += 1
      results.value = []
      searched.value = false
      error.value = null
    }
  },
)

onUnmounted(() => {
  if (debounceTimer) {
    clearTimeout(debounceTimer)
    debounceTimer = null
  }
  if (searchAbort) {
    searchAbort.abort()
    searchAbort = null
  }
})

if (query.value) {
  runSearch()
}
</script>

<template>
  <v-container class="py-6" style="max-width: 760px">
    <h1 class="text-h5 mb-4">Search the course</h1>

    <v-text-field
      v-model="query"
      label="Search lessons by title, chapter, or content"
      prepend-inner-icon="mdi-magnify"
      clearable
      aria-label="Search lessons"
      @update:model-value="scheduleSearch"
      @keyup.enter="runSearch"
      @click:clear="clearSearch"
    >
      <template #append>
        <v-btn color="primary" @click="runSearch">Search</v-btn>
      </template>
    </v-text-field>

    <div v-if="loading" class="text-center py-8" role="status" aria-live="polite">
      <v-progress-circular indeterminate color="primary" />
      <span class="visually-hidden">Searching&hellip;</span>
    </div>

    <v-alert v-else-if="error" type="error" variant="tonal" class="mt-2" role="alert" aria-live="assertive" :text="error">
      <template #append>
        <v-btn size="small" variant="outlined" @click="runSearch">Retry</v-btn>
      </template>
    </v-alert>

    <v-alert v-else-if="searched && 0 === results.length" type="info" variant="tonal" text="No lessons matched your search." />

    <v-list v-else-if="results.length > 0" lines="two" aria-label="Search results">
      <v-list-item
        v-for="result in results"
        :key="result.lesson.slug"
        :to="{ name: 'lesson', params: { slug: result.lesson.slug } }"
        class="mb-2"
        rounded="lg"
        border
      >
        <v-list-item-title class="d-flex align-center ga-2">
          {{ result.lesson.title }}
          <DifficultyBadge :difficulty="result.lesson.difficulty" size="x-small" />
        </v-list-item-title>
        <v-list-item-subtitle>{{ result.lesson.chapterTitle }}</v-list-item-subtitle>
        <p class="text-body-2 text-medium-emphasis mt-1">{{ result.excerpt }}</p>
      </v-list-item>
    </v-list>
  </v-container>
</template>
