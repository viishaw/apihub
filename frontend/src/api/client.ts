import axios from 'axios'

const client = axios.create({
  baseURL: '/api/v1',
  timeout: 30000,
})

// 请求拦截器
client.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// 响应拦截器
client.interceptors.response.use(
  (response) => response.data,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

// 认证 API
export const authApi = {
  login: (username: string, password: string) =>
    client.post('/auth/login', { username, password }),
  
  register: (data: any) =>
    client.post('/auth/register', data),
  
  me: () =>
    client.get('/auth/me'),
  
  changePassword: (oldPassword: string, newPassword: string) =>
    client.put('/auth/password', { oldPassword, newPassword }),
}

// Key API
export const keyApi = {
  list: () =>
    client.get('/keys'),
  
  myKeys: () =>
    client.get('/keys/my'),
  
  create: (data: any) =>
    client.post('/keys', data),
  
  update: (id: string, data: any) =>
    client.put(`/keys/${id}`, data),
  
  delete: (id: string) =>
    client.delete(`/keys/${id}`),
  
  toggle: (id: string) =>
    client.post(`/keys/${id}/toggle`),
}

// 群组 API
export const groupApi = {
  get: (id: string) =>
    client.get(`/groups/${id}`),
  
  members: (id: string) =>
    client.get(`/groups/${id}/members`),
  
  update: (id: string, data: any) =>
    client.put(`/groups/${id}`, data),
  
  regenerateInvite: (id: string) =>
    client.post(`/groups/${id}/regenerate-invite`),
  
  removeMember: (groupId: string, userId: string) =>
    client.delete(`/groups/${groupId}/members/${userId}`),
  
  leave: (id: string) =>
    client.post(`/groups/${id}/leave`),
}

// 统计 API
export const statsApi = {
  usage: (params?: any) =>
    client.get('/stats/usage', { params }),
  
  contribution: (params?: any) =>
    client.get('/stats/contribution', { params }),
  
  leaderboard: () =>
    client.get('/stats/leaderboard'),
  
  credits: () =>
    client.get('/stats/credits'),
}

// 管理员 API
export const adminApi = {
  users: (params?: any) =>
    client.get('/admin/users', { params }),
  
  adjustCredits: (userId: string, amount: number, reason: string) =>
    client.post(`/admin/users/${userId}/adjust-credits`, { amount, reason }),
  
  toggleUser: (userId: string) =>
    client.post(`/admin/users/${userId}/toggle`),
  
  stats: () =>
    client.get('/admin/stats'),
}

export default client
