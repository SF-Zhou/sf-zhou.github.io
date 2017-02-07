import Vue from 'vue'
import { Button, Tag, Rate, Card } from 'element-ui'
import App from './App.vue'

import "highlight.js/styles/github.css"
import "katex/dist/katex.min.css"

Vue.use(Button);
Vue.use(Tag);
Vue.use(Rate);
Vue.use(Card);

window.content.article = document.getElementById("original_article").innerHTML;

new Vue({
  el: '#app',
  render: h => h(App)
})
