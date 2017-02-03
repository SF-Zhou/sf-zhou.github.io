import Vue from 'vue'
import { Button, Tag, Rate } from 'element-ui'
import App from './App.vue'

Vue.use(Button);
Vue.use(Tag);
Vue.use(Rate);

new Vue({
  el: '#app',
  render: h => h(App)
})
