import { createRouter, createWebHistory } from 'vue-router';
import Login from '../views/Login.vue';
import Register from '../views/Register.vue';
import Dashboard from '../views/Dashboard.vue';
import Orders from '../views/Orders.vue';
import Users from '../views/Users.vue';
import Products from '../views/Products.vue';
import Categories from '../views/Categories.vue';
import Stores from '../views/Stores.vue';
import Settings from '../views/Settings.vue';
import MainLayot from "../layouts/MainLayout.vue";


const routes = [
    { path: '/login', component: Login, meta: { requiresAuth: false }},
    { path: '/register', component: Register, meta: { requiresAuth: false } },
    { 
        path: '/', 
        component: MainLayot,
        meta: { requiresAuth: true },
        children: [
            { path: '', redirect: '/dashboard' },            
            { path: '/dashboard', component: Dashboard },
            { path: '/orders', component: Orders },
            { path: '/users', component: Users },
            { path: '/products', component: Products },
            { path: '/categories', component: Categories },
            { path: '/stores', component: Stores }, 
            { path: '/settings', component: Settings },                                   
        ]
    },
    
]

const router = createRouter ({ history: createWebHistory(), routes });

router.beforeEach((to,from,next) => {
    const token = localStorage.getItem('token');
    let isAuthenticated = !!token;
    console.log(`[Router] Переход: ${from.path} -> ${to.path}, auth: ${isAuthenticated}`);
    if (to.meta.requiresAuth && !isAuthenticated) {
        next('/login');
    } else if ((to.path === '/login' || to.path === '/register') && isAuthenticated) {
        next('/');
    } else { 
        next();
    }
})

export default router;