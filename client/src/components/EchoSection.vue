<script setup lang="ts">
import {ref} from "vue";
import {GrpcWebService} from "../client.ts";

const echoMessage = ref("");
const echoResponse = ref("");

const sendEcho = async () => {
  try {
    const res = await new GrpcWebService().echo({message: echoMessage.value});
    echoResponse.value = res.response.message;
  } catch (err) {
    console.error(err)
    echoResponse.value = "Error calling backend";
  }
};

</script>

<template>
  <section class="section">
    <h2>Echo Service</h2>
    <input type="text" v-model="echoMessage" placeholder="Enter message">
    <button @click="sendEcho">Send Echo</button>
    <p class="response">Response: {{echoResponse}}</p>
  </section>
</template>

<style scoped>

</style>