use crate::domain::model::m_style::Position;
use super::layout_templates::{PageElement, Size};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct LayoutSolver {
    constraints: Vec<Box<dyn Constraint>>,
    solver_config: SolverConfig,
}

pub trait Constraint: Send + Sync {
    fn evaluate(&self, layout: &LayoutSolution) -> f32;
    fn suggest(&self, layout: &LayoutSolution) -> Vec<LayoutAdjustment>;
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverConfig {
    pub max_iterations: u32,
    pub convergence_threshold: f32,
    pub learning_rate: f32,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            convergence_threshold: 0.001,
            learning_rate: 0.1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSolution {
    pub elements: Vec<ElementLayout>,
    pub page_width: u32,
    pub page_height: u32,
    pub margin: Margin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementLayout {
    pub element_id: usize,
    pub position: Position,
    pub size: Size,
    pub element_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margin {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: 50,
            right: 50,
            bottom: 50,
            left: 50,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutAdjustment {
    pub element_id: usize,
    pub position_delta: PositionDelta,
    pub size_delta: SizeDelta,
    pub priority: f32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutResult {
    pub solution: LayoutSolution,
    pub score: f32,
    pub iterations: u32,
    pub conflicts_resolved: u32,
    pub constraint_scores: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub element_ids: Vec<usize>,
    pub conflict_type: ConflictType,
    pub severity: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    Overlap,
    OutOfBounds,
    AlignmentViolation,
    SpacingViolation,
    BalanceViolation,
}

impl LayoutSolver {
    pub fn new() -> Self {
        let mut solver = Self {
            constraints: Vec::new(),
            solver_config: SolverConfig::default(),
        };
        solver.add_default_constraints();
        solver
    }

    pub fn with_config(config: SolverConfig) -> Self {
        let mut solver = Self {
            constraints: Vec::new(),
            solver_config: config,
        };
        solver.add_default_constraints();
        solver
    }

    fn add_default_constraints(&mut self) {
        self.constraints.push(Box::new(AlignmentConstraint));
        self.constraints.push(Box::new(SpacingConstraint));
        self.constraints.push(Box::new(BalanceConstraint));
        self.constraints.push(Box::new(HierarchyConstraint));
        self.constraints.push(Box::new(ReadabilityConstraint));
        self.constraints.push(Box::new(BoundsConstraint));
        self.constraints.push(Box::new(OverlapConstraint));
    }

    pub fn solve(&self, initial_layout: &mut LayoutSolution) -> LayoutResult {
        let mut current_layout = initial_layout.clone();
        let mut iterations = 0;
        let mut conflicts_resolved = 0;
        let mut prev_score = 0.0;

        for iteration in 0..self.solver_config.max_iterations {
            iterations = iteration + 1;

            let conflicts = self.detect_conflicts(&current_layout);
            
            if conflicts.is_empty() {
                break;
            }

            let optimized = self.optimize(&mut current_layout, &conflicts);
            if optimized {
                conflicts_resolved += 1;
            }

            let current_score = self.calculate_total_score(&current_layout);
            
            if (current_score - prev_score).abs() < self.solver_config.convergence_threshold {
                break;
            }
            
            prev_score = current_score;
        }

        let constraint_scores = self.calculate_constraint_scores(&current_layout);
        let total_score = self.calculate_total_score(&current_layout);

        LayoutResult {
            solution: current_layout,
            score: total_score,
            iterations,
            conflicts_resolved,
            constraint_scores,
        }
    }

    fn detect_conflicts(&self, layout: &LayoutSolution) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        for i in 0..layout.elements.len() {
            for j in (i + 1)..layout.elements.len() {
                let elem1 = &layout.elements[i];
                let elem2 = &layout.elements[j];

                if self.check_overlap(elem1, elem2) {
                    conflicts.push(Conflict {
                        element_ids: vec![elem1.element_id, elem2.element_id],
                        conflict_type: ConflictType::Overlap,
                        severity: 0.8,
                        description: format!("元素 {} 和 {} 重叠", elem1.element_id, elem2.element_id),
                    });
                }
            }

            let elem = &layout.elements[i];
            if self.check_out_of_bounds(elem, layout) {
                conflicts.push(Conflict {
                    element_ids: vec![elem.element_id],
                    conflict_type: ConflictType::OutOfBounds,
                    severity: 0.9,
                    description: format!("元素 {} 超出页面边界", elem.element_id),
                });
            }
        }

        conflicts
    }

    fn check_overlap(&self, elem1: &ElementLayout, elem2: &ElementLayout) -> bool {
        let x1_min = elem1.position.x;
        let x1_max = elem1.position.x + elem1.size.width as i32;
        let y1_min = elem1.position.y;
        let y1_max = elem1.position.y + elem1.size.height as i32;

        let x2_min = elem2.position.x;
        let x2_max = elem2.position.x + elem2.size.width as i32;
        let y2_min = elem2.position.y;
        let y2_max = elem2.position.y + elem2.size.height as i32;

        x1_min < x2_max && x1_max > x2_min && y1_min < y2_max && y1_max > y2_min
    }

    fn check_out_of_bounds(&self, elem: &ElementLayout, layout: &LayoutSolution) -> bool {
        let x = elem.position.x;
        let y = elem.position.y;
        let width = elem.size.width as i32;
        let height = elem.size.height as i32;

        x < layout.margin.left as i32 
            || x + width > (layout.page_width - layout.margin.right) as i32
            || y < layout.margin.top as i32 
            || y + height > (layout.page_height - layout.margin.bottom) as i32
    }

    fn optimize(&self, layout: &mut LayoutSolution, conflicts: &[Conflict]) -> bool {
        if conflicts.is_empty() {
            return false;
        }

        let mut adjustments: HashMap<usize, Vec<LayoutAdjustment>> = HashMap::new();

        for constraint in &self.constraints {
            let suggestions = constraint.suggest(layout);
            for suggestion in suggestions {
                adjustments
                    .entry(suggestion.element_id)
                    .or_insert_with(Vec::new)
                    .push(suggestion);
            }
        }

        let mut optimized = false;
        for (element_id, element_adjustments) in adjustments {
            if let Some(element) = layout.elements.iter_mut().find(|e| e.element_id == element_id) {
                let total_dx: i32 = element_adjustments.iter()
                    .map(|a| a.position_delta.dx)
                    .sum();
                let total_dy: i32 = element_adjustments.iter()
                    .map(|a| a.position_delta.dy)
                    .sum();

                element.position.x += (total_dx as f32 * self.solver_config.learning_rate) as i32;
                element.position.y += (total_dy as f32 * self.solver_config.learning_rate) as i32;

                optimized = true;
            }
        }

        optimized
    }

    fn calculate_total_score(&self, layout: &LayoutSolution) -> f32 {
        let scores: Vec<f32> = self.constraints.iter()
            .map(|c| c.evaluate(layout))
            .collect();
        
        if scores.is_empty() {
            0.0
        } else {
            scores.iter().sum::<f32>() / scores.len() as f32
        }
    }

    fn calculate_constraint_scores(&self, layout: &LayoutSolution) -> HashMap<String, f32> {
        let mut scores = HashMap::new();
        for constraint in &self.constraints {
            scores.insert(constraint.name().to_string(), constraint.evaluate(layout));
        }
        scores
    }

    pub fn add_constraint(&mut self, constraint: Box<dyn Constraint>) {
        self.constraints.push(constraint);
    }
}

pub struct AlignmentConstraint;

impl Constraint for AlignmentConstraint {
    fn evaluate(&self, layout: &LayoutSolution) -> f32 {
        if layout.elements.len() < 2 {
            return 1.0;
        }

        let mut score = 0.0;
        let mut count = 0;

        let center_x = layout.page_width as i32 / 2;

        for element in &layout.elements {
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

    fn suggest(&self, layout: &LayoutSolution) -> Vec<LayoutAdjustment> {
        let mut adjustments = Vec::new();
        let center_x = layout.page_width as i32 / 2;

        for element in &layout.elements {
            let element_center_x = element.position.x + element.size.width as i32 / 2;
            let deviation = center_x - element_center_x;
            
            if deviation.abs() > 10 {
                adjustments.push(LayoutAdjustment {
                    element_id: element.element_id,
                    position_delta: PositionDelta { dx: deviation / 2, dy: 0 },
                    size_delta: SizeDelta { dw: 0, dh: 0 },
                    priority: 0.7,
                });
            }
        }

        adjustments
    }

    fn name(&self) -> &str {
        "alignment"
    }
}

pub struct SpacingConstraint;

impl Constraint for SpacingConstraint {
    fn evaluate(&self, layout: &LayoutSolution) -> f32 {
        if layout.elements.len() < 2 {
            return 1.0;
        }

        let mut score = 0.0;
        let mut count = 0;
        let min_spacing = 20u32;

        for i in 0..layout.elements.len() {
            for j in (i + 1)..layout.elements.len() {
                let spacing = self.calculate_spacing(&layout.elements[i], &layout.elements[j]);
                
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

    fn suggest(&self, layout: &LayoutSolution) -> Vec<LayoutAdjustment> {
        let mut adjustments = Vec::new();
        let min_spacing = 20i32;

        for i in 0..layout.elements.len() {
            for j in (i + 1)..layout.elements.len() {
                let spacing = self.calculate_spacing(&layout.elements[i], &layout.elements[j]);
                
                if spacing < min_spacing as u32 {
                    let delta = (min_spacing - spacing as i32) / 2;
                    
                    adjustments.push(LayoutAdjustment {
                        element_id: layout.elements[j].element_id,
                        position_delta: PositionDelta { dx: delta, dy: 0 },
                        size_delta: SizeDelta { dw: 0, dh: 0 },
                        priority: 0.8,
                    });
                }
            }
        }

        adjustments
    }

    fn name(&self) -> &str {
        "spacing"
    }
}

impl SpacingConstraint {
    fn calculate_spacing(&self, elem1: &ElementLayout, elem2: &ElementLayout) -> u32 {
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
}

pub struct BalanceConstraint;

impl Constraint for BalanceConstraint {
    fn evaluate(&self, layout: &LayoutSolution) -> f32 {
        if layout.elements.is_empty() {
            return 1.0;
        }

        let center_x = layout.page_width as f32 / 2.0;
        let center_y = layout.page_height as f32 / 2.0;

        let mut left_weight = 0.0;
        let mut right_weight = 0.0;
        let mut top_weight = 0.0;
        let mut bottom_weight = 0.0;

        for element in &layout.elements {
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

    fn suggest(&self, layout: &LayoutSolution) -> Vec<LayoutAdjustment> {
        let mut adjustments = Vec::new();
        let center_x = layout.page_width as f32 / 2.0;

        let mut left_weight = 0.0;
        let mut right_weight = 0.0;

        for element in &layout.elements {
            let elem_center_x = element.position.x as f32 + element.size.width as f32 / 2.0;
            let area = element.size.width as f32 * element.size.height as f32;

            if elem_center_x < center_x {
                left_weight += area;
            } else {
                right_weight += area;
            }
        }

        if (left_weight - right_weight).abs() > 10000.0 {
            let direction = if left_weight > right_weight { 1 } else { -1 };
            
            for element in &layout.elements {
                adjustments.push(LayoutAdjustment {
                    element_id: element.element_id,
                    position_delta: PositionDelta { dx: direction * 10, dy: 0 },
                    size_delta: SizeDelta { dw: 0, dh: 0 },
                    priority: 0.5,
                });
            }
        }

        adjustments
    }

    fn name(&self) -> &str {
        "balance"
    }
}

pub struct HierarchyConstraint;

impl Constraint for HierarchyConstraint {
    fn evaluate(&self, layout: &LayoutSolution) -> f32 {
        if layout.elements.len() < 2 {
            return 1.0;
        }

        let mut score = 0.0;
        let mut count = 0;

        let sorted_elements: Vec<_> = layout.elements.iter()
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

    fn suggest(&self, _layout: &LayoutSolution) -> Vec<LayoutAdjustment> {
        Vec::new()
    }

    fn name(&self) -> &str {
        "hierarchy"
    }
}

pub struct ReadabilityConstraint;

impl Constraint for ReadabilityConstraint {
    fn evaluate(&self, layout: &LayoutSolution) -> f32 {
        let mut score = 0.0;
        let mut count = 0;

        for element in &layout.elements {
            if element.element_type == "text" {
                let min_width = 200u32;
                let max_width = 800u32;
                let min_height = 30u32;

                if element.size.width >= min_width && element.size.width <= max_width {
                    score += 0.5;
                }
                
                if element.size.height >= min_height {
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

    fn suggest(&self, layout: &LayoutSolution) -> Vec<LayoutAdjustment> {
        let mut adjustments = Vec::new();

        for element in &layout.elements {
            if element.element_type == "text" {
                let max_width: u32 = 800;
                
                if element.size.width > max_width {
                    adjustments.push(LayoutAdjustment {
                        element_id: element.element_id,
                        position_delta: PositionDelta { dx: 0, dy: 0 },
                        size_delta: SizeDelta { 
                            dw: -((element.size.width - max_width) as i32), 
                            dh: 0 
                        },
                        priority: 0.6,
                    });
                }
            }
        }

        adjustments
    }

    fn name(&self) -> &str {
        "readability"
    }
}

pub struct BoundsConstraint;

impl Constraint for BoundsConstraint {
    fn evaluate(&self, layout: &LayoutSolution) -> f32 {
        if layout.elements.is_empty() {
            return 1.0;
        }

        let mut score = 0.0;
        let mut count = 0;

        for element in &layout.elements {
            let in_bounds = element.position.x >= layout.margin.left as i32
                && element.position.y >= layout.margin.top as i32
                && element.position.x + element.size.width as i32 <= (layout.page_width - layout.margin.right) as i32
                && element.position.y + element.size.height as i32 <= (layout.page_height - layout.margin.bottom) as i32;

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

    fn suggest(&self, layout: &LayoutSolution) -> Vec<LayoutAdjustment> {
        let mut adjustments = Vec::new();

        for element in &layout.elements {
            let mut dx = 0i32;
            let mut dy = 0i32;

            if element.position.x < layout.margin.left as i32 {
                dx = layout.margin.left as i32 - element.position.x;
            }
            
            if element.position.y < layout.margin.top as i32 {
                dy = layout.margin.top as i32 - element.position.y;
            }

            let right_bound = (layout.page_width - layout.margin.right) as i32;
            if element.position.x + element.size.width as i32 > right_bound {
                dx = right_bound - element.size.width as i32 - element.position.x;
            }

            let bottom_bound = (layout.page_height - layout.margin.bottom) as i32;
            if element.position.y + element.size.height as i32 > bottom_bound {
                dy = bottom_bound - element.size.height as i32 - element.position.y;
            }

            if dx != 0 || dy != 0 {
                adjustments.push(LayoutAdjustment {
                    element_id: element.element_id,
                    position_delta: PositionDelta { dx, dy },
                    size_delta: SizeDelta { dw: 0, dh: 0 },
                    priority: 1.0,
                });
            }
        }

        adjustments
    }

    fn name(&self) -> &str {
        "bounds"
    }
}

pub struct OverlapConstraint;

impl Constraint for OverlapConstraint {
    fn evaluate(&self, layout: &LayoutSolution) -> f32 {
        if layout.elements.len() < 2 {
            return 1.0;
        }

        let mut overlap_count = 0;
        let total_pairs = layout.elements.len() * (layout.elements.len() - 1) / 2;

        for i in 0..layout.elements.len() {
            for j in (i + 1)..layout.elements.len() {
                if self.check_overlap(&layout.elements[i], &layout.elements[j]) {
                    overlap_count += 1;
                }
            }
        }

        if total_pairs > 0 {
            1.0 - overlap_count as f32 / total_pairs as f32
        } else {
            1.0
        }
    }

    fn suggest(&self, layout: &LayoutSolution) -> Vec<LayoutAdjustment> {
        let mut adjustments = Vec::new();

        for i in 0..layout.elements.len() {
            for j in (i + 1)..layout.elements.len() {
                if self.check_overlap(&layout.elements[i], &layout.elements[j]) {
                    adjustments.push(LayoutAdjustment {
                        element_id: layout.elements[j].element_id,
                        position_delta: PositionDelta { dx: 50, dy: 0 },
                        size_delta: SizeDelta { dw: 0, dh: 0 },
                        priority: 0.9,
                    });
                }
            }
        }

        adjustments
    }

    fn name(&self) -> &str {
        "overlap"
    }
}

impl OverlapConstraint {
    fn check_overlap(&self, elem1: &ElementLayout, elem2: &ElementLayout) -> bool {
        let x1_min = elem1.position.x;
        let x1_max = elem1.position.x + elem1.size.width as i32;
        let y1_min = elem1.position.y;
        let y1_max = elem1.position.y + elem1.size.height as i32;

        let x2_min = elem2.position.x;
        let x2_max = elem2.position.x + elem2.size.width as i32;
        let y2_min = elem2.position.y;
        let y2_max = elem2.position.y + elem2.size.height as i32;

        x1_min < x2_max && x1_max > x2_min && y1_min < y2_max && y1_max > y2_min
    }
}

impl Default for LayoutSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutSolution {
    pub fn from_elements(elements: Vec<PageElement>, page_width: u32, page_height: u32) -> Self {
        let element_layouts: Vec<ElementLayout> = elements
            .into_iter()
            .enumerate()
            .map(|(idx, elem)| ElementLayout {
                element_id: idx,
                position: elem.position,
                size: elem.size,
                element_type: elem.element_type,
            })
            .collect();

        Self {
            elements: element_layouts,
            page_width,
            page_height,
            margin: Margin::default(),
        }
    }

    pub fn to_page_elements(&self) -> Vec<PageElement> {
        self.elements
            .iter()
            .enumerate()
            .map(|(idx, elem)| PageElement {
                element_type: elem.element_type.clone(),
                content: super::layout_templates::ElementContent::Text {
                    text: String::new(),
                    font_size: None,
                    font_weight: None,
                },
                position: elem.position.clone(),
                size: elem.size.clone(),
                style: None,
                z_index: idx as i32,
            })
            .collect()
    }
}
