import { Row, Col, Card, Statistic } from 'antd'
import {
  CreditCardOutlined,
  KeyOutlined,
  TeamOutlined,
  ThunderboltOutlined,
} from '@ant-design/icons'
import ReactECharts from 'echarts-for-react'

export default function Dashboard() {
  const usageOption = {
    title: { text: '近 7 天使用量' },
    tooltip: { trigger: 'axis' },
    xAxis: { 
      type: 'category', 
      data: ['周一', '周二', '周三', '周四', '周五', '周六', '周日'] 
    },
    yAxis: { type: 'value' },
    series: [{
      data: [120, 200, 150, 80, 70, 110, 130],
      type: 'line',
      smooth: true,
    }],
  }

  const modelOption = {
    title: { text: '模型使用分布' },
    tooltip: { trigger: 'item' },
    series: [{
      type: 'pie',
      radius: '50%',
      data: [
        { value: 1048, name: 'GPT-4' },
        { value: 735, name: 'GPT-3.5' },
        { value: 580, name: 'Claude 3' },
      ],
    }],
  }

  return (
    <div>
      <Row gutter={16}>
        <Col span={6}>
          <Card>
            <Statistic
              title="我的积分"
              value={1250.5}
              prefix={<CreditCardOutlined />}
              suffix="分"
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="贡献的 Key"
              value={5}
              prefix={<KeyOutlined />}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="群组成员"
              value={12}
              prefix={<TeamOutlined />}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="总请求数"
              value={1234}
              prefix={<ThunderboltOutlined />}
            />
          </Card>
        </Col>
      </Row>

      <Row gutter={16} style={{ marginTop: 24 }}>
        <Col span={16}>
          <Card>
            <ReactECharts option={usageOption} />
          </Card>
        </Col>
        <Col span={8}>
          <Card>
            <ReactECharts option={modelOption} />
          </Card>
        </Col>
      </Row>
    </div>
  )
}
