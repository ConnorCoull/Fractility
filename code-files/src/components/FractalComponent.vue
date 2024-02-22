<template>
    <div class="container">
        <div class="controls-container">
            <div class="header-container">
                <router-link to="/javascript">
                    <img class="logo-image" src="Rust_Logo_128.png" alt="Picture of the Rust Logo"/>
                </router-link>
                <h1 class="no-bottom-margin">Options</h1>
            </div>
            <ControlsContainer name="Iterations" :value="iterations" :min="0" :max="15" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Angle 1" :value="angle_1" :min="0" :max="360" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Angle 2" :value="angle_2" :min="0" :max="360" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Branches" :value="branches" :min="0" :max="10" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Length" :value="length" :min="0" :max="1000" :step="0.1" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Length Scalar" :value="length_scalar" :min="0" :max="2" :step="0.01" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Width" :value="width" :min="0" :max="1002" :step="0.1" @updateValue="updateEmittedValue" />
            <ControlsContainer name="Width Scalar" :value="width_scalar" :min="0" :max="2" :step="0.01" @updateValue="updateEmittedValue" />
            <button class="download-button" @click="downloadCanvas">Download</button>
        </div>
        <div class="canvas-container">
            <div class="timer-container">
                <p>{{ render_time }}ms</p>
            </div>
            <canvas ref="canvas"></canvas>
        </div>
    </div>
</template>
<script>
import init, { draw_alternate_fractal, clear, to_lower_case, to_float, get_canvas_height_up } from '../../public/pkg/rust_backend.js';
import ControlsContainer from './ControlsContainer.vue';

export default {
    components: { ControlsContainer },
    data() {
        return {
            angle_1: 0,
            angle_2: 120,
            iterations: 3,
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
            const startTime = performance.now();

            draw_alternate_fractal(
                get_canvas_height_up(canvas, 0.4),
                0,
                this.angle_1,
                this.angle_2,
                this.iterations,
                this.branches,
                this.length,
                this.length_scalar,
                this.width,
                this.width_scalar,
                canvas,
                this.start_color,
                this.render_time
            );

            const endTime = performance.now();
            const elapsedTime = endTime - startTime;
            this.render_time = elapsedTime.toFixed(2);
        },
        update(element, value) {
            element.innerHTML = value;
        },
        updateEmittedValue(name, value) { 
            name = to_lower_case(name);
            this[name] = to_float(value); //doesn't update?
            this.drawFractal();
        },
        downloadCanvas() {
            const canvas = this.$refs.canvas;
            const link = document.createElement('a');
            link.download = 'fractal.png';
            link.href = canvas.toDataURL();
            link.click();
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

    .no-bottom-margin {
        flex: 70%;
        margin-bottom: 0;
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

    .logo-image {
        padding-left: 16px;
        padding-top: 16px;
        height: 64px;
        width: 64px;
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

    .header-container {
        display: flex;
        align-items: center;
    }

    .timer-container {
        position: fixed;
        top: 1rem;
        right: 1rem;
        z-index: 100;
    }

    .controls-container h1 {
        color: #ffd700;
        font-family: 'Open Sans', sans-serif;
    }

    .download-button {
        background-color: #333333;
        color: #f4f4f4;
        border: none;
        padding: 10px 20px;
        padding-bottom: 1vh;
        font-size: 16px;
        cursor: pointer;
        margin-top: 20px;
        transition: background-color 0.3s ease;
    }
    .download-button:hover {
        color: #333333;
        background-color: #ffd700;
    }

</style>
