<template lang="pug">
  .feedback
    .tags 标签：
      span(v-if='tags.length')
        a(v-for='tag in tags' v-bind:href='"/#/"+tag')
          el-tag(type="gray") {{ tag }}
      span(v-else)
        a(href='/#')
        el-tag(type="gray") 唔

    .comments(ref="gitalk_container")
</template>

<script>
    import Gitalk from 'gitalk';

    export default {
        props: ["tags"],
        mounted: function() {
          const gitalk = new Gitalk({
            clientID: '49aeab30501e804c71b0',
            clientSecret: '6d3c8320ac0c5c461afc2c01994833f05056b0de',
            repo: 'https://github.com/sf-zhou/sf-zhou.github.io',
            owner: 'SF-Zhou',
            admin: ['SF-Zhou'],
            distractionFreeMode: false
          });

          gitalk.render(this.$refs.gitalk_container)
        }
    };
</script>

<style lang="less" scoped>
.feedback {
    .tags {
      .el-tag {
        font-size: 1rem;
        margin-right: 0.5rem;
      }
    }

    .score {
      span {
        vertical-align: middle;
      }
      .el-rate {
        display: inline-block;
      }
    }
}
</style>
