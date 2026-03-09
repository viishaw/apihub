import { useState } from 'react'
import { Card, Form, Input, Button, message } from 'antd'
import { UserOutlined, LockOutlined } from '@ant-design/icons'
import { useNavigate, Link } from 'react-router-dom'

export default function Login() {
  const [loading, setLoading] = useState(false)
  const navigate = useNavigate()

  const handleSubmit = async (values: any) => {
    setLoading(true)
    
    // TODO: 调用实际 API
    setTimeout(() => {
      message.success('登录成功')
      navigate('/')
      setLoading(false)
    }, 1000)
  }

  return (
    <div style={{ 
      height: '100vh', 
      display: 'flex', 
      alignItems: 'center', 
      justifyContent: 'center',
      background: '#f0f2f5',
    }}>
      <Card style={{ width: 400 }}>
        <h1 style={{ textAlign: 'center', marginBottom: 24 }}>ApiHub</h1>
        
        <Form onFinish={handleSubmit}>
          <Form.Item
            name="username"
            rules={[{ required: true, message: '请输入用户名' }]}
          >
            <Input 
              prefix={<UserOutlined />} 
              placeholder="用户名" 
              size="large"
            />
          </Form.Item>

          <Form.Item
            name="password"
            rules={[{ required: true, message: '请输入密码' }]}
          >
            <Input.Password
              prefix={<LockOutlined />}
              placeholder="密码"
              size="large"
            />
          </Form.Item>

          <Form.Item>
            <Button 
              type="primary" 
              htmlType="submit" 
              size="large"
              loading={loading}
              block
            >
              登录
            </Button>
          </Form.Item>

          <div style={{ textAlign: 'center' }}>
            还没有账号？ <Link to="/register">注册群组</Link> 或 <Link to="/register">加入群组</Link>
          </div>
        </Form>
      </Card>
    </div>
  )
}
