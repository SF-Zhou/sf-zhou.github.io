<template lang="pug">
  .chemistry
    .textarea
      el-input(type="textarea" :autosize="{minRows:4}" placeholder="向老师提问" v-model="decode_string")
    .buttons
      el-button(type="success" icon="arrow-down" v-on:click="encode") 提问
      el-button(type="info" icon="arrow-up" v-on:click="decode") 解答
    .textarea
      el-input(type="textarea" :autosize="{minRows:4}" placeholder="化学老师说" v-model="encode_string")
</template>

<script>

const old_chars = Array.from('+/0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz');
const new_chars = Array.from('氖氙氟氢氦氧氩氪氮氯溴砷硅硒硫硼碘碲碳磷钆钇钌钐钒钕钙钛钠钡钪钯钴钷钼钾铁铈铌铍铑铕铜铝铟铬铯银铷铺锂锆锌锑锗锝锡锰锶镁镉镍镓镧');

const encode_mapper = {};
const decode_mapper = {};
for (const i in old_chars) {
  encode_mapper[old_chars[i]] = new_chars[i];
  decode_mapper[new_chars[i]] = old_chars[i];
}
decode_mapper['。'] = '';
decode_mapper['！'] = '=';
decode_mapper['？'] = '==';

export default {
  data() {
    return {
      decode_string: '',
      encode_string: '',
    }
  },
  methods: {
    encode: function() {
      const raw = this.decode_string;
      const base_64 = btoa(encodeURIComponent(raw));
      const end = base_64.endsWith('==') ? '？' : base_64.endsWith('=') ? '！' : base_64 ? '。' : '';
      const encode = Array.from(base_64).map((c) => encode_mapper[c]).join('') + end;
      this.encode_string = "化学老师说：" + encode;
    },
    decode: function() {
      const encode = this.encode_string;
      if (encode.startsWith("化学老师说：")) {
        const base64 = Array.from(encode.substring(6)).map((c) => decode_mapper[c]).join('');
        this.decode_string = decodeURIComponent(atob(base64));
      } else {
        this.decode_string = "";
      }
    }
  }
}
</script>

<style lang="less" scoped>
  .chemistry {
    .textarea {
      margin: 0.3em;
    }

    .buttons {
      margin: 0.5em auto;
      text-align: center;
    }
  }
</style>
