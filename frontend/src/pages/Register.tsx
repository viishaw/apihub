import { useState } from 'react'
import { Card, Form, Input, Button, message, Radio, Steps } from 'antd'
import { UserOutlined, LockOutlined, MailOutlined, TeamOutlined, GiftOutlined } from '@ant-design/icons'
import { useNavigate, Link } from 'react-router-dom'

export default function Register() {
  const [loading, setLoading] = useState(false)
  const [mode, setMode] = useState<'create' | 'join'>('create')
  const navigate = useNavigate()

  const handleSubmit = async (values: any) => {
    setLoading(true)
    
    // TODO: 调用实际 API
    setTimeout(() => {
      message.success('注册成功')
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
      <Card style={{ width: 500 }}>
        <h1 style={{ textAlign: 'center', marginBottom: 24 }}>ApiHub</h1>
        
        <div style={{ textAlign: 'center', marginBottom: 24 }}>
          <Radio.Group value={mode} onChange={(e) => setMode(e.target.value)}>
            <Radio.Button value="create">创建群组</Radio.Button>
            <Radio.Button value="join">加入群组</Radio.Button>
          </Radio.Group>
        </div>

        <Form onFinish={handleSubmit}>
          {mode === 'create' ? (
            <>
              <Form.Item
                name="groupName"
                rules={[{ required: true, message: '请输入群组名称' }]}
              >
                <Input 
                  prefix={<TeamOutlined />} 
                  placeholder="群组名称（如：我的小团队）" 
                  size="large"
                />
              </Form.Item>
            </>
          ) : (
            <>
              <Form.Item
                name="inviteCode"
                rules={[{ required: true, message: '请输入邀请码' }]}
              >
                <Input 
                  prefix={<GiftOutlined />} 
                  placeholder="邀请码" 
                  size="large"
                />
              </Form.Item>
            </>
          )}

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
            name="email"
            rules={[{ required: true, message: '请输入邮箱' }]}
          >
            <Input 
              prefix={<MailOutlined />} 
              placeholder="邮箱" 
              size="large"
            />
          </Form.Item>

          <Form.Item
            name="password"
            rules={[{ required: true, message: '请输入密码' }]}
          >
            <Input.Password
              prefix={<LockOutlined />}
              placeholder="密码（至少 8 位）"
              size="large"
            />
          </Form.Item>

          <Form.Item
            name="confirmPassword"
            dependencies={['password']}
            rules={[
              { required: true, message: '请确认密码' },
              ({ getFieldValue }) => ({
                validator(_, value) {
                  if (!value || getFieldValue('password') === value) {
                    return Promise.resolve()
                  }
                  return Promise.reject(new Error('两次密码不一致'))
                },
              }),
            ]}
          >
            <Input.Password
              prefix={<LockOutlined />}
              placeholder="确认密码"
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
              {mode === 'create' ? '创建群组' : '加入群组'}
            </Button>
          </Form.Item>

          <div style={{ textAlign: 'center' }}>
            已有账号？ <Link to="/login">登录</Link>
          </div>
        </Form>
      </Card>
    </div>
  )
}
