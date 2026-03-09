import { useState } from 'react'
import { Card, Tabs, Table, Button, InputNumber, message } from 'antd'

export default function Admin() {
  const [creditsAdjustment, setCreditsAdjustment] = useState<number>(0)

  const usersColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: '用户名', dataIndex: 'username', key: 'username' },
    { title: '邮箱', dataIndex: 'email', key: 'email' },
    { title: '积分', dataIndex: 'credits', key: 'credits' },
    { title: '贡献 Key', dataIndex: 'keyCount', key: 'keyCount' },
    { 
      title: '状态', 
      dataIndex: 'isActive', 
      key: 'isActive',
      render: (v: boolean) => v ? '启用' : '禁用',
    },
    {
      title: '操作',
      key: 'actions',
      render: () => (
        <Button.Group>
          <Button size="small">调整积分</Button>
          <Button size="small" danger>禁用</Button>
        </Button.Group>
      ),
    },
  ]

  const mockUsers = [
    { id: '1', username: 'Vincent', email: 'vincent@example.com', credits: 1250.5, keyCount: 5, isActive: true },
    { id: '2', username: 'Alice', email: 'alice@example.com', credits: 980.0, keyCount: 3, isActive: true },
    { id: '3', username: 'Bob', email: 'bob@example.com', credits: 450.2, keyCount: 1, isActive: true },
  ]

  const keysColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: '提供商', dataIndex: 'provider', key: 'provider' },
    { title: '名称', dataIndex: 'name', key: 'name' },
    { title: '贡献者', dataIndex: 'contributor', key: 'contributor' },
    { title: '使用量', dataIndex: 'usage', key: 'usage' },
    { 
      title: '状态', 
      dataIndex: 'isActive', 
      key: 'isActive',
      render: (v: boolean) => v ? '启用' : '暂停',
    },
    {
      title: '操作',
      key: 'actions',
      render: () => (
        <Button.Group>
          <Button size="small">查看</Button>
          <Button size="small" danger>禁用</Button>
        </Button.Group>
      ),
    },
  ]

  const mockKeys = [
    { id: '1', provider: 'openai', name: 'GPT-4 Key', contributor: 'Vincent', usage: '45.5 / 100', isActive: true },
    { id: '2', provider: 'anthropic', name: 'Claude Key', contributor: 'Alice', usage: '12.3 / 50', isActive: true },
  ]

  return (
    <Card>
      <Tabs
        items={[
          {
            key: 'users',
            label: '用户管理',
            children: (
              <Table
                dataSource={mockUsers}
                columns={usersColumns}
                rowKey="id"
              />
            ),
          },
          {
            key: 'keys',
            label: 'Key 管理',
            children: (
              <Table
                dataSource={mockKeys}
                columns={keysColumns}
                rowKey="id"
              />
            ),
          },
          {
            key: 'settings',
            label: '群组设置',
            children: (
              <div style={{ maxWidth: 600 }}>
                <div style={{ marginBottom: 16 }}>
                  <label>邀请码：</label>
                  <span style={{ fontFamily: 'monospace', marginLeft: 8 }}>abc123xyz</span>
                  <Button style={{ marginLeft: 16 }} size="small">
                    重新生成
                  </Button>
                </div>
                <div style={{ marginBottom: 16 }}>
                  <label>积分汇率（积分/美元）：</label>
                  <InputNumber 
                    value={10} 
                    min={1} 
                    style={{ marginLeft: 8, width: 100 }}
                  />
                </div>
                <Button type="primary">保存设置</Button>
              </div>
            ),
          },
        ]}
      />
    </Card>
  )
}
