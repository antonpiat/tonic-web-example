<script setup lang="ts">
import {ref} from "vue";
import {Operation} from "../proto/grpc-web.ts";
import {GrpcWebService} from "../client.ts";

const num1 = ref(0);
const num2 = ref(0);
const operation = ref<keyof typeof Operation>("ADD");
const mathResult = ref<number | string>(0);

const doMath = async () => {
  try {
    const res = await new GrpcWebService().doMath({
      number1: num1.value,
      number2: num2.value,
      operation: Operation[operation.value]
    });
    mathResult.value = res.response.result;
  } catch (err: any) {
    console.error(err)
    mathResult.value = err.message || "Error";
  }
};
</script>

<template>
  <section class="section">
    <h2>Math Service</h2>
    <div class="inputs">
      <input type="number" v-model.number="num1" placeholder="Number 1" />
      <input type="number" v-model.number="num2" placeholder="Number 2" />
      <select v-model="operation">
        <option value="ADD">Add</option>
        <option value="SUBTRACT">Subtract</option>
        <option value="MULTIPLY">Multiply</option>
        <option value="DIVIDE">Divide</option>
      </select>
    </div>
    <button @click="doMath">Calculate</button>
    <p class="response">Result: {{mathResult}}</p>
  </section>
</template>

<style scoped>

</style>