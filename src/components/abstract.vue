<template lang="pug">
    .abstract
        h2.row
            span.date {{ info.date }}
            a(v-bind:href="'/' + info.url_path")
                span {{ info.title }}
</template>

<script>
export default {
    props: ["info"]
}
</script>

<style lang="less">
    .abstract {
        .row {
            .date {
                margin-right: 1rem;
            }
        }
    }
</style>
