import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { authApi, keyApi, groupApi, statsApi, adminApi } from '../api/client'
import type { 
  RegisterRequest, 
  LoginRequest, 
  CreateKeyRequest,
  UsageQuery,
  AdjustCreditsRequest,
} from '../api/types'

// ============ 认证 Hooks ============

export function useLogin() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (req: LoginRequest) => authApi.login(req.username, req.password),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['auth'] })
    },
  })
}

export function useRegister() {
  return useMutation({
    mutationFn: (req: RegisterRequest) => authApi.register(req),
  })
}

export function useMe() {
  return useQuery({
    queryKey: ['auth', 'me'],
    queryFn: () => authApi.me(),
    retry: false,
  })
}

// ============ Key Hooks ============

export function useKeys() {
  return useQuery({
    queryKey: ['keys'],
    queryFn: () => keyApi.list(),
  })
}

export function useMyKeys() {
  return useQuery({
    queryKey: ['keys', 'my'],
    queryFn: () => keyApi.myKeys(),
  })
}

export function useCreateKey() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (req: CreateKeyRequest) => keyApi.create(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['keys'] })
    },
  })
}

export function useToggleKey() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (id: string) => keyApi.toggle(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['keys'] })
    },
  })
}

export function useDeleteKey() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (id: string) => keyApi.delete(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['keys'] })
    },
  })
}

// ============ 统计 Hooks ============

export function useUsageStats(params?: UsageQuery) {
  return useQuery({
    queryKey: ['stats', 'usage', params],
    queryFn: () => statsApi.usage(params),
  })
}

export function useLeaderboard() {
  return useQuery({
    queryKey: ['stats', 'leaderboard'],
    queryFn: () => statsApi.leaderboard(),
  })
}

// ============ 管理员 Hooks ============

export function useAdminUsers() {
  return useQuery({
    queryKey: ['admin', 'users'],
    queryFn: () => adminApi.users(),
  })
}

export function useAdjustCredits() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: ({ userId, amount, reason }: { userId: string } & AdjustCreditsRequest) => 
      adminApi.adjustCredits(userId, amount, reason),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'users'] })
    },
  })
}
