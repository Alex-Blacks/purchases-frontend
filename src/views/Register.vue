<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import { useAuth } from "../composables/useAuth";

const name = ref('');
const email = ref('');
const password = ref('');
const passwordConfirmation = ref('');
const router = useRouter();
const auth = useAuth();

const passwordMatch = computed(() => {
    return password.value === passwordConfirmation.value;
})

const handleRegister = async () => {
    if (!passwordMatch) {
        alert('Пароли не совпадают!');
        return;
    }
    try {
        await auth.register(name.value, email.value, password.value);
        router.push('/');
    } catch ( error ) {
        console.log(`[ERROR] Register: ${error}`);
        throw new Error(`Ошибка: ${error}`);
    }
}

</script>

<template>
    <div class="flex items-center justify-center min-h-screen bg-gray-50">
        <div class="w-full max-w-md p-8 bg-white shadow-md rounded-xl">
            <h2 class="mb-6 text-2xl font-bold text-center text-gray-800">Регистрация</h2>

            <form @submit.prevent="handleRegister">
                <div class="mb-4">
                    <label class="block mb-1 text-sm font-medium text-gray-700">Имя</label>
                    <input
                        v-model="name"
                        type="text"
                        required
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-transparent" 
                        placeholder="Иван"
                    />
                </div>

                <div class="mb-4">
                    <label class="block mb-1 text-sm font-medium text-gray-700">Email</label>
                    <input 
                        v-model="email"
                        type="email"
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-transparent"
                        placeholder="user@example.com"
                    />
                </div>

                <div class="mb-4">
                    <label class="block mb-1 text-sm font-medium text-gray-700">Пароль</label>
                    <input
                        v-model="password"
                        type="password"
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-transparent"
                        placeholder="••••••••"
                    />
                </div>

                <div class="mb-4">
                    <label class="block mb-1 text-sm font-medium text-gray-700">Подтверждение пароля</label>
                    <input
                        v-model="passwordConfirmation"
                        type="password"
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-transparent"
                        :class="{
                            'border-gray-300': passwordMatch || passwordConfirmation === '',
                            'border-red-500': !passwordMatch && passwordConfirmation !== ''
                        }"
                        placeholder="••••••••"
                    />
                    <p v-if="!passwordMatch && passwordConfirmation !== ''" class="mt-1 text-xs text-red-500">Пароли не совпадают</p>
                </div>

                <button
                    type="submit"
                    class="w-full px-4 py-2 font-medium text-white transition duration-150 bg-teal-500 rounded-lg shadow-sm hover:bg-teal-600 hover:shadow-md"
                    :disabled="!passwordMatch && passwordConfirmation !== ''"
                >
                    Зарегистрироваться
                </button>
            </form>

            <p class="mt-6 text-sm text-center text-gray-600">
                Уже есть аккаунт?
                <router-link to="/login" class="font-medium text-teal-500 hover:underline">
                    Войти
                </router-link>
            </p>
        </div>
    </div>
</template>