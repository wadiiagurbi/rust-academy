<script setup>
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'
import { useAuthStore } from '../stores/auth'
import { useCoursesStore } from '../stores/courses'
import ProgressBar from '../components/ProgressBar.vue'

const authStore = useAuthStore()
const coursesStore = useCoursesStore()
const router = useRouter()

const progress = ref(null)
const loadingProgress = ref(false)
const progressError = ref(null)

const firstLessonSlug = computed(() => coursesStore.tracks[0]?.chapters?.[0]?.lessons?.[0]?.slug || null)

// First incomplete lesson across all tracks (in course order), so "Continue"
// resumes where the learner left off even when /progress has no last lesson.
const firstIncompleteSlug = computed(() => {
  for (const track of coursesStore.tracks) {
    for (const chapter of track.chapters || []) {
      for (const lesson of chapter.lessons || []) {
        if (!coursesStore.isCompleted(lesson.slug)) {
          return lesson.slug
        }
      }
    }
  }
  return firstLessonSlug.value
})

const continueSlug = computed(() => progress.value?.lastLessonSlug || firstIncompleteSlug.value)

const trackStats = computed(() =>
  coursesStore.tracks.map((track) => {
    const lessons = (track.chapters || []).flatMap((chapter) => chapter.lessons || [])
    const total = lessons.length
    const completed = lessons.filter((lesson) => coursesStore.isCompleted(lesson.slug)).length
    const percentage = total > 0 ? Math.round((completed / total) * 100) : 0
    const mix = { beginner: 0, intermediate: 0, advanced: 0 }
    lessons.forEach((lesson) => {
      if (Object.prototype.hasOwnProperty.call(mix, lesson.difficulty)) {
        mix[lesson.difficulty] += 1
      }
    })
    const nextLesson = lessons.find((lesson) => !coursesStore.isCompleted(lesson.slug)) || lessons[0] || null
    return { track, lessons, total, completed, percentage, mix, nextLesson }
  }),
)

const heroPrimaryLabel = computed(() => {
  if (!authStore.isAuthenticated) {
    return 'Get started'
  }
  return progress.value?.lastLessonSlug ? 'Continue learning' : 'Start learning'
})

const heroPrimaryDisabledReason = computed(() => {
  if (!authStore.isAuthenticated || continueSlug.value) {
    return null
  }
  if (coursesStore.loading && !coursesStore.loaded) {
    return 'Course content is still loading — please wait a moment.'
  }
  return 'No lessons available yet — please check back soon.'
})

// Final-project entry: derived from course data (track titled "Final Project",
// falling back to the last track's first lesson) instead of a hardcoded slug.
const finalProjectSlug = computed(() => {
  const tracks = coursesStore.tracks || []
  if (tracks.length === 0) {
    return null
  }
  const finalTrack = tracks.find((track) => /final project/i.test(track.title || '')) || tracks[tracks.length - 1]
  const lessons = (finalTrack.chapters || []).flatMap((chapter) => chapter.lessons || [])
  return lessons[0]?.slug || null
})

async function loadProgress() {
  if (!authStore.isAuthenticated) {
    return
  }
  loadingProgress.value = true
  progressError.value = null
  try {
    const { data } = await apiClient.get('/progress')
    progress.value = data
  } catch (loadError) {
    progressError.value = loadError.response?.data?.error?.message || 'Could not load your progress.'
  } finally {
    loadingProgress.value = false
  }
}

function heroPrimaryAction() {
  if (!authStore.isAuthenticated) {
    router.push({ name: 'register' })
    return
  }
  if (continueSlug.value) {
    router.push({ name: 'lesson', params: { slug: continueSlug.value } })
  }
}

function retry() {
  coursesStore.load(true)
  loadProgress()
}

function trackIcon(track) {
  return /final project/i.test(track.title) ? 'mdi-console-line' : 'mdi-cog-outline'
}

function trackActionLabel(stat) {
  if (stat.percentage >= 100) {
    return 'Review track'
  }
  if (stat.completed === 0) {
    return 'Start track'
  }
  return 'Continue'
}

onMounted(async () => {
  await coursesStore.load()
  await loadProgress()
})
</script>

