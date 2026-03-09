import { useState } from 'react'
import { Table, Button, Modal, Form, Input, Select, Space, Tag, Switch, message } from 'antd'
import { PlusOutlined, DeleteOutlined, EditOutlined, PauseOutlined, CaretRightOutlined } from '@ant-design/icons'

const { TextArea } = Input

interface Key {
  id: string
  provider: string
  name: string
  contributor: string
  monthlyQuota: number | null
  usedQuota: number
  weight: number
  isActive: boolean
  lastUsedAt: string | null
  createdAt: string
}

const mockKeys: Key[] = [
  {
    id: '1',
    provider: 'openai',
    name: 'GPT-4 Key',
    contributor: 'Vincent',
    monthlyQuota: 100,
    usedQuota: 45.5,
    weight: 1,
    isActive: true,
    lastUsedAt: '2024-03-10 10:30:00',
    createdAt: '2024-03-01',
  },
  {
    id: '2',
    provider: 'anthropic',
    name: 'Claude Key',
    contributor: 'Alice',
    monthlyQuota: 50,
    usedQuota: 12.3,
    weight: 1,
    isActive: true,
    lastUsedAt: '2024-03-09 15:20:00',
    createdAt: '2024-03-02',
  },
]

export default function Keys() {
  const [keys, setKeys] = useState<Key[]>(mockKeys)
  const [modalOpen, setModalOpen] = useState(false)
  const [form] = Form.useForm()

  const handleCreate = () => {
    form.resetFields()
    setModalOpen(true)
  }

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields()
      console.log('Create key:', values)
      message.success('Key 添加成功')
      setModalOpen(false)
    } catch (error) {
      // Validation failed
    }
  }

  const handleToggle = (id: string) => {
    setKeys(keys.map(k => k.id === id ? { ...k, isActive: !k.isActive } : k))
    message.success('状态已切换')
  }

  const handleDelete = (id: string) => {
    Modal.confirm({
      title: '确认删除',
      content: '删除后无法恢复，确定要删除这个 Key 吗？',
      onOk: () => {
        setKeys(keys.filter(k => k.id !== id))
        message.success('Key 已删除')
      },
    })
  }

  const columns = [
    {
      title: '提供商',
      dataIndex: 'provider',
      key: 'provider',
      render: (provider: string) => (
        <Tag color={provider === 'openai' ? 'green' : 'blue'}>
          {provider.toUpperCase()}
        </Tag>
      ),
    },
    { title: '名称', dataIndex: 'name', key: 'name' },
    { title: '贡献者', dataIndex: 'contributor', key: 'contributor' },
    {
      title: '配额',
      key: 'quota',
      render: (_: any, record: Key) => (
        <span>
          {record.usedQuota.toFixed(2)} / {record.monthlyQuota || '∞'}
        </span>
      ),
    },
    { title: '权重', dataIndex: 'weight', key: 'weight' },
    {
      title: '状态',
      dataIndex: 'isActive',
      key: 'isActive',
      render: (isActive: boolean) => (
        <Tag color={isActive ? 'success' : 'default'}>
          {isActive ? '启用' : '暂停'}
        </Tag>
      ),
    },
    { title: '最后使用', dataIndex: 'lastUsedAt', key: 'lastUsedAt' },
    {
      title: '操作',
      key: 'actions',
      render: (_: any, record: Key) => (
        <Space>
          <Button
            size="small"
            icon={record.isActive ? <PauseOutlined /> : <CaretRightOutlined />}
            onClick={() => handleToggle(record.id)}
          >
            {record.isActive ? '暂停' : '启用'}
          </Button>
          <Button
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDelete(record.id)}
          >
            删除
          </Button>
        </Space>
      ),
    },
  ]

  return (
    <div>
      <div style={{ marginBottom: 16 }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={handleCreate}>
          添加 Key
        </Button>
      </div>

      <Table dataSource={keys} columns={columns} rowKey="id" />

      <Modal
        title="添加 API Key"
        open={modalOpen}
        onOk={handleSubmit}
        onCancel={() => setModalOpen(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="provider"
            label="提供商"
            rules={[{ required: true, message: '请选择提供商' }]}
          >
            <Select placeholder="选择提供商">
              <Select.Option value="openai">OpenAI</Select.Option>
              <Select.Option value="anthropic">Anthropic</Select.Option>
            </Select>
          </Form.Item>

          <Form.Item
            name="apiKey"
            label="API Key"
            rules={[{ required: true, message: '请输入 API Key' }]}
          >
            <Input.Password placeholder="sk-..." />
          </Form.Item>

          <Form.Item name="name" label="名称（可选）">
            <Input placeholder="例如：GPT-4 Key" />
          </Form.Item>

          <Form.Item name="baseUrl" label="Base URL（可选）">
            <Input placeholder="https://api.openai.com/v1" />
          </Form.Item>

          <Form.Item name="monthlyQuota" label="每月配额（美元）">
            <Input type="number" placeholder="100" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  )
}
