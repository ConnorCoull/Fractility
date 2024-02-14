import { createApp } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';

import App from './App.vue';
import FractalComponent from './components/FractalComponent.vue';
import FractalComponentJS from './components/FractalComponentJS.vue';

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: FractalComponent},
        { path: '/Rust', component: FractalComponent },
        { path: '/JavaScript', component: FractalComponentJS },
    ]
});

const app = createApp(App);
app.use(router);
app.mount('#app');
