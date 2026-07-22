<script setup lang="ts">
    import { ref, onMounted } from "vue";
    import { useApi } from "../composables/useApi";

    const api = useApi();
    const loading = ref(false);
    const error = ref('');
    const orders = ref<any>();
    
    onMounted(async () => {
        loading.value = true;
        error.value = '';
        try{
            const data = await api.listOrders();
            orders.value = data;
        } catch (err:any) {
            error.value = err.message || 'Ошибка загрузки заказа';
        } finally {
            loading.value = false
        }
    })

    function formatDate(dateString: string) {
    const date = new Date(dateString)
    return date.toLocaleString('ru-RU', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
    })
}
</script>
<template>
    <div class="flex flex-col min-h-screen bg-gray-50">
        <header class="top-0 left-0 right-0 flex items-center justify-between h-16 px-4 bg-white border-b shadow-sm ">
            <div class="text-lg font-semibold text-gray-800">Мои заказы</div>
            <div>
                <button class="">Создать новый заказ</button>
            </div>
        </header>
        <div class="flex flex-1 pt-4 top-16">
            <div v-if="loading" class="flex justify-center py-8">
                <div class="text-gray-700 ">Загрузка заказов...</div>
            </div>
            <div v-else-if="error" class="py-8 text-center text-red-500">
                {{ error }}
            </div>
            <div v-else class="overflow-x-auto bg-white rounded-md shadow-xl">
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                        <tr>
                            <th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase">ID</th>
                            <th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase">Пользователь</th>
                            <th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase">Магазин</th>
                            <th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase">Количество</th>
                            <th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase">Дата создания</th>
                            <th class="px-6 py-3 text-xs font-medium tracking-wider text-left text-gray-500 uppercase">Дата обновления</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-gray-200 ">
                        <tr v-for="order in orders" :key="order.id">
                            <td class="px-6 py-4 text-sm text-gray-900 border-r border-gray-200 whitespace-nowrap">{{ order.id }}</td>
                            <td class="px-6 py-4 text-sm text-gray-900 border-r border-gray-200 whitespace-nowrap">{{ order.userId }}</td>
                            <td class="px-6 py-4 text-sm text-gray-900 border-r border-gray-200 whitespace-nowrap">{{ order.store }}</td>
                            <td class="px-6 py-4 text-sm text-gray-900 border-r border-gray-200 whitespace-nowrap">{{ order.itemsCount }}</td>
                            <td class="px-6 py-4 text-sm text-gray-900 border-r border-gray-200 whitespace-nowrap">{{ formatDate(order.createdAt) }}</td>
                            <td class="px-6 py-4 text-sm text-gray-900 whitespace-nowrap">{{ formatDate(order.updatedAt) }}</td>
                        </tr>
                    </tbody>
                </table>

                <div v-if="orders && orders.length === 0" class="px-6 py-4 text-center text-gray-500">
                    У вас пока нет заказов
                </div>
            </div>
        </div>
    </div>
</template>