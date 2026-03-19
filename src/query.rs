use crate::stage::QueryStage;

struct Query {
    stages: Vec<Box<dyn QueryStage>>,
}
