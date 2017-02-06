<template lang="pug">
    .markdown(v-html="article")
</template>

<script>
    export default {
        props: ['article']
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

        .katex-display {
            margin-left: -2rem; // against the text-indent
            padding-top: 0.5rem;
            padding-bottom: 0.5rem;
            max-width: 100%;
            overflow-x: scroll;
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
    }

    code {
        color: darkcyan;
        font-size: 1.0rem;
        font-family: "Courier New", Consolas, "Times New Roman";
    }

    pre {
        max-width: 100%;
        padding-left: 2rem;
        overflow-x: scroll;

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
        }
    }
}
</style>
