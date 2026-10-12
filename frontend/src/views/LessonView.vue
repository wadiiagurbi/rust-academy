<script setup>
import { computed, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import apiClient from '../api/client'
import { useAuthStore } from '../stores/auth'
import { useCoursesStore } from '../stores/courses'
import MarkdownContent from '../components/MarkdownContent.vue'
import CodeBlock from '../components/CodeBlock.vue'
import DifficultyBadge from '../components/DifficultyBadge.vue'
import QuizWidget from '../components/QuizWidget.vue'
import ProgressBar from '../components/ProgressBar.vue'

const props = defineProps({
  slug: { type: String, required: true },
})

const router = useRouter()
const authStore = useAuthStore()
const coursesStore = useCoursesStore()

const lesson = ref(null)
const loading = ref(true)
const error = ref(null)
const solutionVisible = ref(false)
const togglingCompletion = ref(false)
const completionError = ref(null)

const isCompleted = computed(() => coursesStore.isCompleted(props.slug))
const completedCount = computed(() => coursesStore.completedLessonSlugs.length)
const totalCount = computed(() => coursesStore.totalLessons)
const completionPercentage = computed(() => coursesStore.completionPercentage)

// Backend sends navigation.previous/next as { slug, title } objects or null.
// The plain-string branch below is intentionally kept as a defensive fallback
// for unexpected API shapes (see normalizeNav); do not remove as dead code.
function normalizeNav(entry) {
  if (!entry) {
    return null
  }
  if (typeof entry === 'string') {
    return { slug: entry, title: entry }
  }
  return entry
}

const prevNav = computed(() => normalizeNav(lesson.value?.navigation?.previous))
const nextNav = computed(() => normalizeNav(lesson.value?.navigation?.next))

const breadcrumbItems = computed(() => {
  if (!lesson.value) {
    return []
  }
  return [
    { title: 'Home', to: { name: 'dashboard' } },
    { title: lesson.value.trackTitle || 'Track', disabled: true },
    { title: lesson.value.chapterTitle || 'Chapter', disabled: true },
    { title: lesson.value.title || 'Lesson', disabled: true },
  ]
})

const tocItems = [
  { id: 'learning-objective', label: 'Learning Objective', icon: 'mdi-target' },
  { id: 'explanation', label: 'Explanation', icon: 'mdi-book-open-page-variant-outline' },
  { id: 'why-it-matters', label: 'Why It Matters', icon: 'mdi-lightbulb-on-outline' },
  { id: 'example', label: 'Example', icon: 'mdi-code-tags' },
  { id: 'code', label: 'Code', icon: 'mdi-code-braces-box' },
  { id: 'code-explanation', label: 'Code Explanation', icon: 'mdi-text-box-search-outline' },
  { id: 'common-mistakes', label: 'Common Mistakes', icon: 'mdi-alert-circle-outline' },
  { id: 'best-practices', label: 'Best Practices', icon: 'mdi-star-outline' },
  { id: 'exercise', label: 'Exercise', icon: 'mdi-dumbbell' },
  { id: 'solution', label: 'Solution', icon: 'mdi-key-variant' },
  { id: 'quiz', label: 'Mini Quiz', icon: 'mdi-help-circle-outline' },
]

let lessonRequestId = 0

async function loadLesson(slug) {
  const requestId = ++lessonRequestId
  loading.value = true
  error.value = null
  completionError.value = null
  lesson.value = null
  solutionVisible.value = false

  try {
    const { data } = await apiClient.get(`/lessons/${slug}`)
    // Ignore stale responses from fast navigation: only apply if this is
    // still the latest request and the route hasn't moved on.
    if (requestId !== lessonRequestId || slug !== props.slug) {
      return
    }
    lesson.value = data
  } catch (loadError) {
    if (requestId !== lessonRequestId || slug !== props.slug) {
      return
    }
    if (404 === loadError.response?.status) {
      error.value = 'This lesson could not be found. It may have been moved or renamed.'
    } else {
      error.value = 'Something went wrong while loading this lesson. Please try again.'
    }
  } finally {
    if (requestId === lessonRequestId) {
      loading.value = false
    }
  }
}

async function toggleCompletion() {
  togglingCompletion.value = true
  completionError.value = null
  try {
    if (isCompleted.value) {
      await apiClient.delete(`/lessons/${props.slug}/complete`)
      coursesStore.markIncomplete(props.slug)
    } else {
      await apiClient.post(`/lessons/${props.slug}/complete`)
      coursesStore.markCompleted(props.slug)
    }
  } catch (toggleError) {
    if (401 === toggleError.response?.status) {
      authStore.logout()
      coursesStore.resetProgress()
      router.push({ name: 'login', query: { redirect: router.currentRoute.value.fullPath } })
      return
    }
    completionError.value = toggleError.response?.data?.error?.message || 'Could not update your progress. Please try again.'
  } finally {
    togglingCompletion.value = false
  }
}

function goTo(slug) {
  router.push({ name: 'lesson', params: { slug } })
}

watch(() => props.slug, loadLesson, { immediate: true })
</script>

<template>
  <v-container class="lesson-wrap py-6">
    <div v-if="loading">
      <v-skeleton-loader type="heading, text, paragraph, paragraph, actions" class="mb-4" />
    </div>

    <v-alert v-else-if="error" type="error" variant="tonal" :text="error">
      <template #append>
        <v-btn variant="outlined" size="small" @click="loadLesson(props.slug)">Retry</v-btn>
      </template>
    </v-alert>

    <v-row v-else-if="lesson">
      <v-col cols="12" md="8" lg="9">
        <article>
          <v-breadcrumbs :items="breadcrumbItems" density="compact" class="px-0 lesson-breadcrumbs" aria-label="Breadcrumb" />

          <div class="d-flex align-center flex-wrap ga-2 mb-4">
            <h1 class="text-h4 mr-2">{{ lesson.title }}</h1>
            <DifficultyBadge :difficulty="lesson.difficulty" />
            <v-chip size="small" variant="outlined"><v-icon start size="16" icon="mdi-clock-outline" aria-hidden="true" />{{ lesson.estimatedMinutes }} min</v-chip>
          </div>

          <!-- Collapsible mini-TOC on mobile -->
          <v-expansion-panels class="d-md-none mb-4">
            <v-expansion-panel title="On this page" rounded="xl">
              <v-expansion-panel-text>
                <nav aria-label="Lesson contents">
                  <ol class="toc-list">
                    <li v-for="item in tocItems" :key="item.id">
                      <a :href="`#${item.id}`" class="toc-link">
                        <v-icon :icon="item.icon" size="16" aria-hidden="true" />
                        {{ item.label }}
                      </a>
                    </li>
                  </ol>
                </nav>
              </v-expansion-panel-text>
            </v-expansion-panel>
          </v-expansion-panels>

          <v-alert v-if="completionError" type="error" variant="tonal" density="compact" class="mb-4" role="alert" :text="completionError" />
          <v-btn
            v-if="authStore.isAuthenticated"
            class="mb-6"
            :color="isCompleted ? 'success' : 'primary'"
            :variant="isCompleted ? 'flat' : 'outlined'"
            :loading="togglingCompletion"
            :prepend-icon="isCompleted ? 'mdi-check-circle' : 'mdi-circle-outline'"
            @click="toggleCompletion"
          >
            {{ isCompleted ? 'Completed' : 'Mark as complete' }}
          </v-btn>
          <v-alert v-else class="mb-6" type="info" variant="tonal" density="compact">
            <router-link :to="{ name: 'login' }">Log in</router-link> to track your progress and mark lessons as complete.
          </v-alert>

          <section id="learning-objective" class="mb-6 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-target" color="primary" aria-hidden="true" /> Learning Objective</h2>
            <MarkdownContent :source="lesson.sections.learningObjective" />
          </section>

          <section id="explanation" class="mb-6 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-book-open-page-variant-outline" color="info" aria-hidden="true" /> Explanation</h2>
            <MarkdownContent :source="lesson.sections.explanation" />
          </section>

          <section id="why-it-matters" class="mb-6 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-lightbulb-on-outline" color="warning" aria-hidden="true" /> Why It Matters</h2>
            <MarkdownContent :source="lesson.sections.whyItMatters" />
          </section>

          <section id="example" class="mb-6 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-code-tags" color="primary" aria-hidden="true" /> Example</h2>
            <MarkdownContent :source="lesson.sections.example" />
          </section>

          <section id="code" class="mb-6 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-code-braces-box" color="primary" aria-hidden="true" /> Code</h2>
            <CodeBlock :code="lesson.sections.code" :language="lesson.sections.codeLanguage" />
          </section>

          <section id="code-explanation" class="mb-6 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-text-box-search-outline" color="info" aria-hidden="true" /> Code Explanation</h2>
            <MarkdownContent :source="lesson.sections.codeExplanation" />
          </section>

          <section id="common-mistakes" class="mb-6 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-alert-circle-outline" color="warning" aria-hidden="true" /> Common Mistakes</h2>
            <MarkdownContent :source="lesson.sections.commonMistakes" />
          </section>

          <section id="best-practices" class="mb-6 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-star-outline" color="success" aria-hidden="true" /> Best Practices</h2>
            <MarkdownContent :source="lesson.sections.bestPractices" />
          </section>

          <v-divider class="my-6" />

          <section id="exercise" class="mb-6 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-dumbbell" color="primary" aria-hidden="true" /> Exercise</h2>
            <MarkdownContent :source="lesson.sections.exercise" />
          </section>

          <section id="solution" class="mb-6 lesson-section" aria-labelledby="solution-heading">
            <v-card variant="outlined" class="solution-card rgb-glow">
              <v-card-item>
                <template #prepend>
                  <v-icon icon="mdi-key-variant" color="primary" aria-hidden="true" />
                </template>
                <v-card-title id="solution-heading">Solution</v-card-title>
                <v-card-subtitle>Try the exercise first — then compare approaches.</v-card-subtitle>
              </v-card-item>
              <v-card-actions>
                <v-btn
                  variant="tonal"
                  :prepend-icon="solutionVisible ? 'mdi-eye-off' : 'mdi-eye'"
                  :aria-expanded="solutionVisible ? 'true' : 'false'"
                  aria-controls="solution-panel"
                  @click="solutionVisible = !solutionVisible"
                >
                  {{ solutionVisible ? 'Hide Solution' : 'Show Solution' }}
                </v-btn>
              </v-card-actions>
              <v-expand-transition>
                <div v-show="solutionVisible" id="solution-panel" class="solution-panel">
                  <v-divider />
                  <div class="pa-4">
                    <MarkdownContent :source="lesson.sections.solution" />
                  </div>
                </div>
              </v-expand-transition>
            </v-card>
          </section>

          <v-divider class="my-6" />

          <section id="quiz" class="mb-10 lesson-section">
            <h2 class="text-h6 mb-2"><v-icon size="26" icon="mdi-help-circle-outline" color="primary" aria-hidden="true" /> Mini Quiz</h2>
            <QuizWidget :key="lesson.slug" :slug="lesson.slug" :questions="lesson.quiz" />
          </section>

          <div v-if="totalCount > 0" class="lesson-progress mb-4">
            <ProgressBar
              :value="completedCount"
              :total="totalCount"
              label="Overall course progress"
            />
            <p class="text-caption text-medium-emphasis mt-1 mb-0">
              Course progress: {{ completedCount }} of {{ totalCount }} lessons ({{ completionPercentage }}%)
            </p>
          </div>

          <nav class="lesson-nav d-flex ga-2 mb-10" aria-label="Lesson navigation">
            <v-btn
              v-if="prevNav"
              variant="outlined"
              prepend-icon="mdi-arrow-left"
              class="lesson-nav-btn"
              @click="goTo(prevNav.slug)"
            >
              <span class="lesson-nav-label">{{ prevNav.title }}</span>
            </v-btn>
            <v-spacer v-if="prevNav && nextNav" />
            <v-btn
              v-if="nextNav"
              color="primary"
              append-icon="mdi-arrow-right"
              class="lesson-nav-btn"
              @click="goTo(nextNav.slug)"
            >
              <span class="lesson-nav-label">{{ nextNav.title }}</span>
            </v-btn>
          </nav>
        </article>
      </v-col>

      <v-col cols="12" md="4" lg="3" class="d-none d-md-block">
        <aside class="toc-rail" aria-label="Lesson contents">
          <v-card variant="outlined" class="toc-card">
            <v-card-item>
              <v-card-title class="text-subtitle-1">On this page</v-card-title>
            </v-card-item>
            <v-card-text>
              <nav aria-label="Lesson sections">
                <ol class="toc-list">
                  <li v-for="item in tocItems" :key="item.id">
                    <a :href="`#${item.id}`" class="toc-link">
                      <v-icon :icon="item.icon" size="16" aria-hidden="true" />
                      {{ item.label }}
                    </a>
                  </li>
                </ol>
              </nav>
            </v-card-text>
          </v-card>
        </aside>
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>
.lesson-wrap {
  max-width: 1200px;
}

.lesson-section {
  scroll-margin-top: 84px;
}

.lesson-section h2 :deep(.v-icon) {
  margin-inline-end: 4px;
  vertical-align: -4px;
}

.toc-rail {
  position: sticky;
  top: 84px;
}

.toc-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.toc-link {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  border-radius: 8px;
  font-size: 0.875rem;
  color: inherit;
  text-decoration: none;
  border-inline-start: 2px solid transparent;
  transition: background 0.15s ease, border-color 0.15s ease;
}

.toc-link :deep(.v-icon) {
  opacity: 0.7;
}

.toc-link:hover {
  background: rgba(var(--v-theme-primary), 0.08);
  border-inline-start-color: rgb(var(--v-theme-primary));
}

.lesson-breadcrumbs {
  flex-wrap: wrap;
}

.solution-panel {
  overflow: hidden;
}

.lesson-nav-btn {
  min-width: 0;
  max-width: 48%;
}

.lesson-nav-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

@media (max-width: 767px) {
  .lesson-nav {
    flex-direction: column;
  }

  .lesson-nav-btn {
    max-width: 100%;
  }
}
</style>
