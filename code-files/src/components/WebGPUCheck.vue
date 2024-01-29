<template>
  <div class="WebGPUCheckPara">
    <p  v-if="webGPUSupported">WebGPU is supported!</p>
    <p v-else>WebGPU may not be supported, the content may not work as anticipated.</p>
  </div>
</template>
  
<script>
  export default {
    name: 'WebGPUSupport',
    data() {
      return {
        webGPUSupported: false,
      };
    },
    mounted() {
        // Check if WebGPU is supported in Chrome.
        this.webGPUSupported = 'gpu' in navigator;
    },
    async created() { // This may or may not be needed? It appears to work without so let's just pray 🙏🏻🙏🏻
      const adapter = await navigator.gpu?.requestAdapter();
      const device = await adapter?.requestDevice();
      this.isWebGPUSupported = !!device;
    },
  };
</script>

<style scoped>
.WebGPUCheckPara {
  font-family: 'Open Sans', sans-serif;
  color: #f4f4f4;
  text-align: center;
}
</style>
  