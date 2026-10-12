import { defineStore } from 'pinia'
import apiClient from '../api/client'

// Shared in-flight request so concurrent load() calls (e.g. App mounted +
// Dashboard mounted) dedup into a single GET /courses.
let loadInflight = null

export const useCoursesStore = defineStore('courses', {
  state: () => ({
    tracks: [],
    completedLessonSlugs: [],
    loading: false,
    error: null,
    loaded: false,
  }),
  getters: {
    totalLessons: (state) => state.tracks.reduce(
      (trackSum, track) => trackSum + (track.chapters || []).reduce((chapterSum, chapter) => chapterSum + (chapter.lessons || []).length, 0),
      0,
    ),
    completionPercentage(state) {
      if (0 === this.totalLessons) {
        return 0
      }

      return Math.round((state.completedLessonSlugs.length / this.totalLessons) * 100)
    },
    isCompleted: (state) => (slug) => state.completedLessonSlugs.includes(slug),
  },
  actions: {
    async load(force = false) {
      if (this.loaded && !force) {
        return
      }

      // Dedup concurrent loads into one request.
      if (loadInflight) {
        return loadInflight
      }

      this.loading = true
      this.error = null
      loadInflight = (async () => {
        try {
          const { data } = await apiClient.get('/courses')
          this.tracks = Array.isArray(data.tracks) ? data.tracks : []
          this.completedLessonSlugs = Array.isArray(data.completedLessonSlugs) ? data.completedLessonSlugs : []
          this.loaded = true
        } catch (error) {
          this.error = error.response?.data?.error?.message || 'Could not load the course content.'
        } finally {
          this.loading = false
          loadInflight = null
        }
      })()
      return loadInflight
    },

    markCompleted(slug) {
      if (!this.completedLessonSlugs.includes(slug)) {
        this.completedLessonSlugs.push(slug)
      }
    },

    markIncomplete(slug) {
      this.completedLessonSlugs = this.completedLessonSlugs.filter((s) => s !== slug)
    },

    resetProgress() {
      this.completedLessonSlugs = []
    },
  },
})
