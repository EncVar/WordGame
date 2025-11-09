<script setup lang="ts">
import { ref } from 'vue';
import { router } from '../router';
import Button from '../components/Button.vue';
import Input from '../components/Input.vue';

let judge = ref(null);

function join() {
    router.replace(`/judge/${judge.value}`);
}

function back() {
    router.replace("/");
}
</script>

<template>
    <Button class="ml-2.5 mt-2.5 absolute"
        @click="back()"> 
       <p class="mt-auto mb-auto mr-auto ml-auto relative">< Back</p> 
    </Button>
    <div class="flex flex-row select-none">
        <h1 class="mt-90 text-6xl ml-auto">Join as judge </h1>
        <Input v-model="judge" class="mt-90 ml-5 mr-auto" @keyup.enter="join()"/>
    </div>
    <Button class="ml-auto mr-auto mt-10" @click="join()">
       <p class="mt-auto mb-auto ml-auto mr-auto relative items-center justify-center">Enter</p> 
    </Button>
</template>