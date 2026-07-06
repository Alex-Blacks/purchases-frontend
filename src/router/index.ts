import { createRouter, createWebHistory } from 'vue-router';
import loginView from '../View/Login.vue';
import storeView from '../View/Stores.vue';
import categoryView from '../View/Categories.vue'
import productView from '../View/Products.vue'
import userView from '../View/Users.vue';

const routes = [
    { path: '/', redirect: '/login' },
    { path: '/login', component: loginView },
    { path: '/stores', component: storeView },
    { path: '/categories', component: categoryView },
    { path: '/products', component: productView },
    { path: '/users', component: userView },
]

const router = createRouter ({
    history: createWebHistory(),
    routes,
});

export default router;