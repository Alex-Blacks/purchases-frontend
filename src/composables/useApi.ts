// src/composables/useApi.ts
import { invoke } from '@tauri-apps/api/core'
import { useAuth } from './useAuth'

export function useApi() {
    const { token } = useAuth()

    async function call<T>(command: string, params: Record<string, any> = {}): Promise<T> {
        const finalParams = token.value ? { ...params, token: token.value } : params
        return await invoke<T>(command, finalParams)
    }

    return { call }
}