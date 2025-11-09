<script setup lang="ts">
import { ProblemStatus } from '../problem';

defineProps<{
    enabled: boolean;
    status: ProblemStatus;
}>();
</script>

<template>
    <div :class="'rounded-3xl h-full w-full flex justify-center items-center hover:shadow-md border border-gray-200 transition ease-in-out duration-200 hover:scale-101 ' + 
        (status === 'solved' ? ' bg-green-700' : '') +
        (status === 'failed' ? ' bg-red-700' : '') +
        (status === 'judging' ? ' bg-yellow-700' : '') +
        (status === 'pending' ? ' bg-gray-300' : '') ">
        <h1 v-if="(!enabled) && status === 'unrevealed'" class="text-center">Unrevealed</h1>
        <h1 v-if="enabled && status === 'unrevealed'" class="text-center">Click to reveal</h1>
        <h1 v-if="status === 'judging'" class="text-center">Judging</h1>
        <h1 v-if="status === 'pending'" class="text-center">Pending</h1>
        <h1 v-if="status === 'solved'" class="text-center text-white">Solved</h1>
        <h1 v-if="status === 'failed'" class="text-center text-white">Failed</h1>
    </div>
</template>