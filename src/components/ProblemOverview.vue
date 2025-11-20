<script setup lang="ts">
import { ref, watch } from 'vue';
import { ProblemList } from '../problem';

const props = defineProps<{
    group: number,
    score: number,
    problemList: ProblemList;
}>();


let answering = ref(0);
let failed = ref(0);
let judging = ref(0);
let solved = ref(0);

setInterval(() => {
    for (let item of props.problemList) {
        let a: number = 0;
        let f: number = 0;
        let j: number = 0;
        let s: number = 0;
        if (item.group === props.group && item.score === props.score) {
            switch (item.status) {
                case "answering":
                    a++;
                    break;
                case "failed":
                    f++;
                    break;
                case "judging":
                    j++;
                    break;
                case "solved":
                    s++;
                    break;
            }
        }
        answering.value = a;
        failed.value = f;
        judging.value = j;
        solved.value = s;
    }
}, 200)

</script>

<template>
    <div class="rounded-3xl h-full w-full border border-gray-200 grid grid-cols-2 grid-rows-2 gap-1"> 
        <div class="mt-1 ml-1 h-auto w-auto rounded-3xl bg-gray-400 transition ease-in-out duration-200 hover:scale-101 hover:shadow-md opacity-60 flex items-center justify-center">
            <h1 class="text-2xl"> {{ answering }} </h1>
        </div>
        <div class="mt-1 mr-1 h-auto w-auto rounded-3xl bg-yellow-600 transition ease-in-out duration-200 hover:scale-101 hover:shadow-md opacity-80 flex items-center justify-center">
            <h1 class="text-2xl text-white"> {{ judging }} </h1>
        </div>
        <div class="mb-1 ml-1 h-auto w-auto rounded-3xl bg-red-700 transition ease-in-out duration-200 hover:scale-101 hover:shadow-md opacity-80 flex items-center justify-center">
            <h1 class="text-2xl text-white"> {{ failed }} </h1>
        </div>
        <div class="mb-1 mr-1 h-auto w-auto rounded-3xl bg-green-700 transition ease-in-out duration-200 hover:scale-101 hover:shadow-md opacity-80 flex items-center justify-center">
            <h1 class="text-2xl text-white"> {{ solved }} </h1>
        </div>
    </div>
</template>