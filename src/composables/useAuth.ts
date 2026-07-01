import { ref, readonly } from 'vue' 
import { invoke } from '@tauri-apps/api/core'

const token = ref<string | null>(null)
const isAuthenticated = ref(false)

export function useAuth() {
    async function login(email: string, password: string): Promise<void> {
        try {
            const result = await invoke<string>('login', { email, password })
            token.value = result
            isAuthenticated.value = true
        } catch (error) {
            throw new Error(`Ошибка входа: ${error}`)
        }
    }
    function logout() {
        token.value = null
        isAuthenticated.value = false
    }
    return {
        token: readonly(token),
        isAuthenticated: readonly(isAuthenticated),
        login,
        logout
    }
}