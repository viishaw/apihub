//! 积分系统模块

/// 积分计算器
pub struct CreditsCalculator {
    /// 每 $1 对应的积分
    pub credits_per_dollar: f64,
    
    /// 各模型的定价（每 1K tokens）
    pub pricing: std::collections::HashMap<String, ModelPricing>,
}

/// 模型定价
#[derive(Debug, Clone)]
pub struct ModelPricing {
    pub input_per_1k: f64,
    pub output_per_1k: f64,
}

impl CreditsCalculator {
    pub fn new() -> Self {
        let mut pricing = std::collections::HashMap::new();
        
        // OpenAI 定价
        pricing.insert("gpt-4-turbo".to_string(), ModelPricing {
            input_per_1k: 0.01,
            output_per_1k: 0.03,
        });
        pricing.insert("gpt-3.5-turbo".to_string(), ModelPricing {
            input_per_1k: 0.0005,
            output_per_1k: 0.0015,
        });
        
        // Anthropic 定价
        pricing.insert("claude-3-opus".to_string(), ModelPricing {
            input_per_1k: 0.015,
            output_per_1k: 0.075,
        });
        pricing.insert("claude-3-sonnet".to_string(), ModelPricing {
            input_per_1k: 0.003,
            output_per_1k: 0.015,
        });
        
        Self {
            credits_per_dollar: 10.0,
            pricing,
        }
    }
    
    /// 计算消耗的积分
    pub fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64 {
        let pricing = self.pricing.get(model);
        
        match pricing {
            Some(p) => {
                let input_cost = (input_tokens as f64 / 1000.0) * p.input_per_1k;
                let output_cost = (output_tokens as f64 / 1000.0) * p.output_per_1k;
                let total_cost = input_cost + output_cost;
                total_cost * self.credits_per_dollar
            }
            None => {
                // 默认定价
                let default_rate = 0.002; // $0.002/1K tokens
                let total_tokens = (input_tokens + output_tokens) as f64 / 1000.0;
                total_tokens * default_rate * self.credits_per_dollar
            }
        }
    }
    
    /// 计算贡献获得的积分
    pub fn calculate_contribution(&self, monthly_quota: f64) -> f64 {
        monthly_quota * self.credits_per_dollar
    }
    
    /// 计算优先级（基于积分）
    pub fn calculate_priority(&self, credits: f64) -> f64 {
        // 使用对数函数，避免高积分用户过度优势
        (credits + 1.0).ln()
    }
}

impl Default for CreditsCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_cost() {
        let calc = CreditsCalculator::new();
        
        let cost = calc.calculate_cost("gpt-4-turbo", 1000, 500);
        // (1K * $0.01 + 0.5K * $0.03) * 10 = $0.025 * 10 = 0.25 credits
        assert!((cost - 0.25).abs() < 0.001);
    }
    
    #[test]
    fn test_priority() {
        let calc = CreditsCalculator::new();
        
        let p1 = calc.calculate_priority(0.0);
        let p2 = calc.calculate_priority(100.0);
        let p3 = calc.calculate_priority(1000.0);
        
        assert!(p2 > p1);
        assert!(p3 > p2);
        
        // 对数增长，不是线性
        assert!(p3 < p2 * 3.0);
    }
}
