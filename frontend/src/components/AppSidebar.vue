<script setup>
import { computed, ref } from 'vue'
import { useRoute } from 'vue-router'
import { useCoursesStore } from '../stores/courses'

const coursesStore = useCoursesStore()
const route = useRoute()

const search = ref('')

const currentSlug = computed(() => route.params.slug)

const filteredTracks = computed(() => {
  const query = search.value.trim().toLowerCase()
  if (!query) {
    return coursesStore.tracks
  }
  return coursesStore.tracks
    .map((track) => ({
      ...track,
      chapters: (track.chapters || [])
        .map((chapter) => ({
          ...chapter,
          lessons: (chapter.lessons || []).filter((lesson) => (lesson.title || '').toLowerCase().includes(query)),
        }))
        .filter((chapter) => chapter.lessons.length > 0),
    }))
    .filter((track) => track.chapters.length > 0)
})

function trackLessons(track) {
  return (track.chapters || []).flatMap((chapter) => chapter.lessons || [])
}

function trackCompleted(track) {
  return trackLessons(track).filter((lesson) => coursesStore.isCompleted(lesson.slug)).length
}

function chapterCompleted(chapter) {
  return (chapter.lessons || []).filter((lesson) => coursesStore.isCompleted(lesson.slug)).length
}

function trackIcon(track) {
  return /final project/i.test(track.title) ? 'mdi-console-line' : 'mdi-cog-outline'
}

function chapterPercentage(chapter) {
  const lessons = chapter.lessons || []
  if (lessons.length === 0) {
    return 0
  }
  return Math.round((chapterCompleted(chapter) / lessons.length) * 100)
}
</script>

<template>
  <div class="sidebar-content">
    <div class="pa-3 pb-1">
      <v-text-field
        v-model="search"
        density="compact"
        variant="outlined"
        prepend-inner-icon="mdi-magnify"
        label="Filter lessons"
        hide-details
        clearable
        single-line
      />
    </div>

    <div v-if="coursesStore.loading && !coursesStore.loaded" class="pa-4">
      <v-progress-linear indeterminate color="primary" aria-label="Loading course content" />
    </div>

    <v-alert
      v-else-if="coursesStore.error"
      type="error"
      variant="tonal"
      class="ma-2"
      density="compact"
      :text="coursesStore.error"
    >
      <template #append>
        <v-btn variant="outlined" size="small" @click="coursesStore.load(true)">Retry</v-btn>
      </template>
    </v-alert>

    <p v-else-if="filteredTracks.length === 0" class="text-body-2 text-medium-emphasis pa-4">
      No lessons match &ldquo;{{ search }}&rdquo;.
    </p>

    <v-list v-else density="compact" nav open-strategy="multiple" aria-label="Lessons by track">
      <template v-for="track in filteredTracks" :key="track.key">
        <v-list-subheader class="sidebar-track-header">
          <div class="sidebar-track-header-inner">
            <span class="sidebar-track-title">
              <v-icon :icon="trackIcon(track)" size="16" color="primary" class="mr-1" aria-hidden="true" />
              {{ track.title }}
            </span>
            <span class="sidebar-track-count text-caption text-medium-emphasis">
              {{ trackCompleted(track) }}/{{ trackLessons(track).length }}
            </span>
          </div>
        </v-list-subheader>

        <v-list-group v-for="chapter in (track.chapters || [])" :key="chapter.key" :value="chapter.key">
          <template #activator="{ props: activatorProps }">
            <div class="sidebar-chapter">
              <v-list-item
                v-bind="activatorProps"
                :title="chapter.title"
                :subtitle="`${chapterCompleted(chapter)}/${(chapter.lessons || []).length} complete`"
                density="compact"
              />
              <v-progress-linear
                :model-value="chapterPercentage(chapter)"
                color="primary"
                height="4"
                rounded
                class="sidebar-chapter-progress"
                :aria-label="`${chapter.title}: ${chapterPercentage(chapter)} percent complete`"
              />
            </div>
          </template>

          <v-list-item
            v-for="lesson in (chapter.lessons || [])"
            :key="lesson.slug"
            :to="{ name: 'lesson', params: { slug: lesson.slug } }"
            :active="lesson.slug === currentSlug"
            active-class="active-lesson"
            :aria-current="lesson.slug === currentSlug ? 'page' : undefined"
            density="compact"
            class="sidebar-lesson"
          >
            <template #prepend>
              <v-icon
                size="18"
                :color="coursesStore.isCompleted(lesson.slug) ? 'success' : undefined"
                :icon="coursesStore.isCompleted(lesson.slug) ? 'mdi-check-circle' : 'mdi-circle-outline'"
                aria-hidden="true"
              />
            </template>
            <v-list-item-title class="text-body-2">{{ lesson.title }}</v-list-item-title>
          </v-list-item>
        </v-list-group>
      </template>
    </v-list>
  </div>
</template>

<style scoped>
.sidebar-content {
  height: 100%;
  overflow-y: auto;
}

/* Vuetify wraps a v-list-subheader's slot content in its own
   .v-list-subheader__text element, so flex rules on the component root
   never apply to our two spans — target that inner wrapper instead. */
.sidebar-track-header :deep(.v-list-subheader__text) {
  width: 100%;
}

.sidebar-track-header-inner {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 8px;
  width: 100%;
}

.sidebar-track-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sidebar-track-count {
  flex: 0 0 auto;
  font-variant-numeric: tabular-nums;
}

.sidebar-chapter {
  padding-bottom: 4px;
}

.sidebar-chapter-progress {
  margin: 0 16px;
  max-width: calc(100% - 32px);
}

.sidebar-lesson.active-lesson {
  background: rgba(var(--v-theme-primary), 0.1);
  border-inline-start: 3px solid rgb(var(--v-theme-primary));
  box-shadow: inset 0 0 12px -4px rgba(var(--v-theme-primary), 0.4);
  font-weight: 600;
}
</style>
