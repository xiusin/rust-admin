use std::collections::{HashMap, HashSet};
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::common::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedText {
    pub original: String,
    pub cleaned: String,
    pub tokens: Vec<String>,
    pub keywords: Vec<KeywordScore>,
    pub entities: Vec<NamedEntity>,
    pub language: String,
    pub word_count: usize,
    pub char_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordScore {
    pub keyword: String,
    pub score: f64,
    pub tf: f64,
    pub idf: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedEntity {
    pub entity_type: EntityType,
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Date,
    Money,
    Percentage,
    Number,
    Email,
    Phone,
    Url,
    Product,
    Event,
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Person => write!(f, "人名"),
            EntityType::Organization => write!(f, "组织机构"),
            EntityType::Location => write!(f, "地点"),
            EntityType::Date => write!(f, "日期"),
            EntityType::Money => write!(f, "金额"),
            EntityType::Percentage => write!(f, "百分比"),
            EntityType::Number => write!(f, "数字"),
            EntityType::Email => write!(f, "邮箱"),
            EntityType::Phone => write!(f, "电话"),
            EntityType::Url => write!(f, "网址"),
            EntityType::Product => write!(f, "产品"),
            EntityType::Event => write!(f, "事件"),
        }
    }
}

pub struct TextProcessor {
    stop_words_cn: HashSet<String>,
    stop_words_en: HashSet<String>,
    email_regex: Regex,
    phone_regex: Regex,
    url_regex: Regex,
    money_regex: Regex,
    percentage_regex: Regex,
    date_regex: Regex,
}

impl TextProcessor {
    pub fn new() -> Self {
        Self {
            stop_words_cn: Self::load_chinese_stop_words(),
            stop_words_en: Self::load_english_stop_words(),
            email_regex: Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap(),
            phone_regex: Regex::new(r"(?:\+?86[-\s]?)?1[3-9]\d{9}|(?:\+?\d{1,3}[-\s]?)?\d{3,4}[-\s]?\d{7,8}").unwrap(),
            url_regex: Regex::new(r"https?://[^\s]+|www\.[^\s]+").unwrap(),
            money_regex: Regex::new(r"[¥$€£]\s*[\d,]+(?:\.\d{1,2})?|[\d,]+(?:\.\d{1,2})?\s*(?:元|美元|欧元|英镑|万元|亿元)").unwrap(),
            percentage_regex: Regex::new(r"[\d.]+%\s*|[\d.]+\s*%").unwrap(),
            date_regex: Regex::new(r"\d{4}[-/年]\d{1,2}[-/月]\d{1,2}日?|\d{4}[-/]\d{1,2}[-/]\d{1,2}|\d{1,2}月\d{1,2}日").unwrap(),
        }
    }

    fn load_chinese_stop_words() -> HashSet<String> {
        let words = [
            "的", "了", "和", "是", "就", "都", "而", "及", "与", "着",
            "或", "一个", "没有", "我们", "你们", "他们", "它们", "这个", "那个", "这些",
            "那些", "自己", "什么", "怎么", "这个", "那个", "因为", "所以", "但是", "如果",
            "虽然", "可以", "可能", "应该", "需要", "能够", "已经", "正在", "将要", "曾经",
            "一直", "还是", "或者", "而且", "不过", "只是", "就是", "这样", "那样", "怎样",
            "非常", "很", "太", "更", "最", "比较", "相当", "特别", "尤其", "甚至",
            "关于", "对于", "根据", "按照", "通过", "经过", "为了", "由于", "鉴于", "基于",
            "在", "从", "向", "到", "把", "被", "让", "给", "跟", "比",
            "按", "以", "为", "等", "等等", "之", "此", "其", "这", "那",
            "有", "无", "没", "不", "也", "又", "再", "还", "才", "只",
            "一", "二", "三", "四", "五", "六", "七", "八", "九", "十",
            "第一", "第二", "第三", "首先", "其次", "然后", "最后", "总之", "综上所述", "因此",
        ];
        words.iter().map(|s| s.to_string()).collect()
    }

