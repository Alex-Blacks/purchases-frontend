import { createRouter, createWebHistory } from 'vue-router';
import { useAuth } from "../composables/useAuth.ts";
import Login from '../views/Login.vue';
import Register from '../views/Register.vue';

const { token, isAuthenticated } = useAuth();

const routes = [
    { path: '/', redirect: '/login' },
    { path: '/login', component: Login },
    { path: '/register', component: Register },
]

const router = createRouter ({
    history: createWebHistory(),
    routes,
});

router.beforeEach((to,from,next) => {
    token: token.value
})

export default router;