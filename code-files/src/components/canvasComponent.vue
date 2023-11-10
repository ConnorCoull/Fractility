<template>
    <div>
        <canvas ref="canvas" width="500" height="500"></canvas>
        <button @click="drawLine">Draw Line</button>
        <input type="range" v-model="iterations" min="0" max="15" />
        <p id="iterations">0</p>
        <input type="range" v-model="angle" min="0" max="180" />
        <p id="angle">0</p>
    </div>
</template>

<script>
    import init, { draw_dot, draw_fractal, draw_line_given_one_point } from '../../public/pkg/rust_backend.js';

    export default {
        data() {
            return {
                angle: 0,
                iterations: 0,
            };
        },
        methods: {
            drawdot() {
                const canvas = this.$refs.canvas;
                draw_dot(canvas, this.x, this.y);
            },
            drawLine() {
                const canvas = this.$refs.canvas;
                const context = canvas.getContext('2d');
                context.clearRect(0, 0, canvas.width, canvas.height);
                draw_line_given_one_point(canvas, 0, 0, 0, 50);
            },
            drawFractal() {
                if (this.iterations > 0) {
                    const canvas = this.$refs.canvas;
                    draw_line_given_one_point(canvas, 250, 500, 270, 200)
                    draw_fractal(250, 300, 100, 270 - parseInt(this.angle), 0.75, this.iterations, canvas);
                    draw_fractal(250, 300, 100, 270 + parseInt(this.angle), 0.75, this.iterations, canvas);
                }
            },
            clearCanvas() {
                const canvas = this.$refs.canvas;
                const context = canvas.getContext('2d');
                context.clearRect(0, 0, canvas.width, canvas.height);
            }
        },
        async mounted() {
            await init();
            // const cv = this.$refs.canvas;
            // cv.translate(cv.width / 2, cv.height);
            // cv.scale(-1);
            //this.drawLine();
        },
        watch: {
            angle() {
                this.clearCanvas();
                let angleText = document.getElementById("angle");
                angleText.innerHTML = this.angle;
                this.drawFractal();
            },
            iterations() {
                this.clearCanvas();
                let iterationsText = document.getElementById("iterations");
                iterationsText.innerHTML = this.iterations;
                this.drawFractal();
            }
        },
    };
</script>

<style scoped>
    canvas {
        border: 1px solid black;
    }
</style>
