use crate::domain::model::m_style::{Position, StyleParams};
use super::layout_templates::{
    PageTemplate, PageType, PageContent, PageElement, Size,
    TitlePageTemplate, TocPageTemplate, ContentPageTemplate, ChartPageTemplate, EndPageTemplate,
};
use super::layout_solver::{LayoutSolver, LayoutSolution, LayoutResult};
use std::collections::HashMap;
use std::sync::Arc;

pub struct LayoutEngine {
    solver: LayoutSolver,
    templates: HashMap<PageType, Arc<dyn PageTemplate>>,
    page_width: u32,
    page_height: u32,
}

impl LayoutEngine {
    pub fn new() -> Self {
        let mut templates: HashMap<PageType, Arc<dyn PageTemplate>> = HashMap::new();
        
        templates.insert(PageType::Title, Arc::new(TitlePageTemplate::new()));
        templates.insert(PageType::Toc, Arc::new(TocPageTemplate::new()));
        templates.insert(PageType::Content, Arc::new(ContentPageTemplate::new()));
        templates.insert(PageType::Chart, Arc::new(ChartPageTemplate::new()));
        templates.insert(PageType::End, Arc::new(EndPageTemplate::new()));

        Self {
            solver: LayoutSolver::new(),
            templates,
            page_width: 1920,
            page_height: 1080,
        }
    }

    pub fn with_page_size(mut self, width: u32, height: u32) -> Self {
        self.page_width = width;
        self.page_height = height;
        self
    }

    pub async fn layout_page(
        &self,
        content: &PageContent,
        style: &StyleParams,
        page_type: PageType,
    ) -> Result<LayoutResult, Box<dyn std::error::Error>> {
        let template = self.templates.get(&page_type)
            .ok_or_else(|| format!("未找到页面类型 {:?} 的模板", page_type))?;

        let initial_layout = template.get_layout(style);

        let mut elements = template.apply_content(&initial_layout, content);

        self.apply_style_to_elements(&mut elements, style);

        let mut solution = LayoutSolution::from_elements(elements, self.page_width, self.page_height);

        let result = self.solver.solve(&mut solution);

        Ok(result)
    }

    pub async fn layout_multiple_pages(
        &self,
        pages: Vec<(PageContent, PageType)>,
        style: &StyleParams,
    ) -> Result<Vec<LayoutResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        for (content, page_type) in pages {
            let result = self.layout_page(&content, style, page_type).await?;
            results.push(result);
        }

