<template>
    <div>
        <canvas ref="canvas" width="100" height="100"></canvas>
        <input type="range" min="0" max="100" v-model="x" />
        <input type="range" min="0" max="100" v-model="y" />
        <button @click="drawdot">Draw Dot</button>
    </div>
</template>

<script>
    import { draw_dot } from '../../public/pkg/rust_backend.js';

    export default {
        data() {
            return {
                x: 0,
                y: 0,
            };
        },
        methods: {
            drawdot() {
                const canvas = this.$refs.canvas;
                draw_dot(canvas, this.x, this.y);
            },
        },
    };
</script>
