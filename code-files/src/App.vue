<template>
  <HelloWorld/>
  <WebGPUCheck/>
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'
import WebGPUCheck from './components/WebGPUCheck.vue'

export default {
  name: 'App',
  components: {
    HelloWorld,
    WebGPUCheck,
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
