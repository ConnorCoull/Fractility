<template>
  <div class="hello">
    <h1>{{ greeting }}</h1>
  </div>
</template>

<script>
// import { Component, Vue } from 'file-address';
import init, { greet } from '../../public/pkg/rust_backend.js';

export default {
  data() {
    return {
      greeting: '',
    };
  },

  async mounted() {
    await init();
    this.greeting = greet();
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1 {
  font-weight: normal;
}
</style>
