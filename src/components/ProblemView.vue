<script setup lang="ts">
import { ref, watch } from 'vue';
import { ProblemList, ProblemStatus } from '../problem';
import { getGroupStatus, nextProblem } from '../core';

const props = defineProps<{
    problemStatusList: Array<ProblemStatus>,
    problemList: ProblemList,
    group: number
}>();

let progress = ref(0);

watch(props.problemStatusList, () => {
    let revealed = 0;
    for (let problemStatus of props.problemStatusList) 
        if (problemStatus !== 'unrevealed')
            revealed++;
    progress.value = revealed;
})

setInterval(async () => {
    for (let groupStatus of await getGroupStatus()) {
        if (groupStatus.id == props.group)
            progress.value = groupStatus.progress
    };
}, 1000)
</script>

<template>
    <div class="flex flex-col">
        <div class="ml-3 mt-3 w-40 h-10 rounded-3xl bg-gray-300 flex items-center justify-center">
            <h1 class="text-xl">Progress: {{ progress }}/4</h1>
        </div>
        <div v-if="progress === 0" class="transition duration-300 mt-3 mb-3 h-full w-auto ml-3 mr-3 border border-gray-300 rounded-[20px] flex items-center justify-center hover:shadow-md hover:active:scale-99" @click="nextProblem(Number(props.group))">
            <h1 class="text-3xl">Click to start</h1>
        </div>
    </div>
</template>