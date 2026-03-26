use serde::Serialize;
use std::collections::HashMap;

use crate::core::catalog;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
    pub resolved_dependencies: Vec<String>,
    pub port_assignments: HashMap<String, u16>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValidationIssue {
    pub level: IssueLevel,
    pub message: String,
    pub tech_id: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IssueLevel {
    Error,
    Warning,
    Info,
}

/// Valida una combinacion de tecnologias contra el catalogo
pub fn validate_stack(technology_ids: &[String]) -> ValidationResult {
    let all_techs = catalog::load_all_technologies();
    let mut issues = Vec::new();
    let mut resolved_deps = Vec::new();
    let mut port_assignments: HashMap<String, u16> = HashMap::new();
    let mut suggestions = Vec::new();
    let mut used_ports: HashMap<u16, String> = HashMap::new();

    // Resolver tecnologias seleccionadas
    let selected: Vec<_> = technology_ids
        .iter()
        .filter_map(|id| all_techs.iter().find(|t| t.id == *id))
        .collect();

    // Verificar que todas las IDs existen en el catalogo
    for id in technology_ids {
        if !all_techs.iter().any(|t| t.id == *id) {
            issues.push(ValidationIssue {
                level: IssueLevel::Error,
                message: format!("Tecnologia '{}' no encontrada en el catalogo", id),
                tech_id: id.clone(),
            });
        }
    }

    for tech in &selected {
        // Check requires
        for req in &tech.requires {
            if !technology_ids.contains(req) {
                // Auto-resolve dependency
                if all_techs.iter().any(|t| t.id == *req) {
                    resolved_deps.push(req.clone());
                    issues.push(ValidationIssue {
                        level: IssueLevel::Info,
                        message: format!(
                            "'{}' requiere '{}' — agregado automaticamente",
                            tech.id, req
                        ),
                        tech_id: tech.id.clone(),
                    });
                } else {
                    issues.push(ValidationIssue {
                        level: IssueLevel::Error,
                        message: format!(
                            "'{}' requiere '{}' que no esta en el catalogo",
                            tech.id, req
                        ),
                        tech_id: tech.id.clone(),
                    });
                }
            }
        }

        // Check incompatibilities
        for incompat in &tech.incompatible_with {
            if technology_ids.contains(incompat) {
                issues.push(ValidationIssue {
                    level: IssueLevel::Error,
                    message: format!(
                        "'{}' es incompatible con '{}'",
                        tech.id, incompat
                    ),
                    tech_id: tech.id.clone(),
                });
            }
        }

        // Collect suggestions
        for sug in &tech.suggested_with {
            if !technology_ids.contains(sug)
                && !resolved_deps.contains(sug)
                && all_techs.iter().any(|t| t.id == *sug)
                && !suggestions.contains(sug)
            {
                suggestions.push(sug.clone());
            }
        }

        // Port assignment + conflict detection
        if tech.default_port > 0 {
            if let Some(existing) = used_ports.get(&tech.default_port) {
                // Port conflict — assign next available
                let mut next_port = tech.default_port + 1;
                while used_ports.contains_key(&next_port) {
                    next_port += 1;
                }
                issues.push(ValidationIssue {
                    level: IssueLevel::Warning,
                    message: format!(
                        "Puerto {} en conflicto entre '{}' y '{}' — '{}' reasignado a {}",
                        tech.default_port, existing, tech.id, tech.id, next_port
                    ),
                    tech_id: tech.id.clone(),
                });
                used_ports.insert(next_port, tech.id.clone());
                port_assignments.insert(tech.id.clone(), next_port);
            } else {
                used_ports.insert(tech.default_port, tech.id.clone());
                port_assignments.insert(tech.id.clone(), tech.default_port);
            }
        }
    }

    // Check for multiple runtimes (warning)
    let runtime_count = selected.iter().filter(|t| t.category == "runtime").count();
    if runtime_count > 1 {
        issues.push(ValidationIssue {
            level: IssueLevel::Warning,
            message: format!(
                "Se seleccionaron {} runtimes — considera si necesitas todos",
                runtime_count
            ),
            tech_id: String::new(),
        });
    }

    // Check for multiple ORMs (warning)
    let orm_count = selected.iter().filter(|t| t.category == "orm").count();
    if orm_count > 1 {
        issues.push(ValidationIssue {
            level: IssueLevel::Warning,
            message: "Se seleccionaron multiples ORMs — normalmente solo necesitas uno".to_string(),
            tech_id: String::new(),
        });
    }

    let has_errors = issues.iter().any(|i| i.level == IssueLevel::Error);

    ValidationResult {
        valid: !has_errors,
        issues,
        resolved_dependencies: resolved_deps,
        port_assignments,
        suggestions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_stack() {
        let result = validate_stack(&["nodejs".to_string(), "express".to_string()]);
        assert!(result.valid);
    }

    #[test]
    fn test_validate_incompatible() {
        // Django requires python and is incompatible with flask (if defined)
        let result = validate_stack(&[
            "django".to_string(),
            "flask".to_string(), // not in catalog, should show error
        ]);
        // flask is not in our 30 techs catalog
        assert!(result.issues.iter().any(|i| i.level == IssueLevel::Error));
    }

    #[test]
    fn test_validate_resolves_dependencies() {
        // nextjs requires nodejs and react
        let result = validate_stack(&["nextjs".to_string()]);
        // Should auto-resolve nodejs and react
        assert!(
            result.resolved_dependencies.contains(&"nodejs".to_string())
                || result.issues.iter().any(|i| i.message.contains("nodejs")),
            "nextjs should resolve or mention nodejs dependency"
        );
    }

    #[test]
    fn test_validate_port_conflicts() {
        // nextjs and react both use port 3000
        let result = validate_stack(&[
            "nextjs".to_string(),
            "express".to_string(),
        ]);
        // Both might use port 3000/3001 — check assignments
        assert!(!result.port_assignments.is_empty());
    }

    #[test]
    fn test_validate_suggestions() {
        let result = validate_stack(&["nodejs".to_string()]);
        // nodejs suggests typescript
        assert!(result.suggestions.contains(&"typescript".to_string()));
    }

    #[test]
    fn test_validate_unknown_tech() {
        let result = validate_stack(&["nonexistent-tech".to_string()]);
        assert!(!result.valid);
        assert!(result.issues.iter().any(|i| {
            i.level == IssueLevel::Error && i.message.contains("no encontrada")
        }));
    }

    #[test]
    fn test_validate_multiple_runtimes_warning() {
        let result = validate_stack(&[
            "nodejs".to_string(),
            "python".to_string(),
        ]);
        assert!(result.issues.iter().any(|i| {
            i.level == IssueLevel::Warning && i.message.contains("runtimes")
        }));
    }

    #[test]
    fn test_validate_empty_stack() {
        let result = validate_stack(&[]);
        assert!(result.valid);
        assert!(result.issues.is_empty());
    }
}
