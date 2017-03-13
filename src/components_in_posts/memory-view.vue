<template lang="pug">
  .component
    .square_row
      .container(v-for="i in memory")
        .square(v-bind:class="colors[i]")
</template>

<script>
  export default {
    props: ['used', 'length'],
    data() {
      return {
        colors: ['transparent', 'red', 'gray', 'blue', 'green', 'yellow'],
      }
    },
    computed: {
      memory: function() {
        let ret = Array(this.length);
        for (const segment of this.used) {
          const [start, length, color] = segment;
          for (let i = start; i < start + length; i ++) {
            ret[i] = color;
          }
        }
        return ret;
      }
    }
  }
</script>

<style lang="less" scoped>
  .component {
    margin: 10px;
    .transparent {}
    .red { background-color: red; }
    .gray { background-color: gray; }
    .blue { background-color: blue; }
    .green { background-color: green; }
    .yellow { background-color: yellow; }

    .square_row {
      display: flex;

      .container {
        flex: 1;

        .square {
          position: relative;
          width: 100%;
          padding-bottom: 100%;
          border: lightgray 1px solid;
        }
      }
    }

    .operation {
      margin: 10px auto;
      text-align: center;
    }
  }
</style>