    fn load_english_stop_words() -> HashSet<String> {
        let words = [
            "a", "an", "the", "and", "or", "but", "in", "on", "at", "to",
            "for", "of", "with", "by", "from", "as", "is", "was", "are", "were",
            "been", "be", "have", "has", "had", "do", "does", "did", "will", "would",
            "could", "should", "may", "might", "must", "shall", "can", "need", "dare", "ought",
            "used", "it", "its", "this", "that", "these", "those", "i", "you", "he",
            "she", "we", "they", "what", "which", "who", "whom", "whose", "where", "when",
            "why", "how", "all", "each", "every", "both", "few", "more", "most", "other",
            "some", "such", "no", "nor", "not", "only", "own", "same", "so", "than",
            "too", "very", "just", "also", "now", "here", "there", "then", "once", "if",
            "because", "until", "while", "about", "against", "between", "into", "through", "during", "before",
            "after", "above", "below", "up", "down", "out", "off", "over", "under", "again",
            "further", "any", "am", "being", "get", "got", "getting", "go", "goes", "going",
            "went", "gone", "make", "makes", "making", "made", "take", "takes", "taking", "took",
        ];
        words.iter().map(|s| s.to_lowercase()).collect()
    }

    pub fn process(&self, text: &str) -> Result<ProcessedText> {
        let cleaned = self.clean_text(text);
        let language = self.detect_language(&cleaned);
        let tokens = self.tokenize(&cleaned, &language);
        let entities = self.extract_entities(&cleaned);
        let keywords = self.extract_keywords_tfidf(&tokens, &language);
        
        let word_count = tokens.len();
        let char_count = cleaned.chars().count();

        Ok(ProcessedText {
            original: text.to_string(),
            cleaned,
            tokens,
            keywords,
            entities,
            language,
            word_count,
            char_count,
        })
    }

    pub fn clean_text(&self, text: &str) -> String {
        let mut cleaned = text.to_string();
        
        cleaned = cleaned.replace('\r', " ");
        cleaned = cleaned.replace('\n', " ");
        cleaned = cleaned.replace('\t', " ");
        
        let multi_space = Regex::new(r"\s+").unwrap();
        cleaned = multi_space.replace_all(&cleaned, " ").to_string();
        
        let invisible_chars = Regex::new(r"[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]").unwrap();
        cleaned = invisible_chars.replace_all(&cleaned, "").to_string();
        
        cleaned = cleaned.trim().to_string();
        
        cleaned
    }

    pub fn tokenize(&self, text: &str, language: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        
        if language == "zh" || language == "mixed" {
            tokens.extend(self.tokenize_chinese(text));
        }
        
        if language == "en" || language == "mixed" {
            tokens.extend(self.tokenize_english(text));
        }
        
        if language != "zh" && language != "en" && language != "mixed" {
            tokens.extend(self.tokenize_chinese(text));
            tokens.extend(self.tokenize_english(text));
        }
        
        let mut unique_tokens = Vec::new();
        let mut seen = HashSet::new();
        for token in tokens {
            let lower = token.to_lowercase();
            if !seen.contains(&lower) && !self.is_stop_word(&token, language) {
                seen.insert(lower);
                unique_tokens.push(token);
            }
        }
        
        unique_tokens
    }

