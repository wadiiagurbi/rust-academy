import { createApp } from 'vue'
import { createPinia } from 'pinia'
import 'highlight.js/styles/github-dark.css'
import App from './App.vue'
import router from './router'
import vuetify from './plugins/vuetify'

const app = createApp(App)

app.config.errorHandler = (err, instance, info) => {
  // Surface render errors to the console without silently failing; the
  // router's chunk-reload guard handles lazy-load failures separately.
  console.error('[app error]', err, info)
}

app.use(createPinia())
app.use(router)
app.use(vuetify)

app.mount('#app')
