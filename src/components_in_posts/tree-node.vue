<template lang="pug">
    g(v-bind:transform="`translate(${x} ${y})`")
        circle(r="20" v-bind:class="types.join(' ')")
        text {{ text }}
</template>

<script>
    export default {
        props: ['text', 'x', 'y', 'types']
    }
</script>

<style lang="less">
    g {
        circle {
            stroke: black;
            stroke-width: 2;
            fill: white;
        }

        text {
            fill: black;
            font-size: 20px;
            text-anchor: middle;
            transform: translate(0, 0.36em);  // vertical center
        }

        .green {
            fill: #CCFFCC;
        }

        .blue {
            fill: #DDDDFF;
        }

        .red {
            fill: #FFDDDD;
        }
        
        .gray {
            fill: lightgray;
        }
    }
</style>
