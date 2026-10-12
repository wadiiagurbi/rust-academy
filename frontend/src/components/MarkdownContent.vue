<script setup>
import { computed, nextTick, onUnmounted, ref, watch } from 'vue'
import { renderMarkdown } from '../utils/markdown'

const props = defineProps({
  source: { type: String, required: true },
})

const container = ref(null)
// Derived from source so there is a single reactive source of truth; the
// lone watch(html) below attaches copy buttons after each render.
const html = computed(() => renderMarkdown(props.source))

const copyTimers = new Set()

function attachCopyButtons() {
  if (!container.value) {
    return
  }

  container.value.querySelectorAll('pre').forEach((pre) => {
    if (pre.dataset.copyReady) {
      return
    }
    pre.dataset.copyReady = '1'

    const button = document.createElement('button')
    button.type = 'button'
    button.className = 'markdown-copy-btn'
    button.textContent = 'Copy'
    button.setAttribute('aria-label', 'Copy code')
    button.addEventListener('click', async () => {
      try {
        await navigator.clipboard.writeText(pre.textContent || '')
        button.textContent = 'Copied'
        const timerId = setTimeout(() => {
          button.textContent = 'Copy'
          copyTimers.delete(timerId)
        }, 1500)
        copyTimers.add(timerId)
      } catch {
        // Clipboard API unavailable — ignore.
      }
    })

    pre.style.position = 'relative'
    pre.appendChild(button)
  })
}

watch(html, () => nextTick(attachCopyButtons), { immediate: true })

onUnmounted(() => {
  copyTimers.forEach((timerId) => clearTimeout(timerId))
  copyTimers.clear()
})
</script>

<template>
  <div ref="container" class="markdown-content" v-html="html"></div>
</template>

<style>
.markdown-content {
  line-height: 1.7;
}

.markdown-content p {
  margin: 0 0 1em;
}

.markdown-content ul,
.markdown-content ol {
  margin: 0 0 1em;
  padding-left: 1.5em;
}

.markdown-content li {
  margin-bottom: 0.35em;
}

.markdown-content code {
  font-family: 'Fira Code', Consolas, Monaco, monospace;
  background: rgba(125, 125, 125, 0.15);
  padding: 0.15em 0.4em;
  border-radius: 4px;
  font-size: 0.9em;
}

.markdown-content pre {
  position: relative;
  background: #0d1117;
  color: #c9d1d9;
  padding: 16px;
  border-radius: 8px;
  overflow-x: auto;
  margin: 0 0 1em;
}

.markdown-content pre code {
  background: none;
  padding: 0;
  font-size: 0.875rem;
}

.markdown-copy-btn {
  position: absolute;
  top: 8px;
  right: 8px;
  background: rgba(255, 255, 255, 0.1);
  color: #c9d1d9;
  border: none;
  border-radius: 4px;
  padding: 4px 10px;
  font-size: 0.75rem;
  cursor: pointer;
}

.markdown-copy-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
