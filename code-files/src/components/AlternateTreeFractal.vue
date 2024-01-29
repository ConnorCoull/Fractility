<template>
    <div class="container">
        <div class="controls-container">
            <h1>Options</h1>
            <ControlsContainer name="Iterations" :value="iterations" :min="0" :max="15" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Angle 1" :value="angle_1" :min="0" :max="360" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Angle 2" :value="angle_2" :min="0" :max="360" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Branches" :value="branches" :min="0" :max="10" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Length" :value="length" :min="0" :max="1000" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Length Scalar" :value="length_scalar" :min="0" :max="2" :step="0.01" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Width" :value="width" :min="0" :max="100" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Width Scalar" :value="width_scalar" :min="0" :max="2" :step="0.01" @updateValue="updateEmittedValue" />
            <WebGPUCheck />
        </div>
        <div class="canvas-container">
            <canvas ref="canvas"></canvas>
        </div>
    </div>
</template>
<script>
import init, { draw_alternate_fractal, clear, to_lower_case, to_float, get_canvas_height_up } from '../../public/pkg/rust_backend.js';
import ControlsContainer from './ControlsContainer.vue';
import WebGPUCheck from './WebGPUCheck.vue';

export default {
    components: { WebGPUCheck, ControlsContainer },
    data() {
        return {
            angle_1: 0,
            angle_2: 120,
            iterations: 5,
            branches: 3,
            length: 250,
            length_scalar: 0.5,
            width: 4.5,
            width_scalar: 0.75,
            start_color: "#DE493E",
        };
    },
    methods: {
        drawFractal() {
            const canvas = this.$refs.canvas;
            clear(canvas);
            console.log(`canvas width: ${canvas.width}, canvas height: ${canvas.height}`);
            draw_alternate_fractal(get_canvas_height_up(canvas, 0.3), 0, this.angle_1, this.angle_2, this.iterations, this.branches, this.length, this.length_scalar, this.width, this.width_scalar, canvas, this.start_color);
        },
        update(element, value) {
            element.innerHTML = value;
        },
        updateEmittedValue(name, value) { 
            name = to_lower_case(name);
            this[name] = to_float(value); //doesn't update?
            this.drawFractal();
        },
    },
    async mounted() {
        await init();
        const canvas = this.$refs.canvas;
        const context = canvas.getContext('2d');

        context.canvas.width = window.innerWidth * 0.8;
        context.canvas.height = window.innerHeight * 0.975; // this doesn't feel great but not sure what alternative look like

        context.translate(canvas.width / 2, canvas.height);
        context.rotate(-Math.PI / 2);

        this.drawFractal();
    },
};
</script>

<style scoped>
    .h1 {
        color: #ffd700;
    }

    body {
        margin: 0;
        padding: 0;
    }

    canvas {
        border-left: 1px solid #f4f4f4;
        margin: 0;
        padding: 0;
    }
    .container {
        display: flex;
        flex-direction: row;
        margin: 0;
        padding: 0;
        width: 100%;
        height: 100%;
    }
    .canvas-container {
        background: #f4f4f4;
        flex: 80%;
    }
    .controls-container {
        color: #f4f4f4;
        background: #333333;
        flex: 20%;
    }

    .controls-container h1 {
        color: #ffd700;
        font-family: 'Open Sans', sans-serif;
    }
</style>
