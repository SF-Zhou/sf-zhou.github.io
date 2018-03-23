<template lang="pug">
    .markdown(v-html="article")
</template>

<script>
    import Vue from 'vue'

    export default {
        props: ['article'],
        mounted: function() {
            new Vue({el: '.markdown'});
        }
    };
</script>

<style lang="less">
.markdown {
    h2 {
        font-size: 2rem;
        text-align: center;
        @media screen and (max-width: 600px) {
            font-size: 1.1rem;
            transition: 200ms;
        }
    }

    h3 {
        font-size: 1.6rem;
        @media screen and (max-width: 600px) {
            font-size: 1.0rem;
            transition: 200ms;
        }
    }

    p {
        text-indent: 2rem;
    }

    eqn {
        .katex-display {
            padding-top: 0.5rem;
            padding-bottom: 0.5rem;
            max-width: 100%;
            overflow-x: auto;
        }
    }

    strong {
        font-weight: 1000;
    }

    ol, ul {
        padding-left: 3rem;

        p {
            display: inline-block;
            text-indent: 0rem;
            margin-left: 0rem;
            padding-left: 0rem;
        }
    }

    blockquote, ol, ul {
        ol, ul {
            padding-left: 2rem;
        }
    }

    blockquote {
        margin-left: 0rem;
        margin-right: 0rem;
        color: darkslategray;
        font-style: italic;
    }

    img {
        margin: 0 auto;
        display: block;
        max-width: 100%;
        border-radius: 4px;
        border: 1px solid #d1dbe5;
        box-shadow: 0 2px 4px 0 rgba(0,0,0,.12),0 0 6px 0 rgba(0,0,0,.04);
    }

    code {
        color: darkcyan;
        font-size: 1.0rem;
        font-family: "Courier New", Consolas, "Times New Roman";
    }

    pre {
        max-width: 100%;
        overflow-x: auto;
        padding: 0.5rem 0.8rem;
        border-radius: 4px;
        border: 1px solid #d1dbe5;
        box-shadow: 0 2px 4px 0 rgba(0,0,0,.12),0 0 6px 0 rgba(0,0,0,.04);

        code {
            color: black;
            font-size: 0.8rem;
        }
    }

    table {
        margin-left: auto;
        margin-right: auto;
        border-collapse: collapse;
        border: 2px solid black;
        th, td {
            border: 1px solid black;
            padding-left: 0.5rem;
            padding-right: 0.5rem;
        }
    }

    .vue_in_posts_container {
        border-radius: 4px;
        border: 1px solid #d1dbe5;
        box-shadow: 0 2px 4px 0 rgba(0,0,0,.12),0 0 6px 0 rgba(0,0,0,.04);
    }
}
</style>
