<script setup lang="ts">
    import { useApi } from "../composables/useApi";

    const api = useApi();
    

    async function handlerListOrders() {
        try {
            const result = await api.listUsers()
            console.log(`Список заказов: ${JSON.stringify(result)}`);
        }catch ( error ) {
            console.log(`Ошибка: ${error}`);
        }
    }
</script>
<template>
    <div class="flex flex-col min-h-screen bg-gray-50">
        <header class="top-0 left-0 right-0 flex items-center justify-between h-16 px-4 bg-white border-b shadow-sm ">
            <div class="text-lg font-semibold text-gray-800">Мои заказы</div>
            <div>
                <button class="">Создать новую запись</button>
            </div>
        </header>
    </div>
</template>