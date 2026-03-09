// API 类型定义

export interface User {
  id: string
  username: string
  email: string
  credits: number
  is_admin: boolean
  created_at: string
}

export interface Group {
  id: string
  name: string
  invite_code?: string
  owner_id?: string
}

export interface Key {
  id: string
  provider: string
  name?: string
  contributor: {
    id: string
    username: string
  }
  monthly_quota?: number
  used_quota: number
  weight: number
  is_active: boolean
  last_used_at?: string
  created_at: string
}

export interface UsageStats {
  period: string
  summary: {
    total_requests: number
    total_tokens: number
    total_cost_usd: number
    unique_users: number
    avg_latency_ms: number
  }
  by_model: Array<{
    model: string
    requests: number
    tokens: number
    cost_usd: number
  }>
  by_user: Array<{
    user_id: string
    username: string
    requests: number
    tokens: number
    cost_usd: number
  }>
}

export interface LeaderboardEntry {
  rank: number
  user_id: string
  username: string
  credits: number
  key_count: number
  avatar?: string
}

export interface ChatMessage {
  role: 'system' | 'user' | 'assistant'
  content: string
}

export interface ChatRequest {
  model: string
  messages: ChatMessage[]
  temperature?: number
  max_tokens?: number
  stream?: boolean
}

export interface ChatResponse {
  id: string
  object: string
  created: number
  model: string
  choices: Array<{
    index: number
    message: ChatMessage
    finish_reason: string
  }>
  usage: {
    prompt_tokens: number
    completion_tokens: number
    total_tokens: number
  }
}

export interface ApiError {
  success: false
  error: {
    code: string
    message: string
  }
}

export interface ApiResponse<T> {
  success: boolean
  data: T
}
