<template>
  <div class="info-header-container">
    <div class="row">
      <h3 class="var-title">{{ name }}</h3>
      <p :id="name" class="var-content">{{ localValue }}</p>
    </div>
    <div class="input-container">
      <input class="slider" type="range" v-model="localValue" :min="min" :max="max" :step="step" @input="updateValue" />
    </div>
  </div>
</template>

<script>
export default {
  props: {
    name: String,
    value: Number,
    min: Number,
    max: Number,
    step: {
      type: Number,
      default: 1
    },
  },
  data() {
    return {
      localValue: this.value
    }
  },
  methods: {
    updateValue() {
      document.getElementById(this.name).innerHTML = this.localValue;
      this.$emit("updateValue", this.name, this.localValue);
    },
  },
};
</script>

<style scoped>

.info-header-container {
  font-family: 'Open Sans', sans-serif;
}
.row {
    color: #f4f4f4;
    display: flex;
    justify-content: space-between;
    align-items: center;
}
.var-title{
    padding-left: 2rem;
    flex: 50%;
    text-align: left;
}
.var-content{
    flex: 50%;
}

.input-container {
  width: 100%;
  display: flex;
  justify-content: center;
}

/* change the slider input to be red instead of the default */

input[type="range"] {
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
  cursor: pointer;
  width: 15rem;
}

input[type="range"]::-webkit-slider-runnable-track {
  background: #f4f4f4;
  height: 1 rem;
}

input[type="range"]::-moz-range-track {
  background: #f4f4f4;
  height: 1 rem;
}

input[type="range"]::-webkit-slider-thumb {
   -webkit-appearance: none; /* Override default look */
   appearance: none;
   margin-top: -12px; /* Centers thumb on the track */
   background-color: #ffd700;
   height: 2rem;
   width: 1rem;    
}

</style>