<script setup lang="ts">

let model = defineModel();
const emit = defineEmits(["onEnter", "focusout"]);

</script>

<template>
    <input v-model="model" class="transition ease-in-out duration-200 h-15 text-3xl w-30 bg-gray-300 rounded-4xl text-center hover:scale-110 focus:scale-110 focus:bg-gray-400 focus:border-0" 
        @keyup.enter="emit('onEnter')" @focusout="emit('focusout')" autofocus></input>
</template>