import Vue from 'vue'
import App from './App.vue'
import "./plugin.js"

window.content.article = document.getElementById("original_article").innerHTML;

new Vue({
  el: '#app',
  render: h => h(App)
})
