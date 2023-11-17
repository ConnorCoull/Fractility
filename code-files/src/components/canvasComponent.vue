<template>
    <div class="container">
        <div class="controls-container">
            <div class="info-header-container">
                <div class="row">
                    <h3 class="var-title">Iterations:</h3>
                    <p id="iterations" class="var-content"></p>
                </div>
                <div>
                    <input type="range" v-model="iterations" min="0" max="15" />
                </div>
            </div>
            <div class="info-header-container">
                <div class="row">
                    <h3 class="var-title">Angle:</h3>
                    <p id="angle" class="var-content"></p>
                </div>
                <div>
                    <input type="range" v-model="angle" min="0" max="360" />
                </div>
            </div>
            <div class="info-header-container">
                <div class="row">
                    <h3 class="var-title">Length:</h3>
                    <p id="length" class="var-content"></p>
                </div>
                <div>
                    <input type="range" v-model="length" min="0" max="1000" />
                </div>
            </div>
            <div class="info-header-container">
                <div class="row">
                    <h3 class="var-title">Length Scalar:</h3>
                    <p id="length_scalar" class="var-content"></p>
                </div>
                <div>
                    <input type="range" v-model="length_scalar" min="0" max="2" step="0.1" />
                </div>
            </div>
            <div class="info-header-container">
                <div class="row">
                    <h3 class="var-title">Thickness:</h3>
                    <p id="thickness" class="var-content"></p>
                </div>
                <div>
                    <input type="range" v-model="thickness" min="0" max="100" />
                </div>
            </div>
            <div class="info-header-container">
                <div class="row">
                    <h3 class="var-title">Angle Scalar:</h3>
                    <p id="angle_scalar" class="var-content"></p>
                </div>
                <div>
                    <input type="range" v-model="angle_scalar" min="0" max="2" step="0.1"/>
                </div>
            </div>
            <div class="info-header-container">
                <div class="row">
                    <h3 class="var-title">Thickness Scalar:</h3>
                    <p id="thickness_scalar" class="var-content"></p>
                </div>
                <div>
                    <input type="range" v-model="thickness_scalar" min="0" max="2" step="0.1" />
                </div>
            </div>
        </div>
        <div class="canvas-container">
            <canvas ref="canvas"></canvas>
        </div>
    </div>
</template>
<script>
    import init, { /*draw_dot,*/ draw_fractal, clear /*,draw_line_given_one_point*/ } from '../../public/pkg/rust_backend.js';

    export default {
        data() {
            return {
                iterations: 10,
                length: 750,
                length_scalar: 0.49, //change to 50 to recreate bug on Friday, add the /100 too
                length_: 0,
                angle: 45,
                angle_scalar: 0.99,
                angle_constant: 0,
                thickness: 4.5,
                thickness_scalar: 0.75,
            };
        },
        methods: {
            // drawdot() {
            //     const canvas = this.$refs.canvas;
            //     draw_dot(canvas, this.x, this.y);
            // },
            // drawLine() {
            //     const canvas = this.$refs.canvas;
            //     clear(canvas);
            //     draw_line_given_one_point(canvas, 0, 0, 0, canvas.height, 1);
            // },
            drawFractal() {
                const canvas = this.$refs.canvas;
                //move this into a rust function that supports multiple arms
                draw_fractal(10, 0, this.length, this.length_scalar, this.angle, this.angle_scalar, this.iterations, canvas, this.thickness, this.thickness_scalar);
                draw_fractal(10, 0, this.length, this.length_scalar, -this.angle, this.angle_scalar, this.iterations, canvas, this.thickness, this.thickness_scalar);
            },
            clearCanvas() {
                const canvas = this.$refs.canvas;
                clear(canvas);
            },
            update(element, value) {
                element.innerHTML = value;
            }
        },
        async mounted() {
            await init();
            const canvas = this.$refs.canvas;
            const context = canvas.getContext('2d');

            context.canvas.width = window.innerWidth * 0.8;
            context.canvas.height = window.innerHeight * 0.975; // this doesn't feel great but not sure what alternative look like

            context.translate(canvas.width / 2, canvas.height);
            context.rotate(-Math.PI / 2);

            this.update(document.getElementById("iterations"), this.iterations);
            this.update(document.getElementById("angle"), this.angle);
            this.update(document.getElementById("length"), this.length);
            this.update(document.getElementById("length_scalar"), this.length_scalar);
            this.update(document.getElementById("thickness"), this.thickness);
            this.update(document.getElementById("angle_scalar"), this.angle_scalar);
            this.update(document.getElementById("thickness_scalar"), this.thickness_scalar);

            this.drawFractal();
            

        },
        watch: {
            angle() {
                this.clearCanvas();
                this.update(document.getElementById("angle"), this.angle);
                this.drawFractal();
            },
            iterations() {
                this.clearCanvas();
                this.update(document.getElementById("iterations"), this.iterations);
                this.drawFractal();
            },
            length() {
                this.clearCanvas();
                this.update(document.getElementById("length"), this.length);
                this.drawFractal();
            },
            length_scalar() {
                this.clearCanvas();
                this.update(document.getElementById("length_scalar"), this.length_scalar);
                this.drawFractal();
            },
            thickness() {
                this.clearCanvas();
                this.update(document.getElementById("thickness"), this.thickness);
                this.drawFractal();
            },
            angle_scalar() {
                this.clearCanvas();
                this.update(document.getElementById("angle_scalar"), this.angle_scalar);
                this.drawFractal();
            },
            thickness_scalar() {
                this.clearCanvas();
                this.update(document.getElementById("thickness_scalar"), this.thickness_scalar);
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
        margin: none;
        padding: none;
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
    .row {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .var-title{
        flex: 50%;
    }
    .var-content{
        flex: 50%;
    }
</style>
