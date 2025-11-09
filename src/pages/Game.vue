<script setup lang="ts">
import { router } from '../router';
import Button from '../components/Button.vue';
import { app } from '../main';
import ProblemView from '../components/ProblemView.vue';
import { ProblemStatus } from '../problem';
import { getProblemList } from '../core';

let problemList = new Array<Array<ProblemStatus>>(8).fill(new Array(5).fill('unrevealed' as ProblemStatus));

function back() {
    router.replace("/");
    app.$cookies.remove("group");
}

function enabled(group: number) {
    return app.$route.params.id === group.toString();
}

setInterval(async () => {
    const list = await getProblemList();
    for (let item of list) 
        problemList[item.column][item.score / 100] = item.status;
}, 1000);
</script>
<template>
    <div class="h-full select-none">
        <div class="flex flex-row gap-4 bg-amber-100 h-15">
            <Button @click="back()" class="ml-2.5 mt-auto mb-auto"> 
                <p class="mt-auto mb-auto mr-auto ml-auto relative">< Back</p> 
            </Button>
            <h1 class="mt-auto mb-auto ml-5">Playing as Group {{ $route.params.id }}</h1>
        </div>

        <div class="grid grid-cols-8 grid-rows-5 h-[90%] w-[90%] mt-10 ml-auto mr-auto mb-auto border rounded-4xl border-gray-300 gap-2 p-2">
            <div class="row-start-1 row-end-2 col-start-1 col-end-2 bg-gray-200 rounded-3xl"/>
            <div class="row-start-2 row-end-6 col-start-1 col-end-2 bg-gray-100 rounded-3xl flex flex-col pl-4 pr-4">
                <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                    <h1 class="text-center text-6xl h-fit">100</h1>
                </div>
                <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                    <h1 class="text-center text-6xl h-fit">200</h1>
                </div>
                <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                    <h1 class="text-center text-6xl h-fit">300</h1>
                </div>
                <div class="flex-auto flex justify-center items-center">
                    <h1 class="text-center text-6xl h-fit">400</h1>
                </div>
            </div>
            <div class="col-start-2 col-end-9 row-start-1 row-end-2 bg-gray-100 rounded-3xl grid grid-cols-7 grid-rows-1 pt-6 pb-6">
                <div :class="'flex items-center border-r border-r-gray-300' + (enabled(1) ? ' bg-gray-300' : '')">
                    <h1 class="text-center text-6xl h-fit ml-auto mr-auto">G1</h1>
                </div>
                <div :class="'flex items-center border-r border-r-gray-300' + (enabled(2) ? ' bg-gray-300' : '')">
                    <h1 class="text-center text-6xl h-fit ml-auto mr-auto">G2</h1>
                </div>
                <div :class="'flex items-center border-r border-r-gray-300' + (enabled(3) ? ' bg-gray-300' : '')">
                    <h1 class="text-center text-6xl h-fit ml-auto mr-auto">G3</h1>
                </div>
                <div :class="'flex items-center border-r border-r-gray-300' + (enabled(4) ? ' bg-gray-300' : '')">
                    <h1 class="text-center text-6xl h-fit ml-auto mr-auto">G4</h1>
                </div>
                <div :class="'flex items-center border-r border-r-gray-300' + (enabled(5) ? ' bg-gray-300' : '')">
                    <h1 class="text-center text-6xl h-fit ml-auto mr-auto">G5</h1>
                </div>
                <div :class="'flex items-center border-r border-r-gray-300' + (enabled(6) ? ' bg-gray-300' : '')">
                    <h1 class="text-center text-6xl h-fit ml-auto mr-auto">G6</h1>
                </div>
                <div :class="'flex items-center' + (enabled(7) ? ' bg-gray-300' : '')">
                    <h1 class="text-center text-6xl h-fit ml-auto mr-auto">G7</h1>
                </div>
            </div>
            <ProblemView :enabled="enabled(1)" :status="problemList[1][1]"/>
            <ProblemView :enabled="enabled(2)" :status="problemList[2][1]"/>
            <ProblemView :enabled="enabled(3)" :status="problemList[3][1]"/>
            <ProblemView :enabled="enabled(4)" :status="problemList[4][1]"/>
            <ProblemView :enabled="enabled(5)" :status="problemList[5][1]"/>
            <ProblemView :enabled="enabled(6)" :status="problemList[6][1]"/>
            <ProblemView :enabled="enabled(7)" :status="problemList[7][1]"/>
            <ProblemView :enabled="enabled(1)" :status="problemList[1][2]"/>
            <ProblemView :enabled="enabled(2)" :status="problemList[2][2]"/>
            <ProblemView :enabled="enabled(3)" :status="problemList[3][2]"/>
            <ProblemView :enabled="enabled(4)" :status="problemList[4][2]"/>
            <ProblemView :enabled="enabled(5)" :status="problemList[5][2]"/>
            <ProblemView :enabled="enabled(6)" :status="problemList[6][2]"/>
            <ProblemView :enabled="enabled(7)" :status="problemList[7][2]"/>
            <ProblemView :enabled="enabled(1)" :status="problemList[1][3]"/>
            <ProblemView :enabled="enabled(2)" :status="problemList[2][3]"/>
            <ProblemView :enabled="enabled(3)" :status="problemList[3][3]"/>
            <ProblemView :enabled="enabled(4)" :status="problemList[4][3]"/>
            <ProblemView :enabled="enabled(5)" :status="problemList[5][3]"/>
            <ProblemView :enabled="enabled(6)" :status="problemList[6][3]"/>
            <ProblemView :enabled="enabled(7)" :status="problemList[7][3]"/>
            <ProblemView :enabled="enabled(1)" :status="problemList[1][4]"/>
            <ProblemView :enabled="enabled(2)" :status="problemList[2][4]"/>
            <ProblemView :enabled="enabled(3)" :status="problemList[3][4]"/>
            <ProblemView :enabled="enabled(4)" :status="problemList[4][4]"/>
            <ProblemView :enabled="enabled(5)" :status="problemList[5][4]"/>
            <ProblemView :enabled="enabled(6)" :status="problemList[6][4]"/>
            <ProblemView :enabled="enabled(7)" :status="problemList[7][4]"/>
        </div>
    </div>
</template>