import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ConfigProvider } from 'antd'
import zhCN from 'antd/locale/zh_CN'

import Layout from './components/Layout'
import Dashboard from './pages/Dashboard'
import Keys from './pages/Keys'
import Stats from './pages/Stats'
import Login from './pages/Login'
import Register from './pages/Register'
import Admin from './pages/Admin'
import Playground from './pages/Playground'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
    },
  },
})

const router = createBrowserRouter([
  {
    path: '/login',
    element: <Login />,
  },
  {
    path: '/register',
    element: <Register />,
  },
  {
    path: '/',
    element: <Layout />,
    children: [
      { index: true, element: <Dashboard /> },
      { path: 'keys', element: <Keys /> },
      { path: 'stats', element: <Stats /> },
      { path: 'playground', element: <Playground /> },
      { path: 'admin', element: <Admin /> },
    ],
  },
])

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ConfigProvider locale={zhCN}>
        <RouterProvider router={router} />
      </ConfigProvider>
    </QueryClientProvider>
  )
}

export default App
