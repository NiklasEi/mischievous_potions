use crate::loading::TextureAssets;
use bevy::prelude::*;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Line { slots: Vec<Slot> },
}

#[derive(Clone, Debug)]
pub struct SlotContent {
    pub entity: Entity,
    pub animal: Animal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slot {
    pub row: usize,
    pub column: usize,
}

impl Slot {
    pub fn new(row: usize, column: usize) -> Self {
        Slot { row, column }
    }

    pub fn walk(&self, row_delta: i64, column_delta: i64) -> Slot {
        Slot {
            row: usize::try_from(self.row as i64 + row_delta)
                .expect("Overflow navigating the board"),
            column: usize::try_from(self.column as i64 + column_delta)
                .expect("Overflow navigating the board"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Animal {
    Eye,
    BirdTwo,
}

impl Animal {
    pub fn get_texture(&self, assets: &TextureAssets) -> Handle<Texture> {
        match self {
            &Animal::Eye => assets.eye.clone(),
            _ => assets.eye.clone(),
        }
    }
}
