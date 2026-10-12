import hljs from 'highlight.js/lib/core'
import sql from 'highlight.js/lib/languages/sql'
import bash from 'highlight.js/lib/languages/bash'
import yaml from 'highlight.js/lib/languages/yaml'
import json from 'highlight.js/lib/languages/json'
import xml from 'highlight.js/lib/languages/xml'
import javascript from 'highlight.js/lib/languages/javascript'
import rust from 'highlight.js/lib/languages/rust'
// TOML has no dedicated highlight.js grammar; the INI grammar is a close
// approximation (sections, keys, comments) so we alias it as "toml".
import iniAsToml from 'highlight.js/lib/languages/ini'

hljs.registerLanguage('sql', sql)
hljs.registerLanguage('bash', bash)
hljs.registerLanguage('yaml', yaml)
hljs.registerLanguage('json', json)
hljs.registerLanguage('xml', xml)
hljs.registerLanguage('html', xml)
hljs.registerLanguage('javascript', javascript)
hljs.registerLanguage('rust', rust)
hljs.registerLanguage('toml', iniAsToml)

export default hljs
