<script setup lang="ts">
import { BarsArrowUpIcon, BarsArrowDownIcon } from "@heroicons/vue/24/outline";
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useAuth } from "../composables/useAuth";

const auth = useAuth();
const router = useRouter();
const isReferencesOpen = ref(false);

const handleLogout = async () => {
    await auth.logout();
    router.push('/login');
};

function toggleReferences() {
    isReferencesOpen.value = !isReferencesOpen.value;
};
</script>

<template>
    <div class="flex flex-col h-screen bg-gray-50">
        <header class="fixed top-0 left-0 right-0 z-10 flex items-center justify-between h-16 px-4 bg-white border-b shadow-sm">
            <div class="text-lg font-semibold text-gray-800">Мои покупки</div>
            <div>
                <button class="text-gray-800 bg-gray-300 rounded-lg hover:text-teal-500">Профиль</button>
            </div>
        </header>
        <div class="flex flex-1 pt-16 overflow-hidden">
            <aside class="bottom-0 left-0 flex flex-col w-64 overflow-y-auto bg-gray-100 border-r top-16">
                <nav class="flex-1 px-3 py-4 space-y-1">
                    <router-link 
                        to="/dashboard"
                        class="flex items-center px-4 py-2 text-sm font-medium text-gray-700 rounded-lg hover:bg-gray-200 hover:text-teal-600"
                        active-class="text-teal-700 bg-teal-50"
                    >
                        Главная
                    </router-link>
                    <router-link 
                        to="/orders"
                        class="flex items-center px-4 py-2 text-sm font-medium text-gray-700 rounded-lg hover:bg-gray-200 hover:text-teal-600"
                        active-class="text-teal-700 bg-teal-50"
                    >
                        Корзина
                    </router-link>
                    <div>
                        <div 
                            @click="toggleReferences"
                            class="flex items-center justify-between px-4 py-2 text-sm font-medium text-gray-700 rounded-lg cursor-pointer hover:bg-gray-200 hover:text-teal-600"
                        >
                            <span>Справочники</span>
                            <component
                                :is="isReferencesOpen ? BarsArrowUpIcon : BarsArrowDownIcon"
                                class="w-5 h-5 transition-transform duration-200"
                            />
                        </div>
                        <div v-show="isReferencesOpen" class="ml-4 space-y-1">
                            <router-link 
                                to="/users"
                                class="block px-4 py-2 text-sm text-gray-600 rounded-lg hover:bg-gray-200 hover:text-teal-600"
                                active-class="text-teal-700 bg-teal-50"
                            >
                                Пользователи
                            </router-link>
                            <router-link 
                                to="/products"
                                class="block px-4 py-2 text-sm text-gray-600 rounded-lg hover:bg-gray-200 hover:text-teal-600"
                                active-class="text-teal-700 bg-teal-50"
                            >
                                Товары
                            </router-link>
                            <router-link 
                                to="/stores"
                                class="block px-4 py-2 text-sm text-gray-600 rounded-lg hover:bg-gray-200 hover:text-teal-600"
                                active-class="text-teal-700 bg-teal-50"
                            >
                                Магазины
                            </router-link>
                            <router-link 
                                to="/categories"
                                class="block px-4 py-2 text-sm text-gray-600 rounded-lg hover:bg-gray-200 hover:text-teal-600"
                                active-class="text-teal-700 bg-teal-50"
                            >
                                Категории
                            </router-link>
                        </div>
                    </div>
                </nav>
                <router-link 
                    to="/settings" 
                    class="flex items-center px-4 py-2 text-sm font-medium text-gray-700 rounded-lg hover:bg-gray-200 hover:text-teal-600"
                    active-class="text-teal-700 bg-teal-50"
                >
                    Настройки
                </router-link>
                <div class="p-3 border-t border-gray-300 ">
                    <button 
                        @click="handleLogout" 
                        class="flex items-center justify-center w-full px-4 py-2 mt-auto text-sm font-medium text-white bg-red-500 rounded-lg hover:bg-red-600"
                    >
                        Выйти
                    </button>
                </div>
            </aside>
            <main class="flex-1 p-6 overflow-y-auto">
                <router-view />
            </main>
        </div>
    </div>
</template>