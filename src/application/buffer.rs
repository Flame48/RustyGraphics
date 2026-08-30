pub struct ScreenBuffer2D<P: Copy + PartialEq + Default> {
    pub width: usize,
    pub height: usize,
    pub front: Vec<P>,
    pub back: Vec<P>,
}

impl<P: Copy + PartialEq + Default> ScreenBuffer2D<P> {
    pub fn new(width: usize, height: usize) -> Self {
        let len = width * height;
        Self {
            width,
            height,
            front: vec![P::default(); len],
            back: vec![P::default(); len],
        }
    }

    #[inline]
    pub fn idx(&self, x: usize, y: usize) -> Option<usize> {
        (x < self.width && y < self.height).then(
            || (y as usize) * (self.width as usize) + (x as usize)
        )
    }

    pub fn fill(&mut self, p: P) {
        self.back.fill(p);
    }

    pub fn clear(&mut self) {
        self.fill(P::default());
    }

    pub fn put(&mut self, p: P, x: usize, y: usize) {
        if let Some(i) = self.idx(x, y) {
            self.back[i] = p;
        }
    }

    pub fn fill_bound(&mut self, p: P, x: usize, y: usize, w: usize, h: usize) {
        for row in y..y + h {
            for col in x..x + w {
                self.put(p, col, row);
            }
        }
    }

    pub fn line(&mut self, p: P, x0: i32, y0: i32, x1: i32, y1: i32) {
        let (mut x0, mut y0) = (x0, y0);
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && y0 >= 0 {
                self.put(p, x0 as usize, y0 as usize);
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn triangle(&mut self, p: P, p1: [i32; 2], p2: [i32; 2], p3: [i32; 2]) {
        self.line(p, p1[0], p1[1], p2[0], p2[1]);
        self.line(p, p2[0], p2[1], p3[0], p3[1]);
        self.line(p, p3[0], p3[1], p1[0], p1[1]);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        let len = (width as usize) * (height as usize);
        self.front = vec![P::default(); len];
        self.back = vec![P::default(); len];
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }
}
