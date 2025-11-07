<script setup lang="ts">
import { ref } from 'vue';
import { router } from '../router';

let group = ref(null);

function join(): string {
    router.replace(`/group/${group.value}`);
    return `/group/${group.value}`
    // if (group.value.toLowerCase() === "admin")
}

</script>

<template>
    <div class="flex flex-row">
        <h1 class="mt-90 text-6xl ml-auto">Join as group </h1>
        <input v-model="group" class="h-15 mt-90 ml-5 mr-auto text-3xl w-20 bg-gray-500 rounded-2xl text-center"></input>
    </div>
    <div class="ml-auto mr-auto mt-5 w-25 h-10 text-xl bg-gray-400 hover:bg-gray-500 active:bg-gray-600 rounded-3xl text-center pt-1.5 pb-auto"
        @click="join()"> 
       <p class="mt-auto mb-auto relative">Enter</p> 
    </div>
</template>