<template>
  <v-container class="dashboard py-8">
    <!-- Left-aligned hero -->
    <section class="hero mb-10" aria-labelledby="dashboard-heading">
      <p class="eyebrow text-caption font-weight-bold mb-2">
        <v-icon size="14" icon="mdi-flash-outline" aria-hidden="true" />
        RUST &bull; FROM SCRATCH TO ADVANCED
      </p>
      <h1 id="dashboard-heading" class="hero-title mb-3">Learn Rust from zero to advanced.</h1>
      <p class="hero-sub text-body-1 text-medium-emphasis mb-6">
        One calm path from your first Rust line to ownership, concurrency, and a real CLI app — with cargo test exercises in every lesson.
      </p>
      <div class="d-flex flex-wrap ga-3 mb-4">
        <v-btn
          color="primary"
          variant="flat"
          size="large"
          class="rgb-glow"
          prepend-icon="mdi-rocket-launch-outline"
          :disabled="authStore.isAuthenticated && !continueSlug"
          :title="heroPrimaryDisabledReason || undefined"
          v-tooltip="heroPrimaryDisabledReason ? { text: heroPrimaryDisabledReason } : undefined"
          @click="heroPrimaryAction"
        >
          {{ heroPrimaryLabel }}
        </v-btn>
        <p v-if="heroPrimaryDisabledReason" class="text-caption text-medium-emphasis w-100 mb-0">{{ heroPrimaryDisabledReason }}</p>
        <v-btn
          v-if="!authStore.isAuthenticated"
          variant="outlined"
          size="large"
          prepend-icon="mdi-book-open-page-variant-outline"
          :disabled="!firstLessonSlug"
          :to="firstLessonSlug ? { name: 'lesson', params: { slug: firstLessonSlug } } : undefined"
        >
          Browse the course
        </v-btn>
        <v-btn v-else variant="outlined" size="large" prepend-icon="mdi-console-line" :disabled="!finalProjectSlug" :to="finalProjectSlug ? { name: 'lesson', params: { slug: finalProjectSlug } } : undefined">
          View Final Project
        </v-btn>
      </div>
      <div v-if="authStore.isAuthenticated" class="hero-progress">
        <p v-if="progress" class="text-body-2 text-medium-emphasis mb-2">
          You&rsquo;re {{ progress.completionPercentage }}% through &middot; {{ progress.completedCount }} of
          {{ progress.totalLessons }} lessons
          <span v-if="progress.quizAttemptsCount > 0">
            &middot; quiz average {{ progress.quizAverageScore }}%
          </span>
        </p>
        <p v-else-if="loadingProgress" class="text-body-2 text-medium-emphasis mb-2">Loading your progress&hellip;</p>
        <v-alert
          v-else-if="progressError"
          type="error"
          variant="tonal"
          density="compact"
          class="mt-2"
          :text="progressError"
        >
          <template #append>
            <v-btn variant="outlined" size="small" @click="loadProgress">Retry</v-btn>
          </template>
        </v-alert>
        <ProgressBar
          v-if="progress"
          :value="progress.completedCount"
          :total="progress.totalLessons"
          label="Overall course progress"
          class="hero-progress-bar"
        />
      </div>
    </section>

    <!-- Track grid -->
    <section aria-labelledby="tracks-heading">
      <div class="d-flex align-baseline flex-wrap ga-2 mb-4">
        <h2 id="tracks-heading" class="text-h5">
          <v-icon size="22" icon="mdi-map-marker-path" color="primary" class="mr-1" aria-hidden="true" />
          Learning tracks
        </h2>
        <span v-if="coursesStore.totalLessons > 0" class="text-body-2 text-medium-emphasis">
          {{ coursesStore.tracks.length }} tracks &middot; {{ coursesStore.totalLessons }} lessons
        </span>
      </div>

      <div v-if="coursesStore.loading && !coursesStore.loaded">
        <v-row>
          <v-col v-for="n in 4" :key="n" cols="12" md="6" lg="4">
            <v-skeleton-loader type="heading, subtitle, text, actions" />
          </v-col>
        </v-row>
      </div>

      <v-alert
        v-else-if="coursesStore.error"
        type="error"
        variant="tonal"
        class="mb-4"
        :text="coursesStore.error"
      >
        <template #append>
          <v-btn variant="outlined" size="small" @click="retry">Retry</v-btn>
        </template>
      </v-alert>

      <v-card v-else-if="trackStats.length === 0" variant="outlined" class="pa-6">
        <v-card-title>No tracks yet</v-card-title>
        <v-card-text class="text-medium-emphasis">
          Course content is still being prepared. Please check back soon.
        </v-card-text>
        <v-card-actions>
          <v-btn variant="outlined" @click="retry">Reload</v-btn>
        </v-card-actions>
      </v-card>

      <v-row v-else>
        <v-col
          v-for="(stat, index) in trackStats"
          :key="stat.track.key"
          cols="12"
          :md="index === 0 ? '12' : '6'"
          :lg="index === 0 ? '12' : '4'"
        >
          <v-card
            variant="outlined"
            class="track-card h-100 rgb-glow"
            :class="{ 'track-card--hero': index === 0 }"
          >
            <v-card-item>
              <template #prepend>
                <v-avatar :color="index === 0 ? 'primary' : 'surface-variant'" size="40" class="track-icon-avatar">
                  <v-icon :icon="trackIcon(stat.track)" :color="index === 0 ? 'on-primary' : undefined" aria-hidden="true" />
                </v-avatar>
              </template>
              <v-card-title class="track-title">{{ stat.track.title }}</v-card-title>
              <v-card-subtitle>{{ stat.total }} lessons &middot; {{ (stat.track.chapters || []).length }} chapters</v-card-subtitle>
            </v-card-item>
            <v-card-text>
              <p class="difficulty-mix text-caption text-medium-emphasis mb-3">
                <span class="mix-item"><span class="mix-dot mix-dot--beginner" aria-hidden="true" />{{ stat.mix.beginner }} beginner</span>
                <span class="mix-item"><span class="mix-dot mix-dot--intermediate" aria-hidden="true" />{{ stat.mix.intermediate }} intermediate</span>
                <span class="mix-item"><span class="mix-dot mix-dot--advanced" aria-hidden="true" />{{ stat.mix.advanced }} advanced</span>
              </p>
              <ProgressBar
                :value="stat.completed"
                :total="stat.total"
                :label="`${stat.track.title} progress`"
                class="mb-3"
              />
              <p v-if="stat.nextLesson" class="text-body-2 text-medium-emphasis mb-0">
                {{ stat.percentage >= 100 ? 'Completed — review anytime:' : 'Up next:' }}
                {{ stat.nextLesson.title }}
              </p>
              <ul v-if="index === 0" class="hero-chapters text-body-2 text-medium-emphasis mt-2">
                <li v-for="chapter in (stat.track.chapters || []).slice(0, 4)" :key="chapter.key">
                  {{ chapter.title }} ({{ (chapter.lessons || []).length }})
                </li>
              </ul>
            </v-card-text>
            <v-card-actions>
              <v-btn
                color="primary"
                :variant="stat.completed === 0 ? 'outlined' : 'flat'"
                :disabled="!stat.nextLesson"
                :to="stat.nextLesson ? { name: 'lesson', params: { slug: stat.nextLesson.slug } } : undefined"
              >
                {{ trackActionLabel(stat) }}
              </v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </section>

    <v-card variant="outlined" class="mt-8 rgb-glow">
      <v-card-item>
        <template #prepend>
          <v-avatar color="surface-variant" size="40">
            <v-icon icon="mdi-console-line" aria-hidden="true" />
          </v-avatar>
        </template>
        <v-card-title>Final Project: CLI Task Manager</v-card-title>
        <v-card-subtitle>Apply everything you learned — build a tested Rust CLI with cargo test exercises</v-card-subtitle>
      </v-card-item>
      <v-card-actions>
        <v-btn color="primary" variant="tonal" prepend-icon="mdi-flag-checkered" :disabled="!finalProjectSlug" :to="finalProjectSlug ? { name: 'lesson', params: { slug: finalProjectSlug } } : undefined">Start the Final Project</v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<style scoped>
