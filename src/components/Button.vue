<script setup lang="ts">
import { watch } from 'vue';


const props = defineProps<{
    disabled?: boolean;
}>()

const emitter = defineEmits(["click"])
let emit = props.disabled ? () => {} : emitter

watch(() => props.disabled, (nv) => {
    emit = nv ? () => {} : emitter
});

</script>

<template>
    <div class="transition ease-in-out duration-200 w-25 h-10 text-xl bg-gray-400 hover:active:bg-gray-500 select-none hover:scale-110 rounded-3xl text-center p-0 flex" @click="emit('click')">
        <slot class=""></slot>
    </div>
</template>