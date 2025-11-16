<script setup lang="ts">
import { ref, watch } from 'vue';
import { Problem, ProblemList, ProblemStatus } from '../problem';
import { getGroupStatus, nextProblem } from '../core';

const props = defineProps<{
    problemStatusList: Array<ProblemStatus>,
    problemList: ProblemList,
    group: number
}>();

const progress = ref(0);
const reveal = ref(false);
const end = ref(false);
const timeout = ref(Number.MAX_VALUE);
const currentProblem = ref({
    id: -1,
    question: "There is a problem here.",
    answer: "none",
    limit: 600,
    score: 100
} as Problem);

watch(props.problemStatusList, () => {
    let revealed = 0;
    for (let problemStatus of props.problemStatusList) 
        if (problemStatus !== 'unrevealed')
            revealed++;
    progress.value = revealed;
})

setInterval(() => {
    for (let item of props.problemList) {
        if (item.column == props.group && item.status == "answering") {
            if (item.end)
                timeout.value = item.end - Math.round(Date.now() / 1000);
            if (timeout.value <= 0)
                next();
            return;
        }
    }
}, 200)

setInterval(async () => {
    for (let groupStatus of await getGroupStatus()) {
        if (groupStatus.id == props.group)
            setTimeout(() => {
                progress.value = groupStatus.progress;
                end.value = groupStatus.end;
            }, 200);
    };
}, 1000)

function start() {
    if (progress.value == 0) 
        nextProblem(Number(props.group));
    reveal.value = true;
    return;
}

function next() {
    nextProblem(Number(props.group));
}
</script>

<template>
    <div v-if="!end" class="transition duration-300 h-[90%] ml-0 mr-auto mt-auto mb-auto border rounded-4xl border-gray-300 w-[40%]" :class="{ collapse: end }">
        <Suspense>
            <div class="grid grid-rows-10 grid-cols-1 h-full gap-0">
                <div class="row-span-1 ml-3 mt-3 w-40 h-10 rounded-3xl bg-gray-300 flex items-center justify-center">
                    <h1 class="text-xl">Progress: {{ progress }}/4</h1>
                </div>
                <div v-if="progress === 0 || !reveal" class="row-span-9 transition duration-300 mt-3 mb-3 h-[96%] w-auto ml-3 mr-3 border border-gray-300 rounded-[20px] flex items-center justify-center hover:shadow-md hover:active:scale-99" @click="start()">
                    <h1 class="text-3xl">Click to start</h1>
                </div>
                <div v-if="progress !== 0 && reveal" class="row-span-9 transition duration-300 mt-3 mb-3 h-[96%] w-auto ml-3 mr-3 pl-2 pr-2">
                    <h1 class="text-2xl wrap-break-word overflow-hidden text-gray-600">Timeout: {{ Math.round(timeout).toFixed(0) }}s</h1>
                    <h1 class="text-3xl wrap-break-word overflow-hidden mt-5">{{ currentProblem.question }}</h1>
                </div>
            </div>
        </Suspense>
    </div>
</template>