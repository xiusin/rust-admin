<template>
  <div class="home">
    <h1>Home Page</h1>
    <el-button type="primary">Element Plus Button</el-button>
    <router-link to="/about">Go to About</router-link>
    
    <div class="counter">
      <h2>Counter: {{ counter.count }}</h2>
      <h3>Double Count: {{ counter.doubleCount }}</h3>
      <el-button type="success" @click="counter.increment">Increment</el-button>
      <el-button type="danger" @click="counter.decrement">Decrement</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useCounterStore } from '../stores/counter'

const counter = useCounterStore()
</script>

<style scoped>
.home {
  padding: 20px;
}

.counter {
  margin-top: 20px;
  padding: 20px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
}
</style>