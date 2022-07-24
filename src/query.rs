use gcloud_sdk::google::firestore::v1::*;
use rsb_derive::Builder;

#[derive(Debug, PartialEq, Clone)]
pub enum FirestoreQueryCollection {
    Single(String),
    Group(Vec<String>),
}

impl ToString for FirestoreQueryCollection {
    fn to_string(&self) -> String {
        match self {
            FirestoreQueryCollection::Single(single) => single.to_string(),
            FirestoreQueryCollection::Group(group) => group.join(","),
        }
    }
}

impl From<&str> for FirestoreQueryCollection {
    fn from(collection_id_str: &str) -> Self {
        FirestoreQueryCollection::Single(collection_id_str.to_string())
    }
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct FirestoreQueryParams {
    pub parent: Option<String>,
    pub collection_id: FirestoreQueryCollection,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub order_by: Option<Vec<FirestoreQueryOrder>>,
    pub filter: Option<FirestoreQueryFilter>,
    pub all_descendants: Option<bool>,
}

impl FirestoreQueryParams {
    pub fn to_structured_query(&self) -> StructuredQuery {
        let query_filter = self.filter.as_ref().map(|f| f.to_structured_query_filter());

        StructuredQuery {
            select: None,
            start_at: None,
            end_at: None,
            limit: self.limit.map(|x| x as i32),
            offset: self.offset.map(|x| x as i32).unwrap_or(0),
            order_by: self
                .order_by
                .as_ref()
                .map(|po| po.iter().map(|fo| fo.to_structured_query_order()).collect())
                .unwrap_or_else(Vec::new),
            from: match self.collection_id {
                FirestoreQueryCollection::Single(ref collection_id) => {
                    vec![structured_query::CollectionSelector {
                        collection_id: collection_id.clone(),
                        all_descendants: self.all_descendants.unwrap_or(false),
                    }]
                }
                FirestoreQueryCollection::Group(ref collection_ids) => collection_ids
                    .iter()
                    .map(|collection_id| structured_query::CollectionSelector {
                        collection_id: collection_id.clone(),
                        all_descendants: self.all_descendants.unwrap_or(false),
                    })
                    .collect(),
            },
            r#where: query_filter,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FirestoreQueryFilter {
    Composite(FirestoreQueryFilterComposite),
    Unary(FirestoreQueryFilterUnary),
    Compare(Option<FirestoreQueryFilterCompare>),
}

impl FirestoreQueryFilter {
    fn to_structured_query_filter(&self) -> structured_query::Filter {
        let filter_type = match self {
            FirestoreQueryFilter::Compare(comp) => comp.as_ref().map(|cmp| {
                structured_query::filter::FilterType::FieldFilter(match cmp {
                    FirestoreQueryFilterCompare::Equal(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::Equal.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                    FirestoreQueryFilterCompare::NotEqual(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::NotEqual.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                    FirestoreQueryFilterCompare::In(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::In.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                    FirestoreQueryFilterCompare::NotIn(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::NotIn.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                    FirestoreQueryFilterCompare::ArrayContains(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::ArrayContains.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                    FirestoreQueryFilterCompare::ArrayContainsAny(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::ArrayContainsAny.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                    FirestoreQueryFilterCompare::LessThan(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::LessThan.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                    FirestoreQueryFilterCompare::LessThanOrEqual(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::LessThanOrEqual.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                    FirestoreQueryFilterCompare::GreaterThan(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::GreaterThan.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                    FirestoreQueryFilterCompare::GreaterThanOrEqual(field_name, fvalue) => {
                        structured_query::FieldFilter {
                            field: Some(structured_query::FieldReference {
                                field_path: field_name.clone(),
                            }),
                            op: structured_query::field_filter::Operator::GreaterThanOrEqual.into(),
                            value: Some(fvalue.value.clone()),
                        }
                    }
                })
            }),
            FirestoreQueryFilter::Composite(composite) => {
                Some(structured_query::filter::FilterType::CompositeFilter(
                    structured_query::CompositeFilter {
                        op: structured_query::composite_filter::Operator::And.into(),
                        filters: composite
                            .for_all_filters
                            .iter()
                            .map(|filter| filter.to_structured_query_filter())
                            .filter(|filter| filter.filter_type.is_some())
                            .collect(),
                    },
                ))
            }
            FirestoreQueryFilter::Unary(unary) => match unary {
                FirestoreQueryFilterUnary::IsNan(field_name) => {
                    Some(structured_query::filter::FilterType::UnaryFilter(
                        structured_query::UnaryFilter {
                            op: structured_query::unary_filter::Operator::IsNan.into(),
                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                structured_query::FieldReference {
                                    field_path: field_name.clone(),
                                },
                            )),
                        },
                    ))
                }
                FirestoreQueryFilterUnary::IsNull(field_name) => {
                    Some(structured_query::filter::FilterType::UnaryFilter(
                        structured_query::UnaryFilter {
                            op: structured_query::unary_filter::Operator::IsNull.into(),
                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                structured_query::FieldReference {
                                    field_path: field_name.clone(),
                                },
                            )),
                        },
                    ))
                }
                FirestoreQueryFilterUnary::IsNotNan(field_name) => {
                    Some(structured_query::filter::FilterType::UnaryFilter(
                        structured_query::UnaryFilter {
                            op: structured_query::unary_filter::Operator::IsNotNan.into(),
                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                structured_query::FieldReference {
                                    field_path: field_name.clone(),
                                },
                            )),
                        },
                    ))
                }
                FirestoreQueryFilterUnary::IsNotNull(field_name) => {
                    Some(structured_query::filter::FilterType::UnaryFilter(
                        structured_query::UnaryFilter {
                            op: structured_query::unary_filter::Operator::IsNotNull.into(),
                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                structured_query::FieldReference {
                                    field_path: field_name.clone(),
                                },
                            )),
                        },
                    ))
                }
            },
        };

        structured_query::Filter { filter_type }
    }
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct FirestoreQueryOrder {
    field_name: String,
    direction: FirestoreQueryDirection,
}

impl FirestoreQueryOrder {
    pub fn to_structured_query_order(&self) -> structured_query::Order {
        structured_query::Order {
            field: Some(structured_query::FieldReference {
                field_path: self.field_name.clone(),
            }),
            direction: (match self.direction {
                FirestoreQueryDirection::Ascending => structured_query::Direction::Ascending.into(),
                FirestoreQueryDirection::Descending => {
                    structured_query::Direction::Descending.into()
                }
            }),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FirestoreQueryDirection {
    Ascending,
    Descending,
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct FirestoreQueryFilterComposite {
    pub for_all_filters: Vec<FirestoreQueryFilter>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FirestoreQueryFilterUnary {
    IsNan(String),
    IsNull(String),
    IsNotNan(String),
    IsNotNull(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FirestoreQueryFilterCompare {
    LessThan(String, FirestoreValue),
    LessThanOrEqual(String, FirestoreValue),
    GreaterThan(String, FirestoreValue),
    GreaterThanOrEqual(String, FirestoreValue),
    Equal(String, FirestoreValue),
    NotEqual(String, FirestoreValue),
    ArrayContains(String, FirestoreValue),
    In(String, FirestoreValue),
    ArrayContainsAny(String, FirestoreValue),
    NotIn(String, FirestoreValue),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FirestoreValue {
    pub value: Value,
}
