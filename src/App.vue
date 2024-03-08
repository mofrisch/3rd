<script setup>
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'
import { HashtagIcon, PlayIcon } from "@heroicons/vue/24/solid";
import { info } from "tauri-plugin-log-api";

const input = ref('')

const interpreter = () => {
  invoke('interpret', { input: input.value }).then((result) => {
    info("Result: " + result);
    document.getElementById('output').innerHTML = "";
    document.getElementById('output').innerHTML += result;
    //document.getElementById('input').value = ''
    input.value = '';
  });
}
</script>

<template>
  <div>
    <div class="mt-2 mx-2 flex rounded-md shadow-sm sticky bottom z-20">
      <div class="relative flex flex-grow items-stretch focus-within:z-10">
        <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
          <HashtagIcon class="h-5 w-5 text-gray-400" aria-hidden="true" />
        </div>
        <input id="input" v-model="input" @keyup.enter="interpreter"
          class="block w-full rounded-none rounded-l-md border-0 py-1.5 pl-10 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
      </div>
      <button @click="interpreter"
        class="relative -ml-px inline-flex items-center gap-x-1.5 rounded-r-md px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50">
        <PlayIcon class="-ml-0.5 h-5 w-5 text-green-600" aria-hidden=" true" />
        OK
      </button>
    </div>
  </div>
  <p class="mx-2" id="output"></p>
</template>
