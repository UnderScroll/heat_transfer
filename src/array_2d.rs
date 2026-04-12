#[derive(Debug, Clone)]
pub struct Array2D<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl<T> Array2D<T> {
    #[must_use]
    pub fn new_with_item(item: T, width: usize, height: usize) -> Self
    where
        T: Copy,
    {
        // Init data
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            data.push(item);
        }

        Self { data, width, height }
    }

    #[must_use]
    pub fn new_with(width: usize, height: usize, generator: fn(usize, usize) -> T) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                data.push(generator(x, y));
            }
        }

        Self { data, width, height }
    }

    pub fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        Self { data, width, height }
    }

    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[x + y * self.width]
    }

    #[must_use]
    pub fn get_line(&self, y: usize) -> &[T] {
        let index = y * self.width;
        &self.data[index..index + self.width]
    }

    #[must_use]
    pub fn get_line_mut(&mut self, y: usize) -> &mut [T] {
        let index = y * self.width;
        &mut self.data[index..index + self.width]
    }

    #[must_use]
    pub fn try_get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(self.get(x, y))
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[x + y * self.width]
    }

    #[must_use]
    pub fn try_get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(self.get_mut(x, y))
        } else {
            None
        }
    }
}
