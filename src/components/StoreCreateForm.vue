<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAuth } from '../composables/useAuth'

const { token } = useAuth()
const name = ref('')
const result = ref('')

async function createStore() {
    if (!token.value) {
        result.value = 'Сначала войдите'
        return
    }
    try {
        const res = await invoke('create_store', {
            token: token.value,
            name: name.value
        })
        result.value = `Магазин создан: ${JSON.stringify(res)}`
    } catch (e) {
        result.value = `Ошибка: ${e}`
    }
}
</script>

<template>
    <div class="block">
        <h3>Создать магазин</h3>
        <input v-model="name" placeholder="Название" />
        <button @click="createStore">Создать</button>
        <p>{{ result }}</p>
    </div>
</template>