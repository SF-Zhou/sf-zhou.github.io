<template lang="pug">
  .app
    comp-title(v-bind:title="title")
    comp-info(v-bind:date="date" v-bind:author="author")
    comp-article(v-bind:article="article")
    comp-spliter(margin="2rem auto 1rem auto")
    comp-feedback(v-bind:tags="tags")
    comp-footer(web_master="SF-Zhou")
</template>

<script>
  import CompTitle from './components/title.vue'
  import CompInfo from './components/information.vue'
  import CompArticle from './components/article.vue'
  import CompFeedback from './components/feedback.vue'
  import CompSpliter from './components/spliter.vue'
  import CompFooter from './components/footer.vue'
  import content from './js/content.js'
  import "./style/main.less"

  export default {
    name: 'app',
    data() {
      return {
        title: content.title,
        date: content.date,
        author: content.author,
        article: content.article,
        tags: content.tags
      }
    },
    components: {
      CompTitle,
      CompInfo,
      CompArticle,
      CompFeedback,
      CompSpliter,
      CompFooter
    }
  }
</script>
