<template>
  <!-- <HelloWorld/> -->
  <!-- <canvasComponent/> -->
  <!-- <WebGPUCheck/> -->
  <AlternateTreeFractal />
</template>

<script>
//import HelloWorld from './components/HelloWorld.vue'
// import WebGPUCheck from './components/WebGPUCheck.vue'
// import canvasComponent from './components/canvasComponent.vue'
import AlternateTreeFractal from './components/AlternateTreeFractal.vue'

export default {
  name: 'App',
  components: {
    //HelloWorld,
    // WebGPUCheck,
    // canvasComponent,
    AlternateTreeFractal,
  }
}
</script>

<style>
body {
  margin: 0;
  padding: 0;
  background-color: #f4f4f4;
  width: 100%;
  height: 100%;
  overflow-x: hidden;
  overflow-y: hidden;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>
