<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '../composables/useAuth'

const email = ref('')
const password = ref('')
const error = ref('')
const { login, logout } = useAuth()

const loginResult = ref('')

async function handleLogin() {
    try {
        await login(email.value, password.value)
        error.value = ''
        loginResult.value = 'Логин успешен'
    } catch (e: any) {
        error.value = e.message
        loginResult.value = 'Ошибка'
    }
}

async function handleLogout() {
    logout(); 
    
    alert('Вы вышли из системы');
}
</script>

<template>

</template>