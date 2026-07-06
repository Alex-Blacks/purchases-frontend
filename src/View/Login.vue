<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '../composables/useAuth'

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
  <div id="app">
    <AppTabs />
    <router-view />
  </div>
  <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
    <h1>Авториция</h1>

    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Вход</h3>
      <input v-model="email" placeholder="Email" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="password" placeholder="Пароль" type="password" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleLogin">Войти</button>
    </div>
  </div>
</template>