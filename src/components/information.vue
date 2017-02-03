<template lang="pug">
    .info
        .date(v-html="date")
        .author(v-html="author")
</template>

<script>
    export default {
        props: ['date', 'author']
    };
</script>

<style lang="less" scoped>
    .info {
        font-family: "Courier New", SimSun;
        text-align: center;

        .date, .author {
            display: inline-block;
            margin: 0rem 0.25rem;
        }
    }
</style>
