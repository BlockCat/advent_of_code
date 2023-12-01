use std::collections::HashMap;

use crate::vector::Vector2;

pub trait Grid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    fn set(&mut self, x: isize, y: isize, value: T);
    fn set_vec(&mut self, pos: &Vector2, value: T) {
        self.set(pos[0] as isize, pos[1] as isize, value);
    }
    fn get_vec(&self, pos: &Vector2) -> Option<&T> {
        self.get(pos[0], pos[1])
    }
    fn get(&self, x: isize, y: isize) -> Option<&T>;

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T>;
    fn get_mut_vec(&mut self, pos: &Vector2) -> Option<&mut T>;
    fn get_insert_vec_mut(&mut self, pos: &Vector2, default: T) -> &mut T;
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct DynamicGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub map: HashMap<(isize, isize), T>,
}

impl<T> DynamicGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub fn from_vec(vec: Vec<Vec<T>>) -> Self {
        let mut grid = Self::default();

        for y in 0..vec.len() {
            let rows = &vec[y];
            for x in 0..rows.len() {
                grid.set(x as isize, y as isize, rows[x].clone());
            }
        }

        grid
    }
}

impl<T> Grid<T> for DynamicGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    fn set(&mut self, x: isize, y: isize, value: T) {
        self.map.insert((x, y), value);
    }

    fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.map.get(&(x, y))
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        self.map.get_mut(&(x, y))
    }

    fn get_mut_vec(&mut self, pos: &Vector2) -> Option<&mut T> {
        unimplemented!()
    }

    fn get_insert_vec_mut(&mut self, pos: &Vector2, default: T) -> &mut T {
        self.map
            .entry((pos[0] as isize, pos[1] as isize))
            .or_insert(default)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct StaticGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub grid: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> for StaticGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    fn set(&mut self, x: isize, y: isize, value: T) {
        self.grid[(x + y * self.width as isize) as usize] = value;
    }

    fn set_vec(&mut self, pos: &Vector2, value: T) {
        self.set(pos[0] as isize, pos[1] as isize, value);
    }

    fn get_vec(&self, pos: &Vector2) -> Option<&T> {
        self.get(pos[0] as isize, pos[1] as isize)
    }

    fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x < self.width as isize && y < self.height as isize {
            self.grid.get((x + y * self.width as isize) as usize)
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if (0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y) {
            self.grid.get_mut((x + y * self.width as isize) as usize)
        } else {
            None
        }
    }
    fn get_mut_vec(&mut self, pos: &Vector2) -> Option<&mut T> {
        self.get_mut(pos[0] as isize, pos[1] as isize)
    }

    fn get_insert_vec_mut(&mut self, pos: &Vector2, _: T) -> &mut T {
        self.get_mut(pos[0] as isize, pos[1] as isize).unwrap()
    }
}

impl<T> StaticGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub fn new(width: usize, height: usize) -> StaticGrid<T> {
        StaticGrid {
            grid: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    pub fn from_slice(width: usize, height: usize, slice: Vec<T>) -> StaticGrid<T> {
        StaticGrid {
            width,
            height,
            grid: slice,
        }
    }

    pub fn from_vec(grid: Vec<Vec<T>>) -> StaticGrid<T> {
        let width = grid[0].len();
        let height = grid.len();
        StaticGrid {
            grid: grid.into_iter().flatten().collect(),
            width,
            height,
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.grid.chunks(self.width).map(|x| Vec::from(x)).collect()
    }

    pub fn iter<'a>(&'a self) -> GridIterator<'a, T> {
        GridIterator {
            grid: &self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIterator<'a, T>
where
    T: Clone + PartialEq + Eq + Default,
{
    grid: &'a StaticGrid<T>,
    x: isize,
    y: isize,
}

impl<'a, T> Iterator for GridIterator<'a, T>
where
    T: Clone + PartialEq + Eq + Default,
{
    type Item = (Vector2, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.get(self.x, self.y);

        if let Some(v) = val {
            let x = self.x;
            let y = self.y;

            self.x += 1;
            if self.x >= self.grid.width as isize {
                self.x = 0;
                self.y += 1;
            }

            return Some((Vector2::new([x as isize, y as isize]), v));
        } else {
            return None;
        }
    }
}
