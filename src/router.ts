import { createMemoryHistory, createRouter } from 'vue-router';
import Entry from './pages/Entry.vue';
import Game from './pages/Game.vue';

const routes = [
    {
        path: '/',
        component: Entry
    },
    {
        path: '/group/:id',
        component: Game
    }
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes: routes
})