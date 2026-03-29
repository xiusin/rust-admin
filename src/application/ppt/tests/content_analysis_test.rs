#[cfg(test)]
mod tests {
    use crate::application::ppt::text_processor::TextProcessor;
    use crate::application::ppt::industry_classifier::IndustryClassifier;

    #[test]
    fn test_text_cleaning() {
        let processor = TextProcessor::new();
        let dirty_text = "  这是  一段   多余空格的  文本  \n\n  ";
        let clean_text = processor.clean_text(dirty_text);
        
        assert!(!clean_text.starts_with(' '));
        assert!(!clean_text.ends_with(' '));
        assert!(!clean_text.contains("  "));
    }

    #[test]
    fn test_keyword_extraction() {
        let processor = TextProcessor::new();
        let text = "人工智能是计算机科学的一个分支，它企图了解智能的实质，并生产出一种新的能以人类智能相似的方式做出反应的智能机器。人工智能研究的主要目标包括推理、知识、规划、学习、自然语言处理、感知和操作物体的能力。";
        
        let keywords = processor.extract_keywords_tfidf(text, 5);
        
        assert!(!keywords.is_empty());
        assert!(keywords.len() <= 5);
        
        for (_, score) in &keywords {
            assert!(*score >= 0.0 && *score <= 1.0);
        }
    }

    #[test]
    fn test_industry_classification_tech() {
        let classifier = IndustryClassifier::new();
        let text = "我们是一家专注于人工智能和机器学习技术的科技公司，主要提供云计算和大数据解决方案。";
        
        let result = classifier.classify(text, &[]);
        
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.industry, "科技互联网");
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_industry_classification_finance() {
        let classifier = IndustryClassifier::new();
        let text = "我们提供专业的金融投资理财服务，包括股票、基金、保险等金融产品。";
        
        let result = classifier.classify(text, &[]);
        
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.industry, "金融财经");
    }

    #[test]
    fn test_industry_classification_education() {
        let classifier = IndustryClassifier::new();
        let text = "我们是一所教育培训机构，提供在线课程和学习资源。";
        
        let result = classifier.classify(text, &[]);
        
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.industry, "教育培训");
    }

    #[test]
    fn test_entity_extraction() {
        let processor = TextProcessor::new();
        let text = "张三在北京的阿里巴巴公司工作，月薪20000元。";
        
        let entities = processor.extract_entities(text);
        
        assert!(!entities.is_empty());
    }
}
