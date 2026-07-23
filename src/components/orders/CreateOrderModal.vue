<script setup lang="ts">
import { emit } from '@tauri-apps/api/event';

emit('close')
</script>

<template>
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        <div class="max-w-2xl overflow-y-auto bg-white shadow-2xl rounded-xl max-h-[90vh] p-6">
            <header class="flex items-center justify-between mb-4 ">
                <div class="text-xl font-semibold">Создание заказа</div>
                <div>
                    <button class="text-2xl leading-none text-gray-400 hover:text-gray-600">✕</button>
                </div>
            </header>
            <div class="mb-4 ">
                <label class="block mb-1 text-sm font-medium text-gray-700 ">Магазин:</label>
                <select class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-teal-500 focus:border-transparent">
                    <option>Выберите магазин</option>
                </select>
            </div>
            <div>
                <label class="mb-2 text-sm font-medium text-gray-700">Товары:</label>
                <div class="flex items-center gap-2 mb-4">
                    <select class="flex-1 px-4 py-2 border border-gray-300 rounded-lg">
                        <option>Выберите товар</option>
                    </select>
                    <input type="number" value="1" class="w-20 px-4 py-2 border border-gray-300 rounded-lg">
                    <button class="p-1 text-red-500 hover:text-red-700">✕</button>
                </div>
                <div class="flex justify-center">
                    <button class="flex items-center gap-1 mt-1 text-sm font-medium text-teal-500 hover:text-teal-600">+ Добавить товар</button>
                </div>
            </div>
            <div class="flex justify-end gap-2 pt-4 mt-6 border-t border-gray-200">
                <button class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200">Отмена</button>
                <button class="px-4 py-2 text-sm font-medium text-white bg-teal-500 rounded-lg hover:bg-teal-600">Создать заказ</button>
            </div>
        </div>
    </div>
</template>