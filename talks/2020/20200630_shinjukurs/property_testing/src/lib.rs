use quickcheck::{Arbitrary, Gen};

pub const VOLUME_WEIGHT_RATE: f32 = 280.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Baggage {
    pub height: f32,
    pub width: f32,
    pub depth: f32,
    pub weight: f32,
    pub has_restriction: bool,
    pub description: String,
}
impl Baggage {
    pub fn is_valid(&self) -> bool {
        if self.height <= 0.0 || self.width <= 0.0 || self.depth <= 0.0 || self.weight <= 0.0 {
            return false;
        }
        if self.weight > 30.0 {
            return false;
        }
        if self.height + self.width + self.depth > 2.0 {
            return false;
        }
        if self.height > 1.7 || self.width > 1.7 || self.depth > 1.7 {
            return false;
        }
        if self.has_restriction {
            if self.height > 1.0 || self.width > 1.0 || self.depth > 1.0 {
                return false;
            }
        }
        true
    }
}
impl Default for Baggage {
    fn default() -> Self {
        Baggage {
            height: 0.3,
            width: 0.3,
            depth: 0.3,
            weight: 10.0,
            has_restriction: false,
            description: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_valid_default() {
        let b = Baggage {
            ..Baggage::default()
        };
        assert!(b.is_valid());
    }
    #[test]
    fn is_valid_epsilon() {
        let b = Baggage {
            height: f32::EPSILON,
            width: f32::EPSILON,
            depth: f32::EPSILON,
            ..Baggage::default()
        };
        assert!(b.is_valid());
    }
}

#[derive(Clone, Debug)]
pub struct Query {
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub description_contains: Option<String>,
}
impl Query {
    pub fn is_satisfied(&self, b: &Baggage) -> bool {
        if let Some(min_height) = self.min_height {
            if b.height < min_height {
                return false;
            }
        }
        if let Some(max_height) = self.max_height {
            if max_height < b.height {
                return false;
            }
        }
        if let Some(description_contains) = &self.description_contains {
            if !b.description.contains(description_contains) {
                return false;
            }
        }
        true
    }
}
impl Default for Query {
    fn default() -> Self {
        Query {
            min_height: None,
            max_height: None,
            description_contains: None,
        }
    }
}

pub fn search(bs: &Vec<Baggage>, q: &Query) -> Vec<Baggage> {
    let mut result = vec![];
    for b in bs {
        if q.is_satisfied(b) {
            result.push(b.clone());
        }

    result
}

impl Arbitrary for Baggage {
    fn arbitrary<G: Gen>(g: &mut G) -> Baggage {
        Baggage {
            height: f32::arbitrary(g),
            width: f32::arbitrary(g),
            depth: f32::arbitrary(g),
            weight: f32::arbitrary(g),
            has_restriction: bool::arbitrary(g),
            description: String::arbitrary(g),
        }
    }
}
impl Arbitrary for Query {
    fn arbitrary<G: Gen>(g: &mut G) -> Query {
        Query {
            min_height: Option::<f32>::arbitrary(g),
            max_height: Option::<f32>::arbitrary(g),
            description_contains: Option::<String>::arbitrary(g),
        }
    }
}
