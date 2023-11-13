<template>
    <div class="container">
        <div class="controls-container">
            <h3>Iterations:</h3>
            <input type="range" v-model="iterations" min="0" max="15" />
            <p id="iterations"></p>
            <h3>Angle:</h3>
            <input type="range" v-model="angle" min="0" max="180" />
            <p id="angle"></p>
            <h3>Length:</h3>
            <input type="range" v-model="length" min="1" max="1000" />
            <p id="length"></p>
            <h3>Length Scalar:</h3>
            <input type="range" v-model="length_scalar" min="0" max="2" step="0.01" />
            <p id="length_scalar"></p>
            <h3>Thickness:</h3>
            <input type="range" v-model="thickness" min="1" max="20" step="0.1" />
            <p id="thickness"></p>
            <h3>Angle Scalar:</h3>
            <input type="range" v-model="angle_scalar" min="0" max="2" step="0.01" />
            <p id="angle_scalar"></p>
            <h3>Thickness Scalar:</h3>
            <input type="range" v-model="thickness_scalar" min="0" max="2" step="0.01" />
            <p id="thickness_scalar"></p>
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
                length: 750,
                length_scalar: 0.49, //change to 50 to recreate bug on Friday, add the /100 too
                angle_scalar: 0.99,
                thickness: 4.5,
                thickness_scalar: 0.75,
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
                draw_line_given_one_point(canvas, 0, 0, 0, canvas.height, 1);
            },
            drawFractal() {
                if (this.iterations > 0) {
                    const canvas = this.$refs.canvas;
                    draw_fractal(0, 0, this.length, this.length_scalar, this.angle, this.angle_scalar, this.iterations, canvas, this.thickness, this.thickness_scalar);
                    draw_fractal(0, 0, this.length, this.length_scalar, -this.angle, this.angle_scalar, this.iterations, canvas, this.thickness, this.thickness_scalar);
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
            length_scalarText.innerHTML = this.length_scalar;

            let thicknessText = document.getElementById("thickness");
            thicknessText.innerHTML = this.thickness;

            let angle_scalarText = document.getElementById("angle_scalar");
            angle_scalarText.innerHTML = this.angle_scalar;

            let thickness_scalarText = document.getElementById("thickness_scalar");
            thickness_scalarText.innerHTML = this.thickness_scalar;

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
                length_scalarText.innerHTML = this.length_scalar;
                this.drawFractal();
            },
            thickness() {
                this.clearCanvas();
                let thicknessText = document.getElementById("thickness");
                thicknessText.innerHTML = this.thickness;
                this.drawFractal();
            },
            angle_scalar() {
                this.clearCanvas();
                let angle_scalarText = document.getElementById("angle_scalar");
                angle_scalarText.innerHTML = this.angle_scalar;
                this.drawFractal();
            },
            thickness_scalar() {
                this.clearCanvas();
                let thickness_scalarText = document.getElementById("thickness_scalar");
                thickness_scalarText.innerHTML = this.thickness_scalar;
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
        border-left: 1px solid black;
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
