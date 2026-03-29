use crate::common::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryResult {
    pub industry: String,
    pub confidence: f32,
    pub alternatives: Vec<IndustryMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryMatch {
    pub industry: String,
    pub confidence: f32,
}

pub struct IndustryClassifier {
    industry_keywords: HashMap<String, Vec<String>>,
}

impl IndustryClassifier {
    pub fn new() -> Self {
        let mut industry_keywords: HashMap<String, Vec<String>> = HashMap::new();
        
        industry_keywords.insert("科技".to_string(), vec![
            "技术".to_string(), "软件".to_string(), "硬件".to_string(), "人工智能".to_string(), "AI".to_string(), "机器学习".to_string(), "云计算".to_string(), "大数据".to_string(),
            "互联网".to_string(), "数字化".to_string(), "智能化".to_string(), "创新".to_string(), "研发".to_string(), "科技".to_string(), "IT".to_string(), "编程".to_string(),
        ]);
        
        industry_keywords.insert("金融".to_string(), vec![
            "银行".to_string(), "投资".to_string(), "基金".to_string(), "股票".to_string(), "证券".to_string(), "保险".to_string(), "理财".to_string(), "金融".to_string(),
            "资产".to_string(), "风险".to_string(), "收益".to_string(), "贷款".to_string(), "信用".to_string(), "资本".to_string(), "融资".to_string(),
        ]);
        
        industry_keywords.insert("医疗".to_string(), vec![
            "医院".to_string(), "医疗".to_string(), "健康".to_string(), "药品".to_string(), "诊断".to_string(), "治疗".to_string(), "患者".to_string(), "医生".to_string(),
            "临床".to_string(), "医学".to_string(), "护理".to_string(), "康复".to_string(), "保健".to_string(), "医药".to_string(), "生物".to_string(),
        ]);
        
        industry_keywords.insert("教育".to_string(), vec![
            "学校".to_string(), "教育".to_string(), "学习".to_string(), "培训".to_string(), "课程".to_string(), "教学".to_string(), "学生".to_string(), "教师".to_string(),
            "课堂".to_string(), "知识".to_string(), "考试".to_string(), "学历".to_string(), "培训".to_string(), "在线教育".to_string(),
        ]);
        
        industry_keywords.insert("零售".to_string(), vec![
            "零售".to_string(), "电商".to_string(), "购物".to_string(), "商品".to_string(), "消费".to_string(), "门店".to_string(), "销售".to_string(), "客户".to_string(),
            "品牌".to_string(), "营销".to_string(), "供应链".to_string(), "库存".to_string(), "物流".to_string(), "超市".to_string(),
        ]);
        
        industry_keywords.insert("制造".to_string(), vec![
            "制造".to_string(), "生产".to_string(), "工厂".to_string(), "工业".to_string(), "设备".to_string(), "流水线".to_string(), "质量".to_string(), "工艺".to_string(),
            "自动化".to_string(), "装配".to_string(), "加工".to_string(), "材料".to_string(), "产品".to_string(), "车间".to_string(),
        ]);
        
        industry_keywords.insert("房地产".to_string(), vec![
            "房地产".to_string(), "房产".to_string(), "住宅".to_string(), "商业地产".to_string(), "楼盘".to_string(), "物业".to_string(), "建筑".to_string(),
            "开发".to_string(), "投资".to_string(), "租赁".to_string(), "房价".to_string(), "土地".to_string(), "项目".to_string(),
        ]);
        
        industry_keywords.insert("能源".to_string(), vec![
            "能源".to_string(), "电力".to_string(), "石油".to_string(), "天然气".to_string(), "新能源".to_string(), "太阳能".to_string(), "风能".to_string(),
            "发电".to_string(), "电网".to_string(), "储能".to_string(), "环保".to_string(), "碳排放".to_string(), "绿色".to_string(),
        ]);

        Self { industry_keywords }
    }

    pub async fn classify(&self, text: &str) -> Result<IndustryResult> {
        let text_lower = text.to_lowercase();
        let mut scores: HashMap<String, f32> = HashMap::new();

        for (industry, keywords) in &self.industry_keywords {
            let mut score = 0.0f32;
            for keyword in keywords {
                let count = text_lower.matches(&keyword.to_lowercase()).count();
                score += count as f32;
            }
            scores.insert(industry.clone(), score);
        }

        let mut sorted: Vec<_> = scores.into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let total_score: f32 = sorted.iter().map(|(_, s)| *s).sum();
        
        let top = sorted.first().map(|(industry, score)| {
            let confidence = if total_score > 0.0 { *score / total_score } else { 0.0 };
            (industry.clone(), confidence)
        }).unwrap_or(("通用".to_string(), 0.0));

        let alternatives: Vec<IndustryMatch> = sorted
            .iter()
            .skip(1)
            .take(3)
            .map(|(industry, score)| {
                let confidence = if total_score > 0.0 { *score / total_score } else { 0.0 };
                IndustryMatch {
                    industry: industry.clone(),
                    confidence,
                }
            })
            .collect();

        Ok(IndustryResult {
            industry: top.0,
            confidence: top.1,
            alternatives,
        })
    }
}
