<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAuth } from '../../composables/useAuth'

const { token } = useAuth();
const name = ref('');
const result = ref('');

async function createStore() {
    if (!token.value) {
        result.value = 'Сначала войдите';
        return;
    }
    try {
        const res = await invoke('create_store', {
            token: token.value,
            name: name.value
        });
        result.value = `Магазин создан: ${JSON.stringify(res)}`;
    } catch (e) {
        result.value = `Ошибка: ${e}`;
    }
}
</script>

<template>
    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Создать магазин</h3>
      <input v-model="name" placeholder="Название магазина" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="createStore">Создать магазин</button>
      <p style="margin-top:8px;">{{ result }}</p>
    </div>
</template>