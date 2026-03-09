import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { User, Group } from '../api/types'

interface AuthState {
  token: string | null
  user: User | null
  group: Group | null
  
  setAuth: (token: string, user: User, group: Group | null) => void
  logout: () => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      token: null,
      user: null,
      group: null,
      
      setAuth: (token, user, group) => set({ token, user, group }),
      logout: () => set({ token: null, user: null, group: null }),
    }),
    {
      name: 'apihub-auth',
    }
  )
)

// 检查是否登录
export function useIsLoggedIn() {
  return useAuthStore((state) => state.token !== null)
}

// 获取当前用户
export function useCurrentUser() {
  return useAuthStore((state) => state.user)
}

// 获取当前群组
export function useCurrentGroup() {
  return useAuthStore((state) => state.group)
}
