import Vue from 'vue'

exports.install = function() {
    Vue.component('memory-view', require('./memory-view.vue'));
    Vue.component('binary-tree', require('./binary-tree.vue'));
    Vue.component('chemistry-teacher', require('./chemistry-teacher.vue'));
}
