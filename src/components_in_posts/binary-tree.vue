<template>
    <div class="binary_tree">
        <svg :viewBox="`0 0 ${tree.width} ${tree.height}`" :style="`max-width: ${tree.width}px`">
            <line v-for="e in tree.edges" :x1="e.x1" :y1="e.y1" :x2="e.x2" :y2="e.y2" />
            <tree-node v-for="n in tree.nodes" :x="n.x" :y="n.y" :text="n.text" :types="n.types" />
        </svg>
    </div>
</template>

<script>
import treeNode from './tree-node.vue'

export default {
    props: {
        info: {
            type: Array,
            default: ['空']
        },
        w: {
            type: Number,
            default: 50
        },
        h: {
            type: Number,
            default: 50
        }
    },
    computed: {
        tree: function() {
            const w = this.w;
            const h = this.h;

            const edges = [];
            const nodes = [];
            const loop = function(list, base, depth) {
                let root_list = list.filter(e => !Array.isArray(e));
                if (root_list.length !== 1) {
                    throw `Tree Root Not Right: ${root}`;
                }

                const root = root_list[0];
                const is_object = (typeof root === 'object');
                const value = is_object ? root.value : root;
                const types = is_object ? root.types : [];

                let is_left = true;
                let left_width = 0;
                let right_width = 0;

                let max_depth = depth;
                const children_hori_pos = [];
                for (const e of list) {
                    if (!Array.isArray(e)) {
                        is_left = false;
                        if (left_width == 0) {
                            base += w / 2;
                        }
                        continue;
                    };

                    const sub_tree = loop(e, base + right_width, depth + 1);
                    max_depth = Math.max(max_depth, sub_tree.max_depth);
                    children_hori_pos.push(sub_tree.hori_pos);

                    if (is_left) {
                        left_width += sub_tree.width;
                        base += sub_tree.width;
                    } else {
                        right_width += sub_tree.width;
                    }
                }

                const current_height = (depth - 0.5) * h;
                nodes.push({
                    x: base,
                    y: current_height,
                    text: value,
                    types: types
                })

                for (const hori_pos of children_hori_pos) {
                    edges.push({
                        x1: base,
                        y1: current_height,
                        x2: hori_pos,
                        y2: current_height + h
                    })
                }

                const root_width = Math.max(left_width, w / 2) + Math.max(right_width, w / 2);
                return {
                    width: root_width,
                    hori_pos: base,
                    max_depth
                }
            }
            const root = loop(this.info, 0, 1);

            return {
                width: root.width,
                height: root.max_depth * 50,
                nodes,
                edges
            }
        }
    },
    components: {
        treeNode
    }
}
</script>

<style lang="less">
    .binary_tree {
        svg {
            margin-left: auto;
            margin-right: auto;
            display: block;

            line {
                stroke: black;
                stroke-width: 2;
            }
        }
    }
</style>
