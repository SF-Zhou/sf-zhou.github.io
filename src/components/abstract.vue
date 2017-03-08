<template lang="pug">
    el-card.abstract
        a.title(v-bind:href="'/' + info.url_path")
            span {{ info.title }}
        .info
            .base
                span.date {{ info.date }}
                span.author {{ info.author }}
            .tags
                span(v-if='info.tags.length')
                    span(v-for='tag in info.tags')
                        el-tag(v-if="current_tag && current_tag === tag" closable type="primary" v-on:close="go_home()")
                            a(v-bind:href='"/#/"+tag') {{ tag }}
                        el-tag(v-else type="gray")
                            a(v-bind:href='"/#/"+tag') {{ tag }}
</template>

<script>
export default {
    props: ["info", "current_tag"],
    methods: {
        go_home: function() {
            window.location.hash = "";
        }
    }
}
</script>

<style lang="less">
    .abstract {
        margin-top: 2rem;

        .title {
            font-size: 1.2rem;
            font-weight: 700;

            @media screen and (max-width: 600px) {
                font-size: 1.0rem;
                font-weight: 700;
            }
        }

        .info {
            .base, .tags {
                display: inline-block;
            }
            @media screen and (max-width: 600px) {
                .base, .tags {
                    display: block;
                }
            }
            .base {
                .date, .author {
                    font-family: "Courier New", Consolas, "Times New Roman";
                    vertical-align: top;
                    margin-right: 0.5rem;
                }
            }
            .tags {
                .el-tag {
                    font-family: inherit;
                    font-size: 1rem;
                    margin-right: 0.5rem;
                }
            }
        }
    }
</style>
