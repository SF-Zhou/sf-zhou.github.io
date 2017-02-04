import Vue from 'vue'
import { Button, Tag, Rate } from 'element-ui'
import App from './App.vue'

import "highlight.js/styles/github.css"

Vue.use(Button);
Vue.use(Tag);
Vue.use(Rate);

new Vue({
  el: '#app',
  render: h => h(App)
})
