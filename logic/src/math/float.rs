use std::cmp::Ordering;

pub const DEFAULT_PRECISION: f32 = f32::EPSILON;

pub fn compare_with_precision(left: f32, right: f32, precision: f32) -> Ordering {
    let difference = left - right;

    if difference > precision {
        Ordering::Greater
    } else if difference < -precision {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

pub fn compare(left: f32, right: f32) -> Ordering {
    compare_with_precision(left, right, DEFAULT_PRECISION)
}

pub fn order_with_precision(tuple: (f32, f32), precision: f32) -> (f32, f32) {
    if compare_with_precision(tuple.0, tuple.1, precision).is_gt() {
        (tuple.1, tuple.0)
    } else {
        tuple
    }
}

pub fn order(tuple: (f32, f32)) -> (f32, f32) {
    order_with_precision(tuple, DEFAULT_PRECISION)
}
