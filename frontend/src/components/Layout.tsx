import { Outlet, Link, useLocation } from 'react-router-dom'
import { Layout as AntLayout, Menu } from 'antd'
import {
  DashboardOutlined,
  KeyOutlined,
  BarChartOutlined,
  CodeOutlined,
  SettingOutlined,
  UserOutlined,
} from '@ant-design/icons'

const { Sider, Content, Header } = AntLayout

const menuItems = [
  { key: '/', icon: <DashboardOutlined />, label: <Link to="/">仪表盘</Link> },
  { key: '/keys', icon: <KeyOutlined />, label: <Link to="/keys">Key 管理</Link> },
  { key: '/stats', icon: <BarChartOutlined />, label: <Link to="/stats">统计</Link> },
  { key: '/playground', icon: <CodeOutlined />, label: <Link to="/playground">Playground</Link> },
  { key: '/admin', icon: <SettingOutlined />, label: <Link to="/admin">管理</Link> },
]

export default function Layout() {
  const location = useLocation()

  return (
    <AntLayout style={{ minHeight: '100vh' }}>
      <Sider width={200} theme="light">
        <div style={{ 
          height: 64, 
          display: 'flex', 
          alignItems: 'center', 
          justifyContent: 'center',
          fontSize: 20,
          fontWeight: 'bold',
        }}>
          ApiHub
        </div>
        <Menu
          mode="inline"
          selectedKeys={[location.pathname]}
          items={menuItems}
        />
      </Sider>
      <AntLayout>
        <Header style={{ 
          background: '#fff', 
          padding: '0 24px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
        }}>
          <div />
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <UserOutlined />
            <span>用户名</span>
          </div>
        </Header>
        <Content style={{ margin: 24 }}>
          <Outlet />
        </Content>
      </AntLayout>
    </AntLayout>
  )
}
