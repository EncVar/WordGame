<script setup lang="ts">
import { router } from '../router';
import Button from '../components/Button.vue';

function back() {
    router.replace("/");
    return;
}
</script>

<template>
    <div>
        <div class="flex flex-row gap-4 bg-amber-100 h-15">
            <Button @click="back()" class="ml-2.5 mt-auto mb-auto"> 
                <p class="mt-auto mb-auto mr-auto ml-auto relative">< Back</p> 
            </Button>
            <h1 class="mt-auto mb-auto ml-5">Judging Group {{ $route.params.id }}</h1>
        </div>
    </div>
</template>