<script setup lang="ts">
import EchoSection from "./components/EchoSection.vue";
import MathSection from "./components/MathSection.vue";
</script>

<template>
  <div
    class="mx-auto flex min-h-svh w-full max-w-[820px] flex-col gap-8 px-6 py-10 max-sm:gap-6 max-sm:px-4 max-sm:py-6 max-sm:pb-10"
  >
    <header
      class="flex items-center justify-center gap-2 self-center rounded-full border border-violet-500/15 bg-white/80 px-5 py-3 shadow-sm backdrop-blur-md dark:border-violet-400/10 dark:bg-[rgba(28,26,38,0.78)]"
    >
      <div class="flex items-center gap-1">
        <a
          href="https://vite.dev"
          target="_blank"
          rel="noopener noreferrer"
          class="flex rounded-lg p-2 transition hover:-translate-y-0.5 hover:bg-violet-600/10"
        >
          <img
            src="/favicon.svg"
            alt="Vite logo"
            class="h-9 w-auto transition hover:drop-shadow-[0_0_0.75rem_rgba(100,108,255,0.55)]"
          />
        </a>
        <a
          href="https://vuejs.org/"
          target="_blank"
          rel="noopener noreferrer"
          class="flex rounded-lg p-2 transition hover:-translate-y-0.5 hover:bg-violet-600/10"
        >
          <img
            src="./assets/vue.svg"
            alt="Vue logo"
            class="h-9 w-auto transition hover:drop-shadow-[0_0_0.75rem_rgba(66,184,131,0.55)]"
          />
        </a>
      </div>
    </header>

    <main class="flex flex-col gap-7">
      <div class="text-center">
        <h1
          class="mb-2 bg-gradient-to-br from-[#1a1523] from-30% to-violet-600 bg-clip-text text-[clamp(2rem,5vw,2.75rem)] font-semibold tracking-tight text-transparent dark:from-[#f4f1fa] dark:to-violet-400"
        >
          Rust gRPC Web Frontend
        </h1>
        <p
          class="mx-auto max-w-lg text-[1.05rem] leading-relaxed text-[#8b8494] dark:text-[#7d7890]"
        >
          Interact with your Rust backend through gRPC-Web — echo messages and run
          math operations in real time.
        </p>
      </div>

      <div class="flex flex-col gap-5">
        <EchoSection />
        <MathSection />
      </div>
    </main>
  </div>
</template>
