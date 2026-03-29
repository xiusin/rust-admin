#[cfg(test)]
mod tests {
    use crate::application::ppt::layout_solver::{LayoutSolver, SolverConfig};
    use crate::application::ppt::layout_templates::*;

    #[test]
    fn test_constraint_evaluation() {
        let solver = LayoutSolver::new();
        
        let layout = Layout {
            elements: vec![
                PageElement {
                    id: "1".to_string(),
                    element_type: ElementType::Text,
                    position: ElementPosition { x: 100.0, y: 100.0 },
                    size: ElementSize { width: 200.0, height: 50.0 },
                    z_index: 1,
                },
            ],
            page_size: PageSize {
                width: 1920.0,
                height: 1080.0,
            },
        };
        
        let score = solver.evaluate_layout(&layout);
        
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_conflict_detection() {
        let solver = LayoutSolver::new();
        
        let layout = Layout {
            elements: vec![
                PageElement {
                    id: "1".to_string(),
                    element_type: ElementType::Text,
                    position: ElementPosition { x: 100.0, y: 100.0 },
                    size: ElementSize { width: 200.0, height: 50.0 },
                    z_index: 1,
                },
                PageElement {
                    id: "2".to_string(),
                    element_type: ElementType::Text,
                    position: ElementPosition { x: 150.0, y: 120.0 },
                    size: ElementSize { width: 200.0, height: 50.0 },
                    z_index: 2,
                },
            ],
            page_size: PageSize {
                width: 1920.0,
                height: 1080.0,
            },
        };
        
        let conflicts = solver.detect_conflicts(&layout);
        
        assert!(!conflicts.is_empty());
    }

    #[test]
    fn test_layout_optimization() {
        let solver = LayoutSolver::new();
        
        let mut layout = Layout {
            elements: vec![
                PageElement {
                    id: "1".to_string(),
                    element_type: ElementType::Text,
                    position: ElementPosition { x: 50.0, y: 50.0 },
                    size: ElementSize { width: 300.0, height: 100.0 },
                    z_index: 1,
                },
            ],
            page_size: PageSize {
                width: 1920.0,
                height: 1080.0,
            },
        };
        
        let initial_score = solver.evaluate_layout(&layout);
        solver.optimize(&mut layout);
        let final_score = solver.evaluate_layout(&layout);
        
        assert!(final_score >= initial_score);
    }

    #[test]
    fn test_alignment_constraint() {
        let solver = LayoutSolver::new();
        
        let layout = Layout {
            elements: vec![
                PageElement {
                    id: "1".to_string(),
                    element_type: ElementType::Text,
                    position: ElementPosition { x: 960.0, y: 100.0 },
                    size: ElementSize { width: 200.0, height: 50.0 },
                    z_index: 1,
                },
            ],
            page_size: PageSize {
                width: 1920.0,
                height: 1080.0,
            },
        };
        
        let alignment_score = solver.check_alignment(&layout);
        
        assert!(alignment_score > 0.8);
    }

    #[test]
    fn test_balance_constraint() {
        let solver = LayoutSolver::new();
        
        let balanced_layout = Layout {
            elements: vec![
                PageElement {
                    id: "1".to_string(),
                    element_type: ElementType::Text,
                    position: ElementPosition { x: 100.0, y: 540.0 },
                    size: ElementSize { width: 200.0, height: 50.0 },
                    z_index: 1,
                },
                PageElement {
                    id: "2".to_string(),
                    element_type: ElementType::Text,
                    position: ElementPosition { x: 1620.0, y: 540.0 },
                    size: ElementSize { width: 200.0, height: 50.0 },
                    z_index: 2,
                },
            ],
            page_size: PageSize {
                width: 1920.0,
                height: 1080.0,
            },
        };
        
        let balance_score = solver.check_balance(&balanced_layout);
        
        assert!(balance_score > 0.7);
    }
}
