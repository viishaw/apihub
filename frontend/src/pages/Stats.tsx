import { Card, Tabs } from 'antd'
import ReactECharts from 'echarts-for-react'

export default function Stats() {
  const usageOption = {
    title: { text: '使用量趋势' },
    tooltip: { trigger: 'axis' },
    legend: { data: ['请求数', 'Token数', '成本'] },
    xAxis: { 
      type: 'category', 
      data: ['周一', '周二', '周三', '周四', '周五', '周六', '周日'] 
    },
    yAxis: { type: 'value' },
    series: [
      { name: '请求数', data: [120, 200, 150, 80, 70, 110, 130], type: 'line' },
      { name: 'Token数', data: [220, 400, 250, 180, 170, 210, 230], type: 'line' },
      { name: '成本', data: [12, 20, 15, 8, 7, 11, 13], type: 'line' },
    ],
  }

  const leaderboardOption = {
    title: { text: '贡献排行榜' },
    tooltip: { trigger: 'axis', axisPointer: { type: 'shadow' } },
    xAxis: { type: 'value' },
    yAxis: { 
      type: 'category', 
      data: ['Alice', 'Bob', 'Charlie', 'David', 'Eve'],
    },
    series: [{
      type: 'bar',
      data: [1500, 1200, 800, 600, 400],
      itemStyle: { color: '#1890ff' },
    }],
  }

  const modelDistributionOption = {
    title: { text: '模型使用分布' },
    tooltip: { trigger: 'item' },
    series: [{
      type: 'pie',
      radius: ['40%', '70%'],
      data: [
        { value: 1048, name: 'GPT-4 Turbo' },
        { value: 735, name: 'GPT-3.5 Turbo' },
        { value: 580, name: 'Claude 3 Opus' },
        { value: 484, name: 'Claude 3 Sonnet' },
        { value: 300, name: 'Claude 3 Haiku' },
      ],
    }],
  }

  const providerDistributionOption = {
    title: { text: '提供商分布' },
    tooltip: { trigger: 'item' },
    series: [{
      type: 'pie',
      radius: '60%',
      data: [
        { value: 1783, name: 'OpenAI', itemStyle: { color: '#10a37f' } },
        { value: 1364, name: 'Anthropic', itemStyle: { color: '#d97706' } },
      ],
    }],
  }

  return (
    <Card>
      <Tabs
        items={[
          {
            key: 'usage',
            label: '使用统计',
            children: (
              <div>
                <ReactECharts option={usageOption} style={{ height: 400 }} />
                <div style={{ display: 'flex', gap: 24, marginTop: 24 }}>
                  <div style={{ flex: 1 }}>
                    <ReactECharts option={modelDistributionOption} style={{ height: 300 }} />
                  </div>
                  <div style={{ flex: 1 }}>
                    <ReactECharts option={providerDistributionOption} style={{ height: 300 }} />
                  </div>
                </div>
              </div>
            ),
          },
          {
            key: 'leaderboard',
            label: '贡献排行',
            children: (
              <ReactECharts option={leaderboardOption} style={{ height: 400 }} />
            ),
          },
        ]}
      />
    </Card>
  )
}
