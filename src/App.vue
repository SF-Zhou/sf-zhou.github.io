<template lang="pug">
  .app
    comp-title(v-bind:title="title")
    .index(v-if="index")
      comp-abstract(v-for="info in index" v-bind:info="info")
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
  
  const content = window.content;

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
        web_master: config.web_master
      }
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