.dashboard {
  max-width: 1120px;
}

.eyebrow {
  color: #CE422B;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.v-theme--dark .eyebrow {
  color: #F0A68D;
}

.hero-title {
  font-size: clamp(1.75rem, 4vw, 2.5rem);
  line-height: 1.15;
  letter-spacing: -0.01em;
  max-width: 22ch;
  background: linear-gradient(90deg, #ce422b, #f0a68d 50%, #ce422b);
  background-size: 200% auto;
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: hero-title-pan 8s ease-in-out infinite;
}

@keyframes hero-title-pan {
  0%,
  100% {
    background-position: 0% center;
  }
  50% {
    background-position: 100% center;
  }
}

.track-card {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.track-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 12px 24px -12px rgba(206, 66, 43, 0.35);
}

.track-icon-avatar {
  box-shadow: 0 0 0 4px rgba(var(--v-theme-primary), 0.08);
}

.hero-sub {
  max-width: 52ch;
}

.hero-progress-bar {
  max-width: 360px;
}

.track-card--hero {
  border-color: rgba(206, 66, 43, 0.4);
}

.difficulty-mix {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.mix-item {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.mix-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  display: inline-block;
}

.mix-dot--beginner {
  background: #15803D;
}

.mix-dot--intermediate {
  background: #B45309;
}

.mix-dot--advanced {
  background: #B91C1C;
}

.v-theme--dark .mix-dot--beginner {
  background: #4ADE80;
}

.v-theme--dark .mix-dot--intermediate {
  background: #FBBF24;
}

.v-theme--dark .mix-dot--advanced {
  background: #F87171;
}

.hero-chapters {
  list-style: disc;
  padding-inline-start: 20px;
  margin: 0;
}
</style>
