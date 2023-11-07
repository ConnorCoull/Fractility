<template>
  <div class="hello-div">
    <h1>{{ greeting }}</h1>
    <input v-model="username" @keyup.enter="greet">
  </div>
  <div class="counter-div">
    <h1>{{ counter }}</h1>
    <input v-model="number_input" @keyup.enter="add">
    <button @click="set_to_zero">Set to zero</button>
  </div>
</template>

<script>
// import { Component, Vue } from 'file-address';
import init, { greet, add } from '../../public/pkg/rust_backend.js';

export default {
  data() {
    return {
      greeting: 'Hello, World!',
      username: '',
      counter: 0,
      number_input: 0,
    };
  },

  async mounted() {
    await init();
  },

  methods: {
    greet() {
      this.greeting = greet(this.username);
    },
    add() {
      // check type of counter and value, log them to console
      console.log("Counter: type, value")
      console.log(typeof this.counter);
      console.log(this.counter);
      console.log("number_input: type, value")
      console.log(typeof this.number_input);
      console.log(this.number_input);
      this.counter = add(this.counter, this.number_input);
    },
    set_to_zero() {
      this.counter = 0; // had a set to zero function in rust, but this is easier. DO NOT OVERCOMPLICATE THINGS!!!!!
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1 {
  font-weight: normal;
}
</style>
