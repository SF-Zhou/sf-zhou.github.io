<template lang="pug">
    .title
        h1 {{ title }}
</template>

<script>
    export default {
        props: ['title']
    };
</script>

<style lang="less" scoped>
    .title {
        padding-top: 2rem;
        h1 {
            margin-top: 0;
            text-align: center;
            font-size: 3rem;

            @media screen and (max-width: 600px) {
                font-size: 1.2rem;
                font-weight: bold;
                transition: 200ms;
            }
        }
    }
</style>
