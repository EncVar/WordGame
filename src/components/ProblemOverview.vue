<script setup lang="ts">
import { ProblemStatus } from '../problem';

defineProps<{
    enabled: boolean;
    status: ProblemStatus;
}>();
</script>

<template>
    <div :class="'rounded-3xl h-full w-full flex justify-center items-center hover:shadow-md border border-gray-200 transition ease-in-out duration-200 hover:scale-101 ' + 
        (status === 'solved' ? 'bg-green-700' : '') +
        (status === 'failed' ? 'bg-red-700' : '') +
        (status === 'judging' ? 'bg-yellow-700' : '') +
        (status === 'answering' ? 'bg-gray-300' : '')">
        <h1 v-if="status === 'unrevealed'" class="text-center">Unrevealed</h1>
        <h1 v-if="status === 'judging'" class="text-center text-white">Judging</h1>
        <h1 v-if="status === 'answering'" class="text-center">Answering</h1>
        <h1 v-if="status === 'solved'" class="text-center text-white">Solved</h1>
        <h1 v-if="status === 'failed'" class="text-center text-white">Failed</h1>
    </div>
</template>