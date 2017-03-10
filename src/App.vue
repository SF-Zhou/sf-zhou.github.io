<template lang="pug">
  .app
    comp-title(v-bind:title="title")
    .index(v-if="index")
      comp-abstract(v-for="info in index" v-if="!tag || info.tags.indexOf(tag) !== -1" v-bind:current_tag="tag" v-bind:info="info")
    .article(v-else)
      comp-info(v-bind:date="date" v-bind:author="author")
      comp-article(v-bind:article="article")
      comp-spliter(margin="2rem auto 1rem auto")
      comp-feedback(v-bind:tags="tags")
    comp-footer(v-bind:web_master="web_master")
</template>

<script>
  import CompAbstract from './components/abstract.vue'
  import CompTitle from './components/title.vue'
  import CompInfo from './components/information.vue'
  import CompArticle from './components/article.vue'
  import CompFeedback from './components/feedback.vue'
  import CompSpliter from './components/spliter.vue'
  import CompFooter from './components/footer.vue'
  import config from '../config.json'
  import "./style/main.less"
  import "./assets/favicon.ico"
  
  const content = window.content;
  const current_tag = () => decodeURI(window.location.hash.slice(2));

  export default {
    name: 'app',
    data() {
      return {
        index: content.index,
        title: content.title,
        date: content.date,
        author: content.author,
        article: content.article,
        tags: content.tags,
        tag: current_tag(),
        web_master: config.web_master
      }
    },
    methods: {
      anchor_changed: function(anchor) {
        this.tag = current_tag();
      }
    },
    created: function() {
      window.onhashchange = this.anchor_changed;
    },
    components: {
      CompAbstract,
      CompTitle,
      CompInfo,
      CompArticle,
      CompFeedback,
      CompSpliter,
      CompFooter
    }
  }
</script>
