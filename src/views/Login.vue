<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAuth } from "../composables/useAuth";

const email = ref('');
const password = ref('');
const router = useRouter();
const auth = useAuth();

const handleLogin = async () => {
    try {
        await auth.login(email.value,password.value);
        router.push('/dashboard');
    } catch ( error ) {
        console.log(`[ERROR] Login: ${error}`);
        throw new Error(`Ошибка: ${error}`);
    }
}
</script>

<template>
    <div class="flex items-center justify-center min-h-screen bg-gray-50">
        <div class="w-full max-w-md p-8 bg-white shadow-md rounded-xl">
            <h2 class="mb-6 text-2xl font-bold text-center text-gray-800">Вход</h2>

            <form @submit.prevent="handleLogin">
                <div class="mb-4">
                    <label class="block mb-1 text-sm font-medium text-gray-800">Email</label>
                    <input 
                        v-model="email" 
                        type="email" 
                        required 
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-transparent" 
                        placeholder="user@example.com"
                    />
                </div>

                <div class="mb-6">
                    <label class="block mb-1 text-sm font-medium text-gray-800">Password</label>
                    <input 
                        v-model="password"
                        type="password"
                        required
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg outline-none focus: focus:ring-2 focus:ring-teal-500 focus:border-transparent"
                        placeholder="••••••••"
                    />
                </div>

                <button 
                    type="submit"
                    class="w-full px-4 py-2 font-medium text-white transition duration-150 bg-teal-500 rounded-lg shadow-sm hover:bg-teal-600 hover:shadow-md"
                > 
                    Войти
                </button>    
            </form>

            <p class="mt-6 text-sm text-center text-gray-600">
                Нет аккаунта?
                <router-link to="/register" class="font-medium text-teal-500 hover:underline">
                    Зарегистрироваться
                </router-link>
            </p>
        </div>
    </div>
</template>