use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rectangle {
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dataset {
    pub rectangles: Vec<Rectangle>,
}

pub(crate) fn length(start: i64, end: i64) -> u128 {
    (i128::from(end) - i128::from(start)) as u128
}

impl Rectangle {
    pub fn validate(&self) -> Result<()> {
        if self.x1 > self.x2 || self.y1 > self.y2 {
            return Err("rectangle bounds must satisfy x1 <= x2 and y1 <= y2".into());
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    pub fn area(&self) -> Result<u128> {
        self.validate()?;
        Ok(length(self.x1, self.x2) * length(self.y1, self.y2))
    }
}
