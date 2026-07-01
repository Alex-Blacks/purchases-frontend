<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '@/composables/useAuth'

const email = ref('')
const password = ref('')
const error = ref('')
const { login } = useAuth()

async function handleLogin() {
    try {
        await login(email.value, password.value)
        error.value = ''
    } catch (e: any) {
        error.value = e.message
    }
}
</script>

<template>
    <div class="block">
        <h3>Вход</h3>
        <input v-model="email" placeholder="Email" />
        <input v-model="password" placeholder="Пароль" type="password" />
        <button @click="handleLogin">Войти</button>
        <p v-if="error" class="error">{{ error }}</p>
    </div>
</template>