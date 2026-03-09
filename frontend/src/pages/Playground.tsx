import { useState } from 'react'
import { Input, Select, Button, Card, Typography, Space, message } from 'antd'

const { TextArea } = Input
const { Text, Paragraph } = Typography

export default function Playground() {
  const [model, setModel] = useState('gpt-4-turbo')
  const [messages, setMessages] = useState<Array<{ role: string; content: string }>>([])
  const [input, setInput] = useState('')
  const [loading, setLoading] = useState(false)

  const handleSend = async () => {
    if (!input.trim()) return

    const newMessages = [...messages, { role: 'user', content: input }]
    setMessages(newMessages)
    setInput('')
    setLoading(true)

    // TODO: 调用实际 API
    setTimeout(() => {
      setMessages([...newMessages, { 
        role: 'assistant', 
        content: '这是一个模拟响应。实际使用时需要连接后端 API。' 
      }])
      setLoading(false)
    }, 1000)
  }

  return (
    <div style={{ height: 'calc(100vh - 150px)', display: 'flex', flexDirection: 'column' }}>
      <div style={{ marginBottom: 16 }}>
        <Space>
          <Select
            value={model}
            onChange={setModel}
            style={{ width: 200 }}
            options={[
              { value: 'gpt-4-turbo', label: 'GPT-4 Turbo' },
              { value: 'gpt-3.5-turbo', label: 'GPT-3.5 Turbo' },
              { value: 'claude-3-opus', label: 'Claude 3 Opus' },
              { value: 'claude-3-sonnet', label: 'Claude 3 Sonnet' },
            ]}
          />
        </Space>
      </div>

      <Card 
        style={{ flex: 1, overflow: 'auto', marginBottom: 16 }}
        bodyStyle={{ padding: 16 }}
      >
        {messages.length === 0 ? (
          <div style={{ textAlign: 'center', color: '#999', padding: 40 }}>
            开始对话吧
          </div>
        ) : (
          messages.map((msg, idx) => (
            <div 
              key={idx} 
              style={{ 
                marginBottom: 16,
                textAlign: msg.role === 'user' ? 'right' : 'left',
              }}
            >
              <div
                style={{
                  display: 'inline-block',
                  maxWidth: '70%',
                  padding: '12px 16px',
                  borderRadius: 8,
                  backgroundColor: msg.role === 'user' ? '#1890ff' : '#f0f0f0',
                  color: msg.role === 'user' ? '#fff' : '#000',
                }}
              >
                <Text style={{ color: 'inherit', whiteSpace: 'pre-wrap' }}>
                  {msg.content}
                </Text>
              </div>
            </div>
          ))
        )}
      </Card>

      <div style={{ display: 'flex', gap: 8 }}>
        <TextArea
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="输入消息..."
          autoSize={{ minRows: 2, maxRows: 4 }}
          onPressEnter={(e) => {
            if (!e.shiftKey) {
              e.preventDefault()
              handleSend()
            }
          }}
        />
        <Button 
          type="primary" 
          onClick={handleSend}
          loading={loading}
          style={{ height: 'auto' }}
        >
          发送
        </Button>
      </div>
    </div>
  )
}
