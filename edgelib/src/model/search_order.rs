pub struct SearchOrder {
    pub width: usize,
    pub height: usize,
    pub size: usize,
    pub order: Vec<Location>,
}

#[derive(Clone)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub enum SearchOption {
    ScanRows,
    ScanColumns,
    ScanLargest,
    BuildSquare,
    SpiralIn,
    SpiralInSquare,
    SpiralOut,
    SpiralOutSquare,
    FrameFirstRows,
    FrameFirstColumns,
    FrameLastRows,
    FrameLastColumns,
}

impl SearchOrder {
    pub fn new(width: usize, height: usize, search_option: SearchOption) -> Self {
        if width <= 1 || height <= 1 {
            panic!("Width and height must be greater than 1");
        }

        let size = width * height;

        let order: Vec<Location> = match search_option {
            SearchOption::ScanColumns => frame_columns(width, height, true, false),
            SearchOption::ScanRows => frame_rows(width, height, true, false),
            SearchOption::ScanLargest => build_square(width, height, size, false),
            SearchOption::BuildSquare => build_square(width, height, size, true),
            SearchOption::SpiralIn => spiral(width, height, size, false, false),
            SearchOption::SpiralInSquare => spiral(width, height, size, true, false),
            SearchOption::SpiralOut => spiral(width, height, size, false, true),
            SearchOption::SpiralOutSquare => spiral(width, height, size, true, true),
            SearchOption::FrameFirstColumns => frame_columns(width, height, false, false),
            SearchOption::FrameFirstRows => frame_rows(width, height, false, false),
            SearchOption::FrameLastColumns => frame_columns(width, height, false, true),
            SearchOption::FrameLastRows => frame_rows(width, height, false, true),
        };

        SearchOrder {
            width,
            height,
            size,
            order,
        }
    }
}

fn frame_rows(width: usize, height: usize, ignore: bool, reverse: bool) -> Vec<Location> {
    let mut order: Vec<Location> = vec![];
    let mut order_inner: Vec<Location> = vec![];
    (0..height).for_each(|y| {
        let other_edge = y == 0 || y == height - 1;
        (0..width).for_each(|x| {
            let side_edge = x == 0 || x == width - 1;
            let location = Location { x, y };
            if side_edge || other_edge || ignore {
                order.push(location);
            } else {
                order_inner.push(location);
            }
        })
    });
    if reverse {
        order_inner.extend(order);
        return order_inner;
    }
    order.extend(order_inner);
    order
}

fn frame_columns(width: usize, height: usize, ignore: bool, reverse: bool) -> Vec<Location> {
    let mut order: Vec<Location> = vec![];
    let mut order_inner: Vec<Location> = vec![];
    (0..width).for_each(|x| {
        let side_edge = x == 0 || x == width - 1;
        (0..height).for_each(|y| {
            let other_edge = y == 0 || y == height - 1;
            let location = Location { x, y };
            if side_edge || other_edge || ignore {
                order.push(location);
            } else {
                order_inner.push(location);
            }
        })
    });
    if reverse {
        order_inner.extend(order);
        return order_inner;
    }
    order.extend(order_inner);
    order
}

fn build_square(width: usize, height: usize, size: usize, reverse: bool) -> Vec<Location> {
    let mut start_x = 0;
    let mut start_y = 0;

    let mut order: Vec<Location> = vec![];
    while order.len() < size && start_x < width && start_y < height {
        let left_width = width - start_x;
        let left_height = height - start_y;
        if left_width >= left_height {
            (start_y..height).for_each(|y| {
                order.push(Location { x: start_x, y });
            });
            start_x += 1;
        } else {
            (start_x..width).for_each(|x| {
                order.push(Location { x, y: start_y });
            });
            start_y += 1;
        }
    }
    if reverse {
        order.reverse();
        order.iter_mut().for_each(|location| {
            location.x = width - 1 - location.x;
            location.y = height - 1 - location.y;
        });
    }
    order
}

fn spiral(
    width: usize,
    height: usize,
    size: usize,
    keep_square: bool,
    reverse: bool,
) -> Vec<Location> {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = width;
    let mut end_y = height;

    let mut order: Vec<Location> = vec![];
    let mut direction: usize = 0;
    while order.len() < size && start_x < end_x && start_y < end_y {
        let left_width = end_x - start_x;
        let left_height = end_y - start_y;

        match direction {
            0 => {
                if !keep_square || left_height <= left_height {
                    (start_x..end_x).for_each(|x| {
                        order.push(Location { x, y: start_y });
                    });
                    start_y += 1;
                }
            }
            1 => {
                if !keep_square || left_width <= left_width {
                    (start_y..end_y).for_each(|y| {
                        order.push(Location { x: end_x - 1, y: y });
                    });
                    end_x -= 1;
                }
            }
            2 => {
                if !keep_square || left_height <= left_height {
                    (start_x..end_x).rev().for_each(|x| {
                        order.push(Location { x, y: end_y - 1 });
                    });
                    end_y -= 1;
                }
            }
            _ => {
                if !keep_square || left_width <= left_width {
                    (start_y..end_y).rev().for_each(|y| {
                        order.push(Location { x: start_x, y: y });
                    });
                    start_x += 1;
                }
            }
        }
        direction = (direction + 1) % 4;
    }
    if reverse {
        order.reverse();
    }
    order
}
