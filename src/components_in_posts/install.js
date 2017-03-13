import Vue from 'vue'

exports.install = function() {
    Vue.component('memory-view', require('./memory-view.vue'));
}
