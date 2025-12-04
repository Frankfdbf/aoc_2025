use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn pos_to_index(&self, pos: Coordinates) -> Option<usize> {
        if pos.x >= self.width || pos.y >= self.height {
            None
        } else {
            Some(pos.y * self.width + pos.x)
        }
    }

    fn index_to_pos(&self, idx: usize) -> Option<Coordinates> {
        if idx >= self.data.len() {
            None
        } else {
            let x = idx % self.width;
            let y = idx / self.width;
            Some(Coordinates::new(x, y))
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn neighbors(&self, pos: Coordinates) -> impl Iterator<Item = Coordinates> {
        let range_width = pos.x.saturating_sub(1)..self.width.min(pos.x + 2);
        let range_height = pos.y.saturating_sub(1)..self.height.min(pos.y + 2);

        range_width
            .flat_map(move |x| range_height.clone().map(move |y| Coordinates::new(x, y)))
            .filter(move |neighbor| neighbor != &pos)
    }
}

impl<T> Index<Coordinates> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Coordinates) -> &Self::Output {
        let idx = self.pos_to_index(pos).expect("index out of bound");
        &self.data[idx]
    }
}

impl<T> IndexMut<Coordinates> for Grid<T> {
    fn index_mut(&mut self, pos: Coordinates) -> &mut Self::Output {
        let idx = self.pos_to_index(pos).expect("index out of bound");
        &mut self.data[idx]
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<u8>,
{
    type Err = T::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines();

        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().len();

        let data = lines
            .flat_map(|line| line.bytes().map(|b| b.try_into()))
            .collect::<Result<Vec<T>, _>>()?;

        Ok(Self {
            data,
            width,
            height,
        })
    }
}