    fn tokenize_chinese(&self, text: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut prev_char_type = CharType::Other;
        
        for ch in text.chars() {
            let char_type = self.get_char_type(ch);
            
            match char_type {
                CharType::Chinese => {
                    if !current_token.is_empty() && prev_char_type != CharType::Chinese {
                        if self.is_valid_token(&current_token) {
                            tokens.push(current_token.clone());
                        }
                        current_token.clear();
                    }
                    
                    if current_token.is_empty() {
                        current_token.push(ch);
                    } else {
                        if self.is_valid_token(&current_token) {
                            tokens.push(current_token.clone());
                        }
                        current_token = ch.to_string();
                        
                        if current_token.len() >= 2 {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    }
                }
                CharType::English | CharType::Digit => {
                    if prev_char_type == CharType::Chinese && !current_token.is_empty() {
                        if self.is_valid_token(&current_token) {
                            tokens.push(current_token.clone());
                        }
                        current_token.clear();
                    }
                    current_token.push(ch);
                }
                CharType::Punctuation | CharType::Space => {
                    if !current_token.is_empty() {
                        if self.is_valid_token(&current_token) {
                            tokens.push(current_token.clone());
                        }
                        current_token.clear();
                    }
                }
                CharType::Other => {
                    current_token.push(ch);
                }
            }
            
            prev_char_type = char_type;
        }
        
        if !current_token.is_empty() && self.is_valid_token(&current_token) {
            tokens.push(current_token);
        }
        
        self.extract_chinese_ngrams(text, &mut tokens);
        
        tokens
    }

    fn extract_chinese_ngrams(&self, text: &str, tokens: &mut Vec<String>) {
        let chinese_chars: Vec<char> = text.chars()
            .filter(|c| self.is_chinese_char(*c))
            .collect();
        
        for window_size in 2..=4 {
            if chinese_chars.len() >= window_size {
                for window in chinese_chars.windows(window_size) {
                    let ngram: String = window.iter().collect();
                    if !self.is_stop_word(&ngram, "zh") {
                        tokens.push(ngram);
                    }
                }
            }
        }
    }

    fn tokenize_english(&self, text: &str) -> Vec<String> {
        let word_regex = Regex::new(r"[a-zA-Z]+(?:'[a-zA-Z]+)?").unwrap();
        word_regex
            .find_iter(text)
            .filter_map(|m| {
                let word = m.as_str();
                if word.len() >= 2 && !self.is_stop_word(word, "en") {
                    Some(word.to_lowercase())
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_char_type(&self, ch: char) -> CharType {
        if self.is_chinese_char(ch) {
            CharType::Chinese
        } else if ch.is_ascii_alphabetic() {
            CharType::English
        } else if ch.is_ascii_digit() {
            CharType::Digit
        } else if ch.is_ascii_punctuation() || self.is_chinese_punctuation(ch) {
            CharType::Punctuation
        } else if ch.is_whitespace() {
            CharType::Space
        } else {
            CharType::Other
        }
    }

    fn is_chinese_char(&self, ch: char) -> bool {
        matches!(ch,
            '\u{4E00}'..='\u{9FFF}' |
            '\u{3400}'..='\u{4DBF}' |
            '\u{20000}'..='\u{2A6DF}' |
            '\u{2A700}'..='\u{2B73F}' |
            '\u{2B740}'..='\u{2B81F}' |
            '\u{2B820}'..='\u{2CEAF}' |
            '\u{F900}'..='\u{FAFF}' |
            '\u{2F800}'..='\u{2FA1F}'
        )
    }

    fn is_chinese_punctuation(&self, ch: char) -> bool {
        matches!(ch,
            '，' | '。' | '、' | '；' | '：' | '？' | '！' |
            '"' | '\'' | '（' | '）' | '【' | '】' |
            '《' | '》' | '「' | '」' | '『' | '』' | '〈' | '〉'
        )
    }

    fn is_valid_token(&self, token: &str) -> bool {
        let trimmed = token.trim();
        !trimmed.is_empty() && trimmed.len() >= 1
    }

    fn is_stop_word(&self, word: &str, language: &str) -> bool {
        let lower = word.to_lowercase();
        match language {
            "zh" => self.stop_words_cn.contains(&lower),
            "en" => self.stop_words_en.contains(&lower),
            "mixed" => self.stop_words_cn.contains(&lower) || self.stop_words_en.contains(&lower),
            _ => self.stop_words_cn.contains(&lower) || self.stop_words_en.contains(&lower),
        }
    }

    pub fn detect_language(&self, text: &str) -> String {
        let mut chinese_count = 0;
        let mut english_count = 0;
        
        for ch in text.chars() {
            if self.is_chinese_char(ch) {
                chinese_count += 1;
            } else if ch.is_ascii_alphabetic() {
                english_count += 1;
            }
        }
        
        let total = chinese_count + english_count;
        if total == 0 {
            return "unknown".to_string();
        }
        
        let chinese_ratio = chinese_count as f64 / total as f64;
        
        if chinese_ratio > 0.7 {
            "zh".to_string()
        } else if chinese_ratio < 0.3 {
            "en".to_string()
        } else {
            "mixed".to_string()
        }
    }

    pub fn extract_keywords_tfidf(&self, tokens: &[String], language: &str) -> Vec<KeywordScore> {
        let mut term_freq: HashMap<String, usize> = HashMap::new();
        let total_terms = tokens.len();
        
        for token in tokens {
            *term_freq.entry(token.to_lowercase()).or_insert(0) += 1;
        }
        
        let mut keywords: Vec<KeywordScore> = term_freq
            .iter()
            .map(|(term, &freq)| {
                let tf = freq as f64 / total_terms as f64;
                let idf = self.calculate_idf(term, language);
                let score = tf * idf;
                
                KeywordScore {
                    keyword: term.clone(),
                    score,
                    tf,
                    idf,
                }
            })
            .collect();
        
        keywords.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        keywords.truncate(20);
        
        keywords
    }

    fn calculate_idf(&self, term: &str, language: &str) -> f64 {
        let doc_freq = self.estimate_doc_frequency(term, language);
        let total_docs = 1000000.0;
        
        if doc_freq > 0.0 {
            (total_docs / doc_freq).ln() + 1.0
        } else {
            10.0
        }
    }

    fn estimate_doc_frequency(&self, term: &str, language: &str) -> f64 {
        let common_terms: Vec<(&str, f64)> = match language {
            "zh" => vec![
                ("公司", 500000.0), ("产品", 400000.0), ("服务", 350000.0),
                ("系统", 300000.0), ("技术", 280000.0), ("发展", 260000.0),
                ("管理", 250000.0), ("业务", 240000.0), ("数据", 230000.0),
                ("平台", 220000.0), ("用户", 210000.0), ("市场", 200000.0),
                ("企业", 190000.0), ("项目", 180000.0), ("解决方案", 170000.0),
            ],
            "en" => vec![
                ("company", 500000.0), ("product", 400000.0), ("service", 350000.0),
                ("system", 300000.0), ("technology", 280000.0), ("development", 260000.0),
                ("management", 250000.0), ("business", 240000.0), ("data", 230000.0),
                ("platform", 220000.0), ("user", 210000.0), ("market", 200000.0),
            ],
            _ => vec![
                ("公司", 500000.0), ("company", 500000.0), ("产品", 400000.0),
            ],
        };
        
        for (common_term, freq) in common_terms.iter() {
            if term == *common_term {
                return *freq;
            }
        }
        
        let term_len = term.chars().count();
        match term_len {
            1 => 800000.0,
            2 => 500000.0,
            3 => 200000.0,
            4 => 100000.0,
            5 => 50000.0,
            _ => 10000.0,
        }
    }

    pub fn extract_entities(&self, text: &str) -> Vec<NamedEntity> {
        let mut entities = Vec::new();
        
        for cap in self.email_regex.captures_iter(text) {
            if let Some(m) = cap.get(0) {
                entities.push(NamedEntity {
                    entity_type: EntityType::Email,
                    text: m.as_str().to_string(),
                    start: m.start(),
                    end: m.end(),
                    confidence: 0.95,
                });
            }
        }
        
        for cap in self.url_regex.captures_iter(text) {
            if let Some(m) = cap.get(0) {
                entities.push(NamedEntity {
                    entity_type: EntityType::Url,
                    text: m.as_str().to_string(),
                    start: m.start(),
                    end: m.end(),
                    confidence: 0.95,
                });
            }
        }
        
        for cap in self.phone_regex.captures_iter(text) {
            if let Some(m) = cap.get(0) {
                entities.push(NamedEntity {
                    entity_type: EntityType::Phone,
                    text: m.as_str().to_string(),
                    start: m.start(),
                    end: m.end(),
                    confidence: 0.90,
                });
            }
        }
        
        for cap in self.money_regex.captures_iter(text) {
            if let Some(m) = cap.get(0) {
                entities.push(NamedEntity {
                    entity_type: EntityType::Money,
                    text: m.as_str().to_string(),
                    start: m.start(),
                    end: m.end(),
                    confidence: 0.90,
                });
            }
        }
        
        for cap in self.percentage_regex.captures_iter(text) {
            if let Some(m) = cap.get(0) {
                entities.push(NamedEntity {
                    entity_type: EntityType::Percentage,
                    text: m.as_str().to_string(),
                    start: m.start(),
                    end: m.end(),
                    confidence: 0.95,
                });
            }
        }
        
        for cap in self.date_regex.captures_iter(text) {
            if let Some(m) = cap.get(0) {
                entities.push(NamedEntity {
                    entity_type: EntityType::Date,
                    text: m.as_str().to_string(),
                    start: m.start(),
                    end: m.end(),
                    confidence: 0.85,
                });
            }
        }
        
        entities.extend(self.extract_chinese_entities(text));
        
        entities.sort_by_key(|e| e.start);
        entities
    }

    fn extract_chinese_entities(&self, text: &str) -> Vec<NamedEntity> {
        let mut entities = Vec::new();
        
        let org_patterns = [
            (r"([\u{4e00}-\u{9fff}]+(?:公司|集团|企业|银行|医院|学校|大学|研究院|研究所|中心|机构|协会|基金会|电视台|报社|出版社))", EntityType::Organization),
            (r"([\u{4e00}-\u{9fff}]+(?:科技|网络|互联网|软件|信息|电子|通信|智能|数据|云计算|人工智能|AI)(?:公司|集团)?)", EntityType::Organization),
        ];
        
        let location_patterns = [
            (r"([\u{4e00}-\u{9fff}]+(?:省|市|区|县|镇|乡|村|街道|路|街|道|广场|大厦|中心))", EntityType::Location),
            (r"((?:北京|上海|广州|深圳|杭州|南京|武汉|成都|西安|重庆|天津|苏州|郑州|长沙|东莞|青岛|沈阳|宁波|昆明)(?:市)?)", EntityType::Location),
        ];
        
        let person_patterns = [
            (r"([\u{4e00}-\u{9fff}]{2,4})(?:先生|女士|教授|博士|经理|总监|总裁|董事长|CEO|CTO|CFO)", EntityType::Person),
        ];
        
        for (pattern, entity_type) in org_patterns.iter() {
            if let Ok(re) = Regex::new(pattern) {
                for cap in re.captures_iter(text) {
                    if let Some(m) = cap.get(1) {
                        entities.push(NamedEntity {
                            entity_type: entity_type.clone(),
                            text: m.as_str().to_string(),
                            start: m.start(),
                            end: m.end(),
                            confidence: 0.80,
                        });
                    }
                }
            }
        }
        
        for (pattern, entity_type) in location_patterns.iter() {
            if let Ok(re) = Regex::new(pattern) {
                for cap in re.captures_iter(text) {
                    if let Some(m) = cap.get(1) {
                        entities.push(NamedEntity {
                            entity_type: entity_type.clone(),
                            text: m.as_str().to_string(),
                            start: m.start(),
                            end: m.end(),
                            confidence: 0.85,
                        });
                    }
                }
            }
        }
        
        for (pattern, entity_type) in person_patterns.iter() {
            if let Ok(re) = Regex::new(pattern) {
                for cap in re.captures_iter(text) {
                    if let Some(m) = cap.get(1) {
                        entities.push(NamedEntity {
                            entity_type: entity_type.clone(),
                            text: m.as_str().to_string(),
                            start: m.start(),
                            end: m.end(),
                            confidence: 0.75,
                        });
                    }
                }
            }
        }
        
        entities
    }

    pub fn extract_summary(&self, text: &str, max_sentences: usize) -> String {
        let sentences: Vec<&str> = text
            .split(|c| c == '。' || c == '！' || c == '？' || c == '.' || c == '!' || c == '?')
            .filter(|s| !s.trim().is_empty())
            .collect();
        
        if sentences.len() <= max_sentences {
            return text.to_string();
        }
        
        let mut sentence_scores: Vec<(usize, f64)> = sentences
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let score = self.calculate_sentence_score(s, i, sentences.len());
                (i, score)
            })
            .collect();
        
        sentence_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sentence_scores.truncate(max_sentences);
        sentence_scores.sort_by_key(|(i, _)| *i);
        
        let summary: String = sentence_scores
            .iter()
            .map(|(i, _)| sentences[*i].trim())
            .collect::<Vec<&str>>()
            .join("。");
        
        if !summary.ends_with('。') && !summary.ends_with('.') {
            format!("{}。", summary)
        } else {
            summary
        }
    }

    fn calculate_sentence_score(&self, sentence: &str, position: usize, total_sentences: usize) -> f64 {
        let mut score = 0.0;
        
        let position_weight = if position < 3 {
            1.5
        } else if position > total_sentences - 3 {
            0.8
        } else {
            1.0
        };
        score += position_weight;
        
        let word_count = sentence.chars().count() as f64;
        let length_score = if word_count > 20.0 && word_count < 100.0 {
            1.2
        } else if word_count < 10.0 {
            0.5
        } else {
            1.0
        };
        score *= length_score;
        
        let keywords = ["重要", "关键", "核心", "主要", "首先", "其次", "最后", "总结", "总之", "综上所述"];
        for keyword in keywords.iter() {
            if sentence.contains(keyword) {
                score += 0.5;
            }
        }
        
        score
    }
}

impl Default for TextProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CharType {
    Chinese,
    English,
    Digit,
    Punctuation,
    Space,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text() {
        let processor = TextProcessor::new();
        let text = "  这是  一个\n测试  文本  ";
        let cleaned = processor.clean_text(text);
        assert_eq!(cleaned, "这是 一个 测试 文本");
    }

    #[test]
    fn test_detect_language() {
        let processor = TextProcessor::new();
        
        assert_eq!(processor.detect_language("这是一个中文测试"), "zh");
        assert_eq!(processor.detect_language("This is an English test"), "en");
        assert_eq!(processor.detect_language("这是中英文mixed测试"), "mixed");
    }

    #[test]
    fn test_extract_entities() {
        let processor = TextProcessor::new();
        let text = "请联系 support@example.com 或拨打 13800138000，公司地址在北京市朝阳区。";
        let entities = processor.extract_entities(text);
        
        assert!(!entities.is_empty());
        assert!(entities.iter().any(|e| e.entity_type == EntityType::Email));
        assert!(entities.iter().any(|e| e.entity_type == EntityType::Phone));
        assert!(entities.iter().any(|e| e.entity_type == EntityType::Location));
    }

    #[test]
    fn test_extract_keywords() {
        let processor = TextProcessor::new();
        let text = "人工智能技术在医疗健康领域有广泛应用，机器学习算法可以帮助医生进行疾病诊断。";
        let processed = processor.process(text).unwrap();
        
        assert!(!processed.keywords.is_empty());
        assert!(processed.keywords.iter().any(|k| k.keyword.contains("人工智能") || k.keyword.contains("医疗")));
    }
}
