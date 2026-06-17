<script setup lang="ts">
import { ref } from "vue";
import { GrpcWebService } from "../client.ts";

const echoMessage = ref("");
const echoResponse = ref("");

const sendEcho = async () => {
  try {
    const res = await new GrpcWebService().echo({ message: echoMessage.value });
    echoResponse.value = res.response.message;
  } catch (err) {
    console.error(err);
    echoResponse.value = "Error calling backend";
  }
};

const inputClass =
  "min-w-0 rounded-lg border border-violet-500/30 bg-[#f7f5fc] px-3.5 py-2.5 font-sans text-[0.95rem] text-[#1a1523] outline-none transition hover:border-violet-400 focus:border-violet-600 focus:bg-white/95 focus:ring-[3px] focus:ring-violet-600/10 dark:border-violet-400/25 dark:bg-[#0f0e14] dark:text-[#f4f1fa] dark:focus:bg-[rgba(36,33,48,0.92)] placeholder:text-[#8b8494] dark:placeholder:text-[#7d7890]";

const buttonClass =
  "mt-2 cursor-pointer rounded-lg bg-gradient-to-br from-violet-600 to-violet-800 px-5 py-2.5 font-sans text-[0.95rem] font-semibold tracking-wide text-white shadow-md shadow-violet-600/35 transition hover:-translate-y-px hover:brightness-110 hover:shadow-lg active:translate-y-0 active:brightness-95 focus-visible:outline-2 focus-visible:outline-offset-[3px] focus-visible:outline-violet-400";

const responseClass =
  "mt-4 inline-flex max-w-full items-center gap-1.5 break-words rounded-lg border border-emerald-600/20 bg-emerald-600/10 px-4 py-2 font-mono text-sm font-medium text-emerald-600 dark:border-emerald-400/25 dark:text-emerald-400";

const cardClass =
  "rounded-[14px] border border-violet-500/15 bg-white/80 p-7 shadow-md backdrop-blur-md transition hover:-translate-y-[3px] hover:border-violet-500/30 hover:bg-white/95 hover:shadow-xl dark:border-violet-400/10 dark:bg-[rgba(28,26,38,0.78)] dark:hover:bg-[rgba(36,33,48,0.92)]";

const headingClass =
  "mb-5 flex items-center justify-center gap-2.5 text-xl font-semibold tracking-tight text-[#1a1523] before:h-[1.1em] before:w-1 before:shrink-0 before:rounded-full before:bg-gradient-to-b before:from-violet-400 before:to-violet-800 before:content-[''] dark:text-[#f4f1fa]";
</script>

<template>
  <section :class="cardClass">
    <div class="flex w-full flex-col items-center gap-1 text-center">
      <h2 :class="headingClass">Echo Service</h2>
      <input
        type="text"
        v-model="echoMessage"
        placeholder="Enter message"
        :class="[inputClass, 'w-full max-w-[22rem]']"
      />
      <button @click="sendEcho" :class="buttonClass">Send Echo</button>
      <p :class="responseClass">Response: {{ echoResponse }}</p>
    </div>
  </section>
</template>
