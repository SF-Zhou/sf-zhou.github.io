<template lang="pug">
    .spliter(v-bind:style="'margin:' + margin")
</template>

<script>
export default {
    props: ["margin"]
}
</script>

<style lang="less" scoped>
    .spliter {
        border-bottom: 1px solid darkslategray;
    }
</style>
