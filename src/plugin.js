import Vue from 'vue'
import { Button, Tag, Rate, Card } from 'element-ui'

Vue.use(Button);
Vue.use(Tag);
Vue.use(Rate);
Vue.use(Card);

import VueInPosts from "../compiled/vue_in_posts.js"
Vue.use(VueInPosts);

import ComponentsInPosts from "./components_in_posts/install.js"
Vue.use(ComponentsInPosts);

import "highlight.js/styles/github.css"
import "katex/dist/katex.min.css"
