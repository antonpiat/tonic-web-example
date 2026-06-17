<script setup lang="ts">
import EchoSection from "./components/EchoSection.vue";
import MathSection from "./components/MathSection.vue";
import "./style.css";
</script>

<template>
  <header class="site-header">
    <div class="brand-logos">
      <a href="https://vite.dev" target="_blank" rel="noopener noreferrer">
        <img src="/favicon.svg" alt="Vite logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank" rel="noopener noreferrer">
        <img src="./assets/vue.svg" alt="Vue logo" />
      </a>
    </div>
  </header>

  <main class="container">
    <div class="page-intro">
      <h1>Rust gRPC Web Frontend</h1>
      <p class="page-subtitle">
        Interact with your Rust backend through gRPC-Web — echo messages and run
        math operations in real time.
      </p>
    </div>

    <div class="cards">
      <EchoSection class="card" />
      <MathSection class="card" />
    </div>
  </main>
</template>
