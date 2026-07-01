<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const name = ref('')
const password = ref('')
const email = ref('')
const result = ref('')

async function createUser() {
    try {
        const res = await invoke('create_user', {
            name: name.value,
            password: password.value,
            email: email.value,
            role: 'user'
        })
        result.value = `Пользователь создан: ${JSON.stringify(res)}`
    } catch (e) {
        result.value = `Ошибка: ${e}`
    }
}
</script>

<template>
    <div class="block">
        <h3>Создать пользователя</h3>
        <input v-model="name" placeholder="Имя" />
        <input v-model="password" placeholder="Пароль" type="password" />
        <input v-model="email" placeholder="Email" />
        <button @click="createUser">Создать</button>
        <p>{{ result }}</p>
    </div>
</template>