        Ok(results)
    }

    pub fn calculate_element_position(
        &self,
        element: &PageElement,
        container: &LayoutRegion,
        constraints: &[LayoutConstraint],
    ) -> Position {
        let mut x = container.position.x;
        let mut y = container.position.y;

        for constraint in constraints {
            match constraint {
                LayoutConstraint::Center => {
                    x = container.position.x + (container.size.width as i32 - element.size.width as i32) / 2;
                    y = container.position.y + (container.size.height as i32 - element.size.height as i32) / 2;
                }
                LayoutConstraint::AlignLeft(margin) => {
                    x = container.position.x + *margin as i32;
                }
                LayoutConstraint::AlignRight(margin) => {
                    x = container.position.x + container.size.width as i32 - element.size.width as i32 - *margin as i32;
                }
                LayoutConstraint::AlignTop(margin) => {
                    y = container.position.y + *margin as i32;
                }
                LayoutConstraint::AlignBottom(margin) => {
                    y = container.position.y + container.size.height as i32 - element.size.height as i32 - *margin as i32;
                }
                LayoutConstraint::Offset(dx, dy) => {
                    x += *dx;
                    y += *dy;
                }
                _ => {}
            }
        }

        Position { x, y }
    }

    pub fn calculate_element_size(
        &self,
        element: &PageElement,
        container: &LayoutRegion,
        constraints: &[LayoutConstraint],
    ) -> Size {
        let mut width = element.size.width;
        let mut height = element.size.height;

        for constraint in constraints {
            match constraint {
                LayoutConstraint::FillWidth(ratio) => {
                    width = (container.size.width as f32 * ratio) as u32;
                }
                LayoutConstraint::FillHeight(ratio) => {
                    height = (container.size.height as f32 * ratio) as u32;
                }
                LayoutConstraint::MaxWidth(max_w) => {
                    width = width.min(*max_w);
                }
                LayoutConstraint::MaxHeight(max_h) => {
                    height = height.min(*max_h);
                }
                LayoutConstraint::AspectRatio(ratio) => {
                    height = (width as f32 / ratio) as u32;
                }
                _ => {}
            }
        }

        Size::new(width, height)
    }

    pub fn optimize_visual_balance(&self, solution: &mut LayoutSolution) {
        let center_x = self.page_width as f32 / 2.0;
        let center_y = self.page_height as f32 / 2.0;

        let mut left_weight = 0.0;
        let mut right_weight = 0.0;
        let mut top_weight = 0.0;
        let mut bottom_weight = 0.0;

        for element in &solution.elements {
            let elem_center_x = element.position.x as f32 + element.size.width as f32 / 2.0;
            let elem_center_y = element.position.y as f32 + element.size.height as f32 / 2.0;
            let area = element.size.width as f32 * element.size.height as f32;

            if elem_center_x < center_x {
                left_weight += area;
            } else {
                right_weight += area;
            }

            if elem_center_y < center_y {
                top_weight += area;
            } else {
                bottom_weight += area;
            }
        }

        let horizontal_imbalance = (left_weight - right_weight).abs();
        let vertical_imbalance = (top_weight - bottom_weight).abs();

        if horizontal_imbalance > 10000.0 {
            let shift = if left_weight > right_weight { 20 } else { -20 };
            for element in &mut solution.elements {
                element.position.x += shift;
            }
        }

        if vertical_imbalance > 10000.0 {
            let shift = if top_weight > bottom_weight { 20 } else { -20 };
            for element in &mut solution.elements {
                element.position.y += shift;
            }
        }
    }

    pub fn calculate_layout_score(&self, solution: &LayoutSolution) -> f32 {
        let mut total_score = 0.0;
        let mut weight_sum = 0.0;

        let alignment_score = self.evaluate_alignment(solution);
        total_score += alignment_score * 0.2;
        weight_sum += 0.2;

        let balance_score = self.evaluate_balance(solution);
        total_score += balance_score * 0.2;
        weight_sum += 0.2;

        let spacing_score = self.evaluate_spacing(solution);
        total_score += spacing_score * 0.2;
        weight_sum += 0.2;

        let bounds_score = self.evaluate_bounds(solution);
        total_score += bounds_score * 0.2;
        weight_sum += 0.2;

        let hierarchy_score = self.evaluate_hierarchy(solution);
        total_score += hierarchy_score * 0.2;
        weight_sum += 0.2;

        if weight_sum > 0.0 {
            total_score / weight_sum
        } else {
            0.0
        }
    }

    fn evaluate_alignment(&self, solution: &LayoutSolution) -> f32 {
        if solution.elements.len() < 2 {
            return 1.0;
        }

        let center_x = self.page_width as i32 / 2;
        let mut score = 0.0;
        let mut count = 0;

        for element in &solution.elements {
            let element_center_x = element.position.x + element.size.width as i32 / 2;
            let deviation = (element_center_x - center_x).abs();
            
            if deviation < 10 {
                score += 1.0;
            } else if deviation < 30 {
                score += 0.8;
            } else if deviation < 50 {
                score += 0.5;
            }
            count += 1;
        }

        if count > 0 {
            score / count as f32
        } else {
            1.0
        }
    }

    fn evaluate_balance(&self, solution: &LayoutSolution) -> f32 {
        if solution.elements.is_empty() {
            return 1.0;
        }

        let center_x = self.page_width as f32 / 2.0;
        let center_y = self.page_height as f32 / 2.0;

        let mut left_weight = 0.0;
        let mut right_weight = 0.0;
        let mut top_weight = 0.0;
        let mut bottom_weight = 0.0;

        for element in &solution.elements {
            let elem_center_x = element.position.x as f32 + element.size.width as f32 / 2.0;
            let elem_center_y = element.position.y as f32 + element.size.height as f32 / 2.0;
            let area = element.size.width as f32 * element.size.height as f32;

            if elem_center_x < center_x {
                left_weight += area;
            } else {
                right_weight += area;
            }

            if elem_center_y < center_y {
                top_weight += area;
            } else {
                bottom_weight += area;
            }
        }

        let horizontal_balance = if left_weight + right_weight > 0.0 {
            1.0 - (left_weight - right_weight).abs() / (left_weight + right_weight)
        } else {
            1.0
        };

        let vertical_balance = if top_weight + bottom_weight > 0.0 {
            1.0 - (top_weight - bottom_weight).abs() / (top_weight + bottom_weight)
        } else {
            1.0
        };

        (horizontal_balance + vertical_balance) / 2.0
    }

    fn evaluate_spacing(&self, solution: &LayoutSolution) -> f32 {
        if solution.elements.len() < 2 {
            return 1.0;
        }

        let mut score = 0.0;
        let mut count = 0;
        let min_spacing = 20u32;

        for i in 0..solution.elements.len() {
            for j in (i + 1)..solution.elements.len() {
                let spacing = self.calculate_element_spacing(&solution.elements[i], &solution.elements[j]);
                
                if spacing >= min_spacing {
                    score += 1.0;
                } else if spacing >= min_spacing / 2 {
                    score += 0.5;
                }
                count += 1;
            }
        }

        if count > 0 {
            score / count as f32
        } else {
            1.0
        }
    }

    fn evaluate_bounds(&self, solution: &LayoutSolution) -> f32 {
        if solution.elements.is_empty() {
            return 1.0;
        }

        let margin = 50u32;
        let mut score = 0.0;
        let mut count = 0;

        for element in &solution.elements {
            let in_bounds = element.position.x >= margin as i32
                && element.position.y >= margin as i32
                && element.position.x + element.size.width as i32 <= (self.page_width - margin) as i32
                && element.position.y + element.size.height as i32 <= (self.page_height - margin) as i32;

            if in_bounds {
                score += 1.0;
            }
            count += 1;
        }

        if count > 0 {
            score / count as f32
        } else {
            1.0
        }
    }

    fn evaluate_hierarchy(&self, solution: &LayoutSolution) -> f32 {
        if solution.elements.len() < 2 {
            return 1.0;
        }

        let mut score = 0.0;
        let mut count = 0;

        let sorted_elements: Vec<_> = solution.elements.iter()
            .filter(|e| e.element_type == "text")
            .collect();

        for i in 0..sorted_elements.len().saturating_sub(1) {
            let current = sorted_elements[i];
            let next = sorted_elements[i + 1];

            if current.size.height > next.size.height {
                score += 1.0;
            } else if current.size.height == next.size.height {
                score += 0.7;
            }
            count += 1;
        }

        if count > 0 {
            score / count as f32
        } else {
            1.0
        }
    }

    fn calculate_element_spacing(&self, elem1: &super::layout_solver::ElementLayout, elem2: &super::layout_solver::ElementLayout) -> u32 {
        let x1 = elem1.position.x;
        let x1_max = elem1.position.x + elem1.size.width as i32;
        let y1 = elem1.position.y;
        let y1_max = elem1.position.y + elem1.size.height as i32;

        let x2 = elem2.position.x;
        let x2_max = elem2.position.x + elem2.size.width as i32;
        let y2 = elem2.position.y;
        let y2_max = elem2.position.y + elem2.size.height as i32;

        let horizontal_spacing = if x1_max <= x2 {
            x2 - x1_max
        } else if x2_max <= x1 {
            x1 - x2_max
        } else {
            0
        };

        let vertical_spacing = if y1_max <= y2 {
            y2 - y1_max
        } else if y2_max <= y1 {
            y1 - y2_max
        } else {
            0
        };

        if horizontal_spacing > 0 && vertical_spacing > 0 {
            horizontal_spacing.min(vertical_spacing) as u32
        } else if horizontal_spacing > 0 {
            horizontal_spacing as u32
        } else if vertical_spacing > 0 {
            vertical_spacing as u32
        } else {
            0
        }
    }

    fn apply_style_to_elements(&self, elements: &mut [PageElement], style: &StyleParams) {
        for element in elements.iter_mut() {
            if element.style.is_none() {
                element.style = Some(super::layout_templates::ElementStyle {
                    color: Some(style.color_scheme.text_color.clone()),
                    background_color: None,
                    border_radius: None,
                    opacity: None,
                    shadow: None,
                });
            }
        }
    }

    pub fn suggest_layouts(
        &self,
        content: &PageContent,
        style: &StyleParams,
        page_type: PageType,
        count: u32,
    ) -> Vec<LayoutSuggestion> {
        let mut suggestions = Vec::new();

        let template = match self.templates.get(&page_type) {
            Some(t) => t,
            None => return suggestions,
        };

        let base_layout = template.get_layout(style);
        let base_elements = template.apply_content(&base_layout, content);

        for i in 0..count {
            let mut elements = base_elements.clone();
            
            let variation = self.apply_layout_variation(&mut elements, i);

            let solution = LayoutSolution::from_elements(elements, self.page_width, self.page_height);
            let score = self.calculate_layout_score(&solution);

            suggestions.push(LayoutSuggestion {
                layout_name: format!("{}_variation_{}", base_layout.name, i),
                elements: solution.to_page_elements(),
                score,
                variation_type: variation,
            });
        }

        suggestions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        suggestions
    }

    fn apply_layout_variation(&self, elements: &mut [PageElement], variation_index: u32) -> String {
        match variation_index % 5 {
            0 => {
                "default".to_string()
            }
            1 => {
                for element in elements.iter_mut() {
                    element.position.x += 50;
                }
                "shifted_right".to_string()
            }
            2 => {
                for element in elements.iter_mut() {
                    element.position.x -= 50;
                }
                "shifted_left".to_string()
            }
            3 => {
                for element in elements.iter_mut() {
                    element.position.y += 30;
                }
                "shifted_down".to_string()
            }
            4 => {
                for element in elements.iter_mut() {
                    element.position.y -= 30;
                }
                "shifted_up".to_string()
            }
            _ => "default".to_string(),
        }
    }

    pub fn adjust_layout(
        &self,
        solution: &LayoutSolution,
        adjustments: &[LayoutAdjustment],
    ) -> LayoutSolution {
        let mut new_solution = solution.clone();

        for adjustment in adjustments {
            if let Some(element) = new_solution.elements.iter_mut()
                .find(|e| e.element_id == adjustment.element_id) 
            {
                element.position.x += adjustment.position_delta.dx;
                element.position.y += adjustment.position_delta.dy;
                element.size.width = (element.size.width as i32 + adjustment.size_delta.dw).max(0) as u32;
                element.size.height = (element.size.height as i32 + adjustment.size_delta.dh).max(0) as u32;
            }
        }

        new_solution
    }

    pub fn get_template(&self, page_type: PageType) -> Option<Arc<dyn PageTemplate>> {
        self.templates.get(&page_type).cloned()
    }

    pub fn get_available_page_types(&self) -> Vec<PageType> {
        self.templates.keys().cloned().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutRegion {
    pub position: Position,
    pub size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutConstraint {
    Center,
    AlignLeft(u32),
    AlignRight(u32),
    AlignTop(u32),
    AlignBottom(u32),
    Offset(i32, i32),
    FillWidth(f32),
    FillHeight(f32),
    MaxWidth(u32),
    MaxHeight(u32),
    AspectRatio(f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSuggestion {
    pub layout_name: String,
    pub elements: Vec<PageElement>,
    pub score: f32,
    pub variation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutAdjustment {
    pub element_id: usize,
    pub position_delta: PositionDelta,
    pub size_delta: SizeDelta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionDelta {
    pub dx: i32,
    pub dy: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeDelta {
    pub dw: i32,
    pub dh: i32,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

use serde::{Deserialize, Serialize};
