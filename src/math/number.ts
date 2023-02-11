const compare = (
  left: number,
  right: number,
  precision = Number.EPSILON
): 1 | 0 | -1 => {
  const difference = left - right;

  if (difference > precision) return 1;
  if (difference < -precision) return -1;
  return 0;
};

const max = (a: number, b: number, precision = Number.EPSILON) => {
  if (compare(a, b, precision) >= 0) return a;
  return b;
};

const min = (a: number, b: number, precision = Number.EPSILON) => {
  if (compare(a, b, precision) <= 0) return a;
  return b;
};

export { compare, min, max };
