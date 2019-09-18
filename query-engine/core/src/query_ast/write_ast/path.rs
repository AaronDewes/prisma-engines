use connector::filter::{Filter, RecordFinder};
use prisma_models::prelude::*;

pub struct Path {
    segments: Vec<PathSegment>,
}

pub enum PathSegment {
    ToOne(RelationFieldRef),
    ToMany(RelationFieldRef, RecordFinder),
    ToManyFilter(RelationFieldRef, Option<Filter>),
}

impl From<RelationFieldRef> for PathSegment {
    fn from(rf: RelationFieldRef) -> PathSegment {
        PathSegment::ToOne(rf)
    }
}

impl From<(RelationFieldRef, RecordFinder)> for PathSegment {
    fn from(t: (RelationFieldRef, RecordFinder)) -> PathSegment {
        PathSegment::ToMany(t.0, t.1)
    }
}

impl From<(RelationFieldRef, Option<Filter>)> for PathSegment {
    fn from(t: (RelationFieldRef, Option<Filter>)) -> PathSegment {
        PathSegment::ToManyFilter(t.0, t.1)
    }
}

impl Path {
    pub fn append(&mut self, segment: PathSegment) {
        self.segments.push(segment);
    }
}
