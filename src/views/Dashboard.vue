<script setup lang="ts">
import { HomeIcon, MagnifyingGlassIcon, ShoppingCartIcon, BellIcon, UserIcon, ArrowRightEndOnRectangleIcon } from "@heroicons/vue/24/outline";


</script>

<template>
    <div class="flex flex-col min-h-screen bg-gray-50">
        <header class="flex items-center justify-between w-full h-16 px-4 bg-white shadow-sm">
            <div class="text-lg font-semibold">Мои покупки</div>
            <div>
                <button class="flex flex-col items-center text-gray-600 hover:text-teal-500">
                    <ArrowRightEndOnRectangleIcon class="w-6 h-6"/>
                    <span class="mt-1 text-xs">Выйти</span>
                </button>
            </div>
        </header>
        <main class="flex items-center justify-center flex-1">
            <h1 class="text-3xl font-light text-gray-700 ">Добро пожаловать, Александр</h1>
        </main>
        <footer>
            <nav class="flex items-center justify-around w-full h-16 py-3 bg-white border-t border-gray-200">
                <router-link to="/dashboard" class="flex flex-col items-center text-gray-600 hover:text-teal-500">
                    <HomeIcon class="w-6 h-6"/>
                    <span class="mt-1 text-xs">Главная</span>
                </router-link>
                <router-link to="/search" class="flex flex-col items-center text-gray-600 hover:text-teal-500">
                    <MagnifyingGlassIcon class="w-6 h-6"/>
                    <span class="mt-1 text-xs">Поиск</span>
                </router-link>
                <router-link to="/orders" class="flex flex-col items-center text-gray-600 hover:text-teal-500">
                    <ShoppingCartIcon class="w-10 h-10"/>
                    <span class="mt-1 text-xs">Корзина</span>
                </router-link>
                <router-link to="/notifications" class="flex flex-col items-center text-gray-600 hover:text-teal-500">
                    <BellIcon class="w-6 h-6"/>
                    <span class="mt-1 text-xs">Уведомления</span>
                </router-link>
                <router-link to="/profile" class="flex flex-col items-center text-gray-600 hover:text-teal-500">
                    <UserIcon class="w-6 h-6"/>
                    <span class="mt-1 text-xs">Профиль</span>
                </router-link>
            </nav>
        </footer>
    </div>
</template>