use crate::column::Column;
use crate::stage::QueryStage;

struct ColumnFilter<T, U>
where
    T: PartialOrd + PartialEq,
    U: Column,
{
    value: T,
    op: ComparisonOperator,
    column: U,
}

impl<T, U> QueryStage for ColumnFilter<T, U>
where
    T: PartialOrd + PartialEq,
    U: Column,
{
}

enum ComparisonOperator {}
