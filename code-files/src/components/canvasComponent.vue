<template>
    <div class="container">
        <div class="controls-container">
            <input type="range" v-model="iterations" min="0" max="15" />
            <p id="iterations">0</p>
            <input type="range" v-model="angle" min="0" max="180" />
            <p id="angle">0</p>
            <input type="range" v-model="length" min="1" max="500" />
            <p id="length">0</p>
            <input type="range" v-model="length_scalar" min="0" max="200" />
            <p id="length_scalar">0</p>
        </div>
        <div class="canvas-container">
            <canvas ref="canvas" width="500" height="500"></canvas>
        </div>
    </div>
</template>



<script>
    import init, { draw_dot, draw_fractal, draw_line_given_one_point } from '../../public/pkg/rust_backend.js';

    export default {
        data() {
            return {
                angle: 45,
                iterations: 10,
                length: 350,
                length_scalar: 50,
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
                draw_line_given_one_point(canvas, 0, 0, 0, canvas.height);
            },
            drawFractal() {
                if (this.iterations > 0) {
                    const canvas = this.$refs.canvas;
                    draw_fractal(0, 0, this.length, this.angle, this.length_scalar/100, this.iterations, canvas);
                    draw_fractal(0, 0, this.length, -this.angle, this.length_scalar/100, this.iterations, canvas);
                }
            },
            clearCanvas() {
                const canvas = this.$refs.canvas;
                const context = canvas.getContext('2d');
                context.clearRect(0, 0, canvas.width, canvas.height);
                context.clearRect(0, 0, -canvas.width, -canvas.height);
                context.clearRect(0, 0, -canvas.width, canvas.height);
                context.clearRect(0, 0, canvas.width, -canvas.height);
            }
        },
        async mounted() {
            await init();
            const canvas = this.$refs.canvas;
            const context = canvas.getContext('2d');

            context.canvas.width = window.innerWidth * 0.8;
            context.canvas.height = window.innerHeight * 0.97;

            context.translate(canvas.width / 2, canvas.height);
            context.rotate(-Math.PI / 2);

            let iterationsText = document.getElementById("iterations");
            iterationsText.innerHTML = this.iterations;

            let angleText = document.getElementById("angle");
            angleText.innerHTML = this.angle;

            let lengthText = document.getElementById("length");
            lengthText.innerHTML = this.length;

            let length_scalarText = document.getElementById("length_scalar");
            length_scalarText.innerHTML = this.length_scalar/100;

            this.drawFractal();
            

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
            },
            length() {
                this.clearCanvas();
                let lengthText = document.getElementById("length");
                lengthText.innerHTML = this.length;
                this.drawFractal();
            },
            length_scalar() {
                this.clearCanvas();
                let length_scalarText = document.getElementById("length_scalar");
                length_scalarText.innerHTML = this.length_scalar/100;
                this.drawFractal();
            }
        },
    };
</script>

<style scoped>
    body {
        margin: none;
        padding: none;
    }
    canvas {
        border: 1px solid black;
    }
    .container {
        display: flex;
        flex-direction: row;
        margin: none;
    }
    .canvas-container {
        flex: 80%;
    }
    .controls-container {
        flex: 20%;
    }
</style>
