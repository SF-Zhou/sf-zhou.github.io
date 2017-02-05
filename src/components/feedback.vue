<template lang="pug">
    .feedback
        .tags 标签：
            span(v-if='tags.length')
                a(v-for='tag in tags' v-bind:href='"/#/"+tag')
                    el-tag(type="gray") {{ tag }}
            span(v-else)
                a(href='/#')
                    el-tag(type="gray") 唔
        .score 
            span 评分：
            el-rate()
</template>

<script>
    export default {
        props: ["tags"]
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
