<script setup lang="ts">
import { router } from '../router';

function back() {
    router.replace("/");
}
</script>
<template>
    <div class="ml-1 mr-auto mt-1 w-25 h-10 text-xl bg-gray-400 hover:bg-gray-500 active:bg-gray-600 rounded-3xl text-center pt-1.5 pb-auto"
        @click="back()"> 
       <p class="mt-auto mb-auto relative">< Back</p> 
    </div>
    <h1>{{ $route.params.id }}</h1>
</template>