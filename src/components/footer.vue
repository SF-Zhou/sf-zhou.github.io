<template lang="pug">
    .footer
        .license
            span 本站的全部文字在
            a(href="http://creativecommons.org/licenses/by-sa/4.0/") 知识共享·署名·相同方式共享4.0协议
            span 之条款下提供
        .copyright Copyright©{{ year }} {{ web_master }}, All Rights Reserved
</template>

<script>
    export default {
        props: ['web_master'],
        data: () => {
            return {
                year: (new Date()).getFullYear()
            }
        }
    };
</script>

<style lang="less" scoped>
    .footer {
        margin-top: 2rem;
        font-size: 0.8rem;
        line-height: 1.2;
        text-align: center
    }
</style>
