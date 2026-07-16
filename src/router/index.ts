import { createRouter, createWebHistory } from 'vue-router';
import Login from '../views/Login.vue';
import Register from '../views/Register.vue';
import Dashboard from '../views/Dashboard.vue';

const routes = [
    { path: '/', redirect: '/dashboard' },
    { path: '/login', component: Login, meta: { requiresAuth: false }},
    { path: '/register', component: Register, meta: { requiresAuth: false } },
    { path: '/dashboard', component: Dashboard, meta: { requiresAuth: true } },
]

const router = createRouter ({
    history: createWebHistory(),
    routes,
});

router.beforeEach((to,from,next) => {
    const token = localStorage.getItem('token');
    let isAuthenticated = !!token;
    console.log(`[Router] Переход: ${from.path} -> ${to.path}, auth: ${isAuthenticated}`);
    if (to.meta.requiresAuth && !isAuthenticated) {
        next('/login');
    } else if ((to.path === '/login' || to.path === '/register') && isAuthenticated) {
        next('/dashboard');
    } else { 
        next();
    }
})

export default router;