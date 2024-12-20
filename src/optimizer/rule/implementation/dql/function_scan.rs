use crate::errors::DatabaseError;
use crate::optimizer::core::memo::{Expression, GroupExpression};
use crate::optimizer::core::pattern::{Pattern, PatternChildrenPredicate};
use crate::optimizer::core::rule::{ImplementationRule, MatchPattern};
use crate::optimizer::core::statistics_meta::StatisticMetaLoader;
use crate::planner::operator::{Operator, PhysicalOption};
use crate::single_mapping;
use crate::storage::Transaction;
use std::sync::LazyLock;

static FUNCTION_SCAN_PATTERN: LazyLock<Pattern> = LazyLock::new(|| Pattern {
    predicate: |op| matches!(op, Operator::FunctionScan(_)),
    children: PatternChildrenPredicate::None,
});

#[derive(Clone)]
pub struct FunctionScanImplementation;

single_mapping!(
    FunctionScanImplementation,
    FUNCTION_SCAN_PATTERN,
    PhysicalOption::FunctionScan
